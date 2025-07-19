use crate::error::{PortoError, Result};
use crate::utils::{AdminKey, AdminKeyGrant, ConnectParams, DialogBuilder, DialogRequest, RelayServer, Spinner};
use console::style;
use dialoguer::Confirm;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;


pub async fn execute(admin_key: bool, dialog: String) -> Result<()> {
    println!("{}", style("Create a Porto Account").bold());
    println!();

    // Create relay server for browser communication
    let relay_server = RelayServer::new().await?;

    // Create admin key if requested
    let admin_key = if admin_key {
        let key = AdminKey::new()?;
        // Register public key with relay for verification
        relay_server.register_public_key(key.public_key.clone()).await?;
        Some(key)
    } else {
        None
    };

    // Build dialog URL for wallet connection
    let mut dialog_builder = DialogBuilder::new(dialog);
    dialog_builder.set_relay_url(relay_server.url().to_string());

    // Create account
    let spinner = Spinner::new("Creating account (check browser window)...");
    
    let connect_params = ConnectParams {
        create_account: true,
        grant_admins: admin_key.as_ref().map(|key| {
            vec![AdminKeyGrant {
                public_key: key.public_key.clone(),
                key_type: key.key_type.clone(),
            }]
        }),
    };

    let connect_request = DialogRequest {
        method: "wallet_connect".to_string(),
        params: serde_json::json!([{
            "capabilities": connect_params
        }]),
        id: 1,
    };

    let url = dialog_builder.build_url(&connect_request)?;
    dialog_builder.open_dialog(&url).await?;

    // Wait for actual response from browser
    let response = relay_server.wait_for_response(connect_request.id).await?;
    
    // Parse the accounts from response
    let accounts: Vec<String> = response.get("accounts")
        .and_then(|a| a.as_array())
        .and_then(|arr| {
            arr.iter()
                .filter_map(|v| v.get("address").and_then(|a| a.as_str()))
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .into()
        })
        .ok_or_else(|| PortoError::AccountCreationError("No accounts returned".to_string()))?;
    
    if accounts.is_empty() {
        return Err(PortoError::AccountCreationError("No accounts created".to_string()));
    }
    
    let account_address = accounts[0].clone();
    spinner.stop_with_message("Account created.");

    // Onramp
    let spinner = Spinner::new("Onramping (check browser window)...");
    
    let add_funds_request = DialogRequest {
        method: "wallet_addFunds".to_string(),
        params: serde_json::json!([{
            "address": account_address
        }]),
        id: 2,
    };

    let url = dialog_builder.build_url(&add_funds_request)?;
    dialog_builder.open_dialog(&url).await?;

    // Wait for onramp completion
    relay_server.wait_for_response(add_funds_request.id).await?;
    spinner.stop_with_message("Onramped.");

    // Send success message to dialog
    let _ = relay_server.send_message(
        "success",
        serde_json::json!({
            "content": "You have successfully created an account.",
            "title": "Account created"
        })
    ).await;
    
    println!();
    println!("{}", style("✓ You have successfully created an account.").green());

    // Initialize account if admin key was created
    if let Some(ref key) = admin_key {
        // For now, skip the initialization step to isolate the issue
        // TODO: Implement proper headless wallet_prepareCalls/sendPreparedCalls

        // Handle private key
        println!();
        let reveal = Confirm::new()
            .with_prompt("Reveal private key? (This will be visible in terminal)")
            .default(false)
            .interact()?;

        println!();
        println!("{}: {}", style("Address").bold(), account_address);
        
        if reveal {
            println!("{}: {}", style("Private key").bold(), key.private_key);
        } else {
            let key_file = PathBuf::from(format!("{}.key", account_address));
            fs::write(&key_file, &key.private_key)?;
            
            // Set file permissions to 0600 (read/write for owner only)
            let mut perms = fs::metadata(&key_file)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&key_file, perms)?;
            
            println!(
                "{}: {}",
                style("Private key saved securely to").bold(),
                key_file.display()
            );
        }
    }

    println!();
    println!(
        "{}: {}",
        style("Manage your account at").bold(),
        style("https://id.porto.sh").blue().underlined()
    );

    // Wait a moment before exiting
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(())
}