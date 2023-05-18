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
  style::{Color, Style},
  widgets::Block,
  Terminal, 
};

use crate::fetch::fetch_stats::StatsResponse;
use crate::fetch::fetch_query_log::Query;

use crate::widgets::gauge::make_gauge;
use crate::widgets::table::make_query_table;
use crate::widgets::chart::{make_history_chart, prepare_chart_data};

pub async fn draw_ui(
    mut data_rx: tokio::sync::mpsc::Receiver<Vec<Query>>,
    mut stats_rx: tokio::sync::mpsc::Receiver<StatsResponse>,
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

        // Prepare the data for the chart
        prepare_chart_data(&mut stats);

        terminal.draw(|f| {
          let size = f.size();

          // Make the charts
          let gauge = make_gauge(&stats);
          let table = make_query_table(&data);
          let graph = make_history_chart(&stats);

          
            // Split the layout into parts
            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Percentage(50),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(size);

            let block = Block::default()
              .title("AdGuard Dashboard")
              .style(Style::default().fg(Color::Reset));
            f.render_widget(block, size);

            f.render_widget(gauge, chunks[0]);
            f.render_widget(table, chunks[1]);
            f.render_widget(graph, chunks[2]);
  
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

