
// Import modules, and types

use std::error::Error;
use std::io::{stdout};
use std::time::Duration;

use serde::Deserialize;
use reqwest::Client;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, AUTHORIZATION};

use crossterm::event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use tui::{
    backend::CrosstermBackend,
    layout::{Rect, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};

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
) -> Result<QueryResponse, Box<dyn Error>> {
    let auth_string = format!("{}:{}", username, password);
    let auth_header_value = format!("Basic {}", base64::encode(&auth_string));
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(AUTHORIZATION, auth_header_value.parse()?);
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

    let url = format!("{}/control/querylog", endpoint);
    let response = client.get(&url).headers(headers).send().await?;
    if !response.status().is_success() {
        return Err(format!("Request failed with status code {}", response.status()).into());
    }

    // let response_text = response.text().await?;
    // println!("Response JSON: {}", response_text);
    // let data: QueryResponse = serde_json::from_str(&response_text)?;
    // Ok(data)
    
    let data = response.json().await?;
    Ok(data)
}

async fn draw_ui(data: Vec<Query>) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("AdGuard Dashboard")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White));
            f.render_widget(block, size);

            let rows = data.iter().map(|query| {
                let client = Cell::from(query.client.as_str());
                let question_name = Cell::from(query.question.name.as_str());
                let answer_value = query
                    .answer
                    .as_ref()
                    .and_then(|answers| answers.get(0))
                    .map_or_else(|| Cell::from(""), |answer| Cell::from(answer.value.as_str()));
                let status = Cell::from(query.status.as_str());
                Row::new(vec![client, question_name, answer_value, status])
            });

            let table = Table::new(rows)
                .header(Row::new(vec![
                    Cell::from("Client"),
                    Cell::from("Question"),
                    Cell::from("Answer"),
                    Cell::from("Status"),
                ]))
                .block(Block::default().title("Query Log").borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
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
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('Q'),
                    ..
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => break,
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

async fn run() -> Result<(), Box<dyn std::error::Error>> {

    let client = Client::new();
    let hostname = "http://192.168.130.2:8083";
    let username = "admin";
    let password = "";
    let data = fetch_adguard_data(&client, hostname, username, password).await?;
    draw_ui(data.data).await
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(run())
}

