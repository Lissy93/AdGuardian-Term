use std::{
  io::stdout,
  sync::Arc,
  time::Duration,
};
use chrono::{DateTime, Utc};
use crossterm::{
  event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  widgets::{Axis, Block, Borders, Cell, Row, Table, Gauge, Chart, Dataset},
  text::{Span},
  symbols,
  Terminal,
};

use crate::fetch_stats::{StatsResponse};

use crate::fetch::{Query, Question};

fn make_gauge(stats: &StatsResponse) -> Gauge {

  let total_blocked = stats.num_blocked_filtering
    + stats.num_replaced_parental
    + stats.num_replaced_safebrowsing
    + stats.num_replaced_safesearch;

  let percent = (total_blocked as f64 / stats.num_dns_queries as f64 * 100.0) as u16;

  let label = format!("Blocked {} out of {} requests ({}%)", total_blocked, stats.num_dns_queries, percent);

  Gauge::default()
      .block(Block::default().title("Block Percentage")
      .borders(Borders::ALL))
      .gauge_style(Style::default().fg(Color::Red).bg(Color::Green))
      .percent(percent)
      .label(label)
}

fn make_query_table<'a>(data: &'a [Query]) -> Table<'a> {
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
      Row::new(vec![time, question, status, client, elapsed_ms])
          .style(Style::default().fg(color))
  }).collect::<Vec<Row>>(); // Clone the data here

  let table = Table::new(rows) // Table now owns its data
      .header(Row::new(vec![
        Cell::from(Span::raw("Time")),
        Cell::from(Span::raw("Request")),
        Cell::from(Span::raw("Status")),
        Cell::from(Span::raw("Client")),
        Cell::from(Span::raw("Time Taken")),
      ]))
      .block(Block::default().title("Query Log").borders(Borders::ALL))
      .widths(&[
        Constraint::Percentage(15),
        Constraint::Percentage(35),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
      ]);

  table
}

fn make_history_datasets<'a>(stats: &'a StatsResponse) -> Vec<Dataset<'a>> {
  let dns_queries_dataset = Dataset::default()
      .name("DNS Queries")
      .marker(symbols::Marker::Braille)
      .style(Style::default().fg(Color::Green))
      .data(&stats.dns_queries_chart);

  let blocked_filtering_dataset = Dataset::default()
      .name("Blocked Filtering")
      .marker(symbols::Marker::Braille)
      .style(Style::default().fg(Color::Red))
      .data(&stats.blocked_filtering_chart);

  let datasets = vec![dns_queries_dataset, blocked_filtering_dataset];

  datasets
}

fn find_bounds(stats: &StatsResponse) -> (f64, f64) {
    let mut max_length = 0;
    let mut max_value = f64::MIN;

    for dataset in &[&stats.dns_queries_chart, &stats.blocked_filtering_chart] {
        let length = dataset.len();
        if length > max_length {
            max_length = length;
        }

        let max_in_dataset = dataset
            .iter()
            .map(|&(_, y)| y)
            .fold(f64::MIN, f64::max);
        if max_in_dataset > max_value {
            max_value = max_in_dataset;
        }
    }
    (max_length as f64, max_value)
}

fn generate_x_labels(max_days: i32, num_labels: i32) -> Vec<Span<'static>> {
    let step = (max_days / (num_labels - 1)) as i32;
    (0..num_labels)
        .map(|i| {
            let day = (max_days - i * step).to_string();
            if i == num_labels - 1 {
                Span::styled("Today", Style::default().add_modifier(Modifier::BOLD))
            } else {
                Span::raw(day)
            }
        })
        .collect()
}

fn generate_y_labels(max: i32, count: usize) -> Vec<Span<'static>> {
    let step = max / (count - 1) as i32;
    let labels = (0..count)
        .map(|x| Span::raw(format!("{}", x * step as usize)))
        .collect::<Vec<Span<'static>>>();
    labels
}

fn make_history_chart<'a>(stats: &'a StatsResponse) -> Chart<'a> {
    // Convert datasets into vector that can be consumed by chart
    let datasets = make_history_datasets(&stats);
    // Find uppermost x and y-axis bounds for chart
    let (x_bound, y_bound) = find_bounds(&stats);
    // Generate incremental labels from data's values, to render on axis
    let x_labels = generate_x_labels(stats.dns_queries.len() as i32, 5);
    let y_labels = generate_y_labels(y_bound as i32, 5);
    // Create chart
    let chart = Chart::new(datasets)
        .block(Block::default().title("History").borders(Borders::ALL))
        .x_axis(
            Axis::default()
            .title("Time (Days ago)")
            .bounds([0.0, x_bound])
            .labels(x_labels),
        )
        .y_axis(Axis::default().title("Query Count").labels(y_labels).bounds([0.0, y_bound]));

    chart
}

fn convert_to_chart_data(data: Vec<f64>) -> Vec<(f64, f64)> {
    data.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect()
}

// Interpolates data, adding n number of points, to make the chart look smoother
fn interpolate(input: &Vec<f64>, points_between: usize) -> Vec<f64> {
    let mut output = Vec::new();

    for window in input.windows(2) {
        let start = window[0];
        let end = window[1];
        let step = (end - start) / (points_between as f64 + 1.0);

        output.push(start);
        for i in 1..=points_between {
            output.push(start + step * i as f64);
        }
    }

    output.push(*input.last().unwrap());
    output
}

// Adds data formatted for the time-series chart to the stats object
fn prepare_data(stats: &mut StatsResponse) {
    let dns_queries = stats.dns_queries.iter().map(|&v| v as f64).collect::<Vec<_>>();
    let interpolated_dns_queries = interpolate(&dns_queries, 3);
    stats.dns_queries_chart = convert_to_chart_data(interpolated_dns_queries);
    
    let blocked_filtering: Vec<f64> = stats.blocked_filtering.iter()
        .zip(&stats.replaced_safebrowsing)
        .zip(&stats.replaced_parental)
        .map(|((&b, &s), &p)| (b + s + p) as f64)
        .collect();
    
    let interpolated_blocked_filtering = interpolate(&blocked_filtering, 3);
    let blocked_filtering_chart: Vec<(f64, f64)> = convert_to_chart_data(interpolated_blocked_filtering);
    
    stats.blocked_filtering_chart = blocked_filtering_chart;
}



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
        prepare_data(&mut stats);

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
