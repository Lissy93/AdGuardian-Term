
// Import modules, and types
use std::io::{stdout};
use std::time::Duration;
use std::sync::Arc;

use serde::Deserialize;
use reqwest::Client;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, AUTHORIZATION};

use crossterm::event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use chrono::{DateTime, Utc, TimeZone};

use tokio::time::interval;
use anyhow::Error;

use tui::{
    backend::CrosstermBackend,
    layout::{Rect, Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};

use futures::future::FutureExt;

#[derive(Deserialize)]
struct QueryResponse {
    data: Vec<Query>,
}

#[derive(Deserialize)]
struct Query {
    answer: Option<Vec<Answer>>,
    answer_dnssec: bool,
    cached: bool,
    client: String,
    client_info: ClientInfo,
    client_proto: String,
    #[serde(rename = "elapsedMs")]
    elapsed_ms: String,
    question: Question,
    reason: String,
    // rules: Vec<String>,
    status: String,
    time: String,
    upstream: String,
}

#[derive(Deserialize)]
struct Answer {
    #[serde(rename = "type")]
    answer_type: String,
    value: String,
    ttl: u32,
}

#[derive(Deserialize)]
struct ClientInfo {
    whois: serde_json::Value,
    name: String,
    disallowed_rule: String,
    disallowed: bool,
}

#[derive(Deserialize)]
struct Question {
    class: String,
    name: String,
    #[serde(rename = "type")]
    question_type: String,
}

async fn fetch_adguard_data(
    client: &reqwest::Client,
    endpoint: &str,
    username: &str,
    password: &str,
) -> Result<QueryResponse, anyhow::Error> {
    let auth_string = format!("{}:{}", username, password);
    let auth_header_value = format!("Basic {}", base64::encode(&auth_string));
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(AUTHORIZATION, auth_header_value.parse()?);
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

    let url = format!("{}/control/querylog", endpoint);
    let response = client.get(&url).headers(headers).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Request failed with status code {}", response.status()));
    }

    let data = response.json().await?;
    Ok(data)
}


fn time_ago(timestamp: &str) -> Result<String, anyhow::Error> {
    let datetime = DateTime::parse_from_rfc3339(timestamp)?;
    let datetime_utc = datetime.with_timezone(&Utc);
    let now = Utc::now();

    let duration = now - datetime_utc;

    if duration.num_minutes() < 1 {
        Ok(format!("{} sec ago", duration.num_seconds()))
    } else {
        Ok(format!("{} min ago", duration.num_minutes()))
    }
}

fn make_request_cell(q: &Question) -> Result<String, anyhow::Error> {
    Ok(format!("[{}] {} - {}", q.class, q.question_type, q.name))
}

fn make_time_taken_and_color(elapsed: &str) -> Result<(String, Color), anyhow::Error> {
    let elapsed_f64 = elapsed.parse::<f64>()?;
    let rounded_elapsed = (elapsed_f64 * 100.0).round() / 100.0;
    let time_taken = format!("{:.2} ms", rounded_elapsed);
    let color = if elapsed_f64 < 1.0 {
        Color::Green
    } else if elapsed_f64 >= 1.0 && elapsed_f64 <= 20.0 {
        Color::Yellow
    } else {
        Color::Red
    };
    Ok((time_taken, color))
}

fn make_time_taken(elapsed: &str) -> Result<String, anyhow::Error> {
    let elapsed_f64 = elapsed.parse::<f64>()?;
    let rounded_elapsed = (elapsed_f64 * 100.0).round() / 100.0;
    Ok(format!("{:.2} ms", rounded_elapsed))
}

fn elapsed_time_color(elapsed: f64) -> Color {
    if elapsed < 1.0 {
        Color::Green
    } else if elapsed >= 1.0 && elapsed <= 20.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn make_row_color(reason: &str) -> Color {
    return if reason == "NotFilteredNotFound" {
        Color::Green
    } else if reason == "FilteredBlackList" {
        Color::Red
    } else {
        Color::Yellow
    }
}

fn block_status_text(reason: &str, cached: bool) -> (String, Color) {
    let (text, color) =
    if reason == "FilteredBlackList" {
        ("Blacklisted".to_string(), Color::Red)
    } else if cached == true {
        ("Cached".to_string(), Color::Cyan)
    } else if reason == "NotFilteredNotFound" {
        ("Allowed".to_string(), Color::Green)
    } else {
        ("Other Block".to_string(), Color::Yellow)
    };
    (text, color)
}


async fn draw_ui(
    mut data_rx: tokio::sync::mpsc::Receiver<Vec<Query>>, 
    shutdown: Arc<tokio::sync::Notify>
) -> Result<(), anyhow::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;


    loop {
        let data = match data_rx.recv().await {
            Some(data) => data,
            None => break, // Channel has been closed, so we break the loop
        };
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("AdGuard Dashboard")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White));
            f.render_widget(block, size);

            let rows = data.iter().map(|query| {
                
                let time = Cell::from(
                    time_ago(query.time.as_str()).unwrap_or("unknown".to_string())
                ).style(Style::default().fg(Color::Gray));
                
                let question = Cell::from(make_request_cell(&query.question).unwrap())
                    .style(Style::default().add_modifier(Modifier::BOLD));
                
                let client = Cell::from(query.client.as_str())
                    .style(Style::default().fg(Color::Blue));
                
                let (time_taken, elapsed_color) = make_time_taken_and_color(&query.elapsed_ms).unwrap();
                let elapsed_ms = Cell::from(time_taken).style(Style::default().fg(elapsed_color));
                                    
                let (status_txt, status_color) = block_status_text(&query.reason, query.cached);
                let status = Cell::from(status_txt).style(Style::default().fg(status_color));
                    
                let color = make_row_color(&query.reason);
                Row::new(vec![time, question, status, client, elapsed_ms]).style(Style::default().fg(color))
            });

            let table = Table::new(rows)
                .header(Row::new(vec![
                    Cell::from("Time"),
                    Cell::from("Request"),
                    Cell::from("Status"),
                    Cell::from("Client"),
                    Cell::from("Time Taken"),
                ]))
                .block(Block::default().title("Query Log").borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(15),
                    Constraint::Percentage(35),
                    Constraint::Percentage(15),
                    Constraint::Percentage(20),
                    Constraint::Percentage(15),
                ]);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(size);

            f.render_widget(table, chunks[1]);
        })?;

        // Check for user input events
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    shutdown.notify_waiters();
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('Q'),
                    ..
                }) => {
                    shutdown.notify_waiters();
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => {
                    shutdown.notify_waiters();
                    break;
                }
                Event::Resize(_, _) => {}, // Handle resize event, loop will redraw the UI
                _ => {}
            }
        }

    }

    terminal.show_cursor()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;
    Ok(())
}


async fn run() -> Result<(), anyhow::Error> {

    let shutdown = Arc::new(tokio::sync::Notify::new());

    let (data_tx, data_rx) = tokio::sync::mpsc::channel(1);

    let draw_ui_task = tokio::spawn(draw_ui(data_rx, Arc::clone(&shutdown)));

    let client = Client::new();
    let hostname = "http://192.168.130.2:8083";
    let username = "admin";
    let password = "uPbxy1G8g0xO83nw";
    let mut interval = interval(Duration::from_secs(5));

    loop {
        let data = fetch_adguard_data(&client, hostname, username, password).await?;
        data_tx.send(data.data).await;
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

