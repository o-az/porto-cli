use crate::error::{PortoError, Result};
use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::{sse::{Event, Sse}, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use futures_util::stream::Stream;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayMessage {
    pub id: String,
    pub topic: String,
    pub payload: serde_json::Value,
}

#[derive(Clone)]
struct RelayState {
    sender: broadcast::Sender<RelayMessage>,
    keys: Arc<RwLock<HashSet<String>>>,
    pending_responses: Arc<RwLock<HashMap<u64, mpsc::Sender<serde_json::Value>>>>,
}

pub struct RelayServer {
    url: String,
    state: RelayState,
    _handle: tokio::task::JoinHandle<()>,
}

impl RelayServer {
    pub async fn new() -> Result<Self> {
        let (tx, _) = broadcast::channel::<RelayMessage>(100);
        let keys = Arc::new(RwLock::new(HashSet::new()));
        let pending_responses = Arc::new(RwLock::new(HashMap::new()));
        
        let state = RelayState {
            sender: tx.clone(),
            keys: keys.clone(),
            pending_responses: pending_responses.clone(),
        };
        
        // Find available port
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let listener = tokio::net::TcpListener::bind(addr).await
            .map_err(|e| PortoError::DialogError(format!("Failed to bind listener: {}", e)))?;
        let port = listener.local_addr()
            .map_err(|e| PortoError::DialogError(format!("Failed to get local address: {}", e)))?
            .port();
        
        let url = format!("http://localhost:{}", port);
        
        // Set up routes
        let app = Router::new()
            .route("/", get(sse_handler).post(post_handler))
            .route("/.well-known/keys", get(keys_handler))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                    .allow_headers(Any)
                    .allow_private_network(true)
            )
            .with_state(state.clone());
        
        // Start server
        let handle = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });
        
        Ok(Self {
            url,
            state,
            _handle: handle,
        })
    }
    
    pub fn url(&self) -> &str {
        &self.url
    }
    
    pub async fn register_public_key(&self, public_key: String) -> Result<()> {
        let mut keys = self.state.keys.write().await;
        keys.insert(public_key);
        Ok(())
    }
    
    pub async fn send_message(&self, topic: &str, payload: serde_json::Value) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let message = RelayMessage {
            id: id.clone(),
            topic: topic.to_string(),
            payload,
        };
        
        self.state.sender.send(message)
            .map_err(|e| PortoError::DialogError(format!("Failed to send message: {}", e)))?;
        
        Ok(id)
    }
    
    pub async fn wait_for_response(&self, request_id: u64) -> Result<serde_json::Value> {
        let (tx, mut rx) = mpsc::channel(1);
        
        {
            let mut pending = self.state.pending_responses.write().await;
            pending.insert(request_id, tx);
        }
        
        // Listen for broadcasts and check for our response
        let mut receiver = self.state.sender.subscribe();
        
        tokio::select! {
            // Wait for response via channel
            result = rx.recv() => {
                match result {
                    Some(response) => Ok(response),
                    None => Err(PortoError::DialogError("Channel closed".to_string())),
                }
            }
            // Also listen to broadcast messages
            _ = async {
                while let Ok(msg) = receiver.recv().await {
                    if msg.topic == "rpc-response" {
                        if let Some(id) = msg.payload.get("id").and_then(|v| v.as_u64()) {
                            if id == request_id {
                                if let Some(result) = msg.payload.get("result") {
                                    let mut pending = self.state.pending_responses.write().await;
                                    if let Some(sender) = pending.remove(&id) {
                                        let _ = sender.send(result.clone()).await;
                                    }
                                }
                            }
                        }
                    }
                }
            } => {
                Err(PortoError::DialogError("Broadcast channel closed".to_string()))
            }
            // Timeout after 5 minutes
            _ = tokio::time::sleep(Duration::from_secs(300)) => {
                Err(PortoError::DialogError("Request timed out".to_string()))
            }
        }
    }
}

async fn sse_handler(
    State(state): State<RelayState>,
) -> Sse<impl Stream<Item = std::result::Result<Event, Infallible>>> {
    let receiver = state.sender.subscribe();
    let stream = BroadcastStream::new(receiver);
    
    let sse_stream = stream
        .filter_map(|result| {
            match result {
                Ok(msg) => {
                    if let Ok(json) = serde_json::to_string(&msg) {
                        Some(Ok(Event::default().data(json)))
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        });
    
    Sse::new(sse_stream)
}

async fn post_handler(
    State(state): State<RelayState>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    if let (Some(topic), Some(payload)) = (
        body.get("topic").and_then(|t| t.as_str()),
        body.get("payload"),
    ) {
        // Handle rpc-response messages
        if topic == "rpc-response" {
            if let Some(id) = payload.get("id").and_then(|v| v.as_u64()) {
                if let Some(result) = payload.get("result") {
                    let mut pending = state.pending_responses.write().await;
                    if let Some(sender) = pending.remove(&id) {
                        let _ = sender.send(result.clone()).await;
                    }
                }
            }
        }
        
        // Broadcast the message
        let id = body.get("id")
            .and_then(|i| i.as_str())
            .unwrap_or(&Uuid::new_v4().to_string())
            .to_string();
            
        let message = RelayMessage {
            id,
            topic: topic.to_string(),
            payload: payload.clone(),
        };
        
        let _ = state.sender.send(message);
        
        StatusCode::OK.into_response()
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
}

async fn keys_handler(
    State(state): State<RelayState>,
) -> Json<serde_json::Value> {
    let keys = state.keys.read().await;
    Json(serde_json::json!({
        "keys": keys.iter().collect::<Vec<_>>()
    }))
}