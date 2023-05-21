use std::{
  io::stdout,
  sync::Arc,
  time::Duration,
};
use crossterm::{
  event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
  backend::CrosstermBackend,
  layout::{Rect,Constraint, Direction, Layout},
  widgets::Block,
  Terminal,
};

use crate::fetch::fetch_stats::StatsResponse;
use crate::fetch::fetch_query_log::Query;
use crate::fetch::fetch_status::StatusResponse;

use crate::widgets::gauge::make_gauge;
use crate::widgets::table::make_query_table;
use crate::widgets::chart::{make_history_chart, prepare_chart_data};
use crate::widgets::status::render_status_paragraph;

pub async fn draw_ui(
    mut data_rx: tokio::sync::mpsc::Receiver<Vec<Query>>,
    mut stats_rx: tokio::sync::mpsc::Receiver<StatsResponse>,
    mut status_rx: tokio::sync::mpsc::Receiver<StatusResponse>,
    shutdown: Arc<tokio::sync::Notify>
) -> Result<(), anyhow::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        // Recieve query log and stats data from the fetcher
        let data = match data_rx.recv().await {
            Some(data) => data,
            None => break, // Channel has been closed, so we break the loop
        };
        let mut stats = match stats_rx.recv().await {
            Some(stats) => stats,
            None => break,
        };
        let mut status = match status_rx.recv().await {
            Some(status) => status,
            None => break,
        };

        // Prepare the data for the chart
        prepare_chart_data(&mut stats);

        terminal.draw(|f| {
            let size = f.size();

            // Make the charts
            let gauge = make_gauge(&stats);
            let table = make_query_table(&data);
            let graph = make_history_chart(&stats);
            let paragraph = render_status_paragraph(&status);


            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(30), // The top row (gauge + status, and the history chart)
                    Constraint::Min(1), // The query log table
                ]
                .as_ref(),
            )
            .split(size);

            // Split the top part (charts + gauge) into left (gauge + block) and right (line chart)
            let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(30), 
                    Constraint::Percentage(70), 
                ]
                .as_ref(),
            )
            .split(chunks[0]);

            // Split the left part of top (gauge + block) into top (gauge) and bottom (block)
            let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(50), 
                    Constraint::Percentage(50), 
                ]
                .as_ref(),
            )
            .split(top_chunks[0]);

            // Render your widgets here
            f.render_widget(gauge, left_chunks[0]);
            f.render_widget(paragraph, left_chunks[1]);
            f.render_widget(graph, top_chunks[1]);
            f.render_widget(table, chunks[1]);
            
        })?;

        // Check for user input events
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    // std::process::exit(0);
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

