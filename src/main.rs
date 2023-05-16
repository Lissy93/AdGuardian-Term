
mod fetch;
mod fetch_stats;
mod ui;

use fetch::{ fetch_adguard_data };
use fetch_stats::{ fetch_adguard_stats, StatsResponse };
use ui::{ draw_ui };
use std::{sync::Arc, time::Duration};
use reqwest::{Client};
use futures::future::FutureExt;
use tokio::time::interval;

async fn run() -> Result<(), anyhow::Error> {

    let shutdown = Arc::new(tokio::sync::Notify::new());

    // Channels for sending data from the fetcher to the UI
    let (data_tx, data_rx) = tokio::sync::mpsc::channel(1);
    let (stats_tx, stats_rx) = tokio::sync::mpsc::channel(1);

    let draw_ui_task = tokio::spawn(draw_ui(data_rx, stats_rx, Arc::clone(&shutdown)));

    let client = Client::new();
    let hostname = "http://192.168.130.2:8083";
    let username = "admin";
    let password = "uPbxy1G8g0xO83nw";
    let mut interval = interval(Duration::from_secs(5));

    loop {
        let data = fetch_adguard_data(&client, hostname, username, password).await?;
        data_tx.send(data.data).await;

        let stats = fetch_adguard_stats(&client, hostname, username, password).await?;
        stats_tx.send(stats).await;

        interval.tick().await;
        if shutdown.notified().now_or_never().is_some() {
            break;
        }
    }

    draw_ui_task.await??;
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(run())
}

