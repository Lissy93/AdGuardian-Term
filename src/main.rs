
mod fetch;
mod ui;
mod widgets;
mod welcome;

use fetch::fetch_query_log::fetch_adguard_query_log;
use fetch::fetch_stats::fetch_adguard_stats;
use fetch::fetch_status::fetch_adguard_status;

use ui::{ draw_ui };
use std::{sync::Arc, time::Duration};
use reqwest::{Client};
use tokio::time::interval;

fn get_update_interval() -> u64 {
    match env::var("ADGUARDIAN_UPDATE_INTERVAL") {
        Ok(val) => {
            match val.parse::<u64>() {
                Ok(val) => val,
                Err(_) => 3
            }
        },
        Err(_) => 3
    }
}

async fn run() -> Result<(), anyhow::Error> {

    let shutdown = Arc::new(tokio::sync::Notify::new());

    // Channels for sending data from the fetcher to the UI
    let (queries_tx, queries_rx) = tokio::sync::mpsc::channel(1);
    let (stats_tx, stats_rx) = tokio::sync::mpsc::channel(1);
    let (status_tx, status_rx) = tokio::sync::mpsc::channel(1);

    let draw_ui_task = tokio::spawn(draw_ui(queries_rx, stats_rx, status_rx, Arc::clone(&shutdown)));

    let client = Client::new();

    let ip = env::var("ADGUARD_IP").unwrap();
    let port = env::var("ADGUARD_PORT").unwrap();
    let username = env::var("ADGUARD_USERNAME").unwrap();
    let password = env::var("ADGUARD_PASSWORD").unwrap();
    let hostname = format!("http://{}:{}", ip, port);

    let mut interval = interval(Duration::from_secs(3));
    
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let queries = fetch_adguard_query_log(&client, hostname, username, password).await?;
                if queries_tx.try_send(queries.data).is_err() {
                    break;
                }
                
                let stats = fetch_adguard_stats(&client, hostname, username, password).await?;
                if stats_tx.try_send(stats).is_err() {
                    break;
                }

                let status = fetch_adguard_status(&client, hostname, username, password).await?;
                if status_tx.try_send(status).is_err() {
                    break;
                }
            }
            _ = shutdown.notified() => {
                break;
            }
        }
    }
    
    draw_ui_task.await??;
    Ok(())
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = welcome::welcome().await {
            eprintln!("Failed to initialize: {}", e);
            std::process::exit(1);
        }

        if let Err(e) = run().await {
            eprintln!("Failed to run: {}", e);
            std::process::exit(1);
        }
    });
}



// fn main() -> Result<(), anyhow::Error> {
//     let rt = tokio::runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()?;

//     rt.block_on(run())
// }

