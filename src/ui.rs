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
  layout::{Constraint, Direction, Layout},
  style::Color,
  Terminal,
};

use crate::fetch::fetch_stats::StatsResponse;
use crate::fetch::fetch_query_log::Query;
use crate::fetch::fetch_status::StatusResponse;
use crate::fetch::fetch_filters::AdGuardFilteringStatus;

use crate::widgets::gauge::make_gauge;
use crate::widgets::table::make_query_table;
use crate::widgets::chart::{make_history_chart, prepare_chart_data};
use crate::widgets::status::render_status_paragraph;
use crate::widgets::filters::make_filters_list;
use crate::widgets::list::make_list;

pub async fn draw_ui(
    mut data_rx: tokio::sync::mpsc::Receiver<Vec<Query>>,
    mut stats_rx: tokio::sync::mpsc::Receiver<StatsResponse>,
    mut status_rx: tokio::sync::mpsc::Receiver<StatusResponse>,
    filters: AdGuardFilteringStatus,
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
        let status = match status_rx.recv().await {
            Some(status) => status,
            None => break,
        };

        // Prepare the data for the chart
        prepare_chart_data(&mut stats);

        terminal.draw(|f| {
            let size = f.size();

            // Make the charts
            let gauge = make_gauge(&stats);
            let table = make_query_table(&data, size.width);
            let graph = make_history_chart(&stats);
            let paragraph = render_status_paragraph(&status, &stats);
            let filters = make_filters_list(filters.filters.as_slice(), size.width);
            let top_queried_domains = make_list("Top Queried Domains", &stats.top_queried_domains, Color::Green, size.width);
            let top_blocked_domains = make_list("Top Blocked Domains", &stats.top_blocked_domains, Color::Red, size.width);
            let top_clients = make_list("Top Clients", &stats.top_clients, Color::Cyan, size.width);

            let constraints = if size.height > 42 {
                vec![
                    Constraint::Percentage(30),
                    Constraint::Min(1),
                    Constraint::Percentage(20)
                ]
            } else {
                vec![
                    Constraint::Percentage(30),
                    Constraint::Min(1),
                    Constraint::Percentage(0)
                ]
            };

            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&*constraints)
            .split(size);

            // Split the top part (charts + gauge) into left (gauge + block) and right (line chart)
            let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
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
                .constraints(
                    [
                        Constraint::Min(0),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(top_chunks[0]);

            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(25), 
                        Constraint::Percentage(25), 
                        Constraint::Percentage(25), 
                        Constraint::Percentage(25), 
                    ]
                    .as_ref(),
                )
                .split(chunks[2]);

            // Render the widgets to the UI
            f.render_widget(paragraph, left_chunks[0]);
            f.render_widget(gauge, left_chunks[1]);
            f.render_widget(graph, top_chunks[1]);
            f.render_widget(table, chunks[1]);
            if size.height > 42 {
                f.render_widget(filters, bottom_chunks[0]);
                f.render_widget(top_queried_domains, bottom_chunks[1]);
                f.render_widget(top_blocked_domains, bottom_chunks[2]);
                f.render_widget(top_clients, bottom_chunks[3]);
            }
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

