
use tui::{
  style::{Color, Modifier, Style},
  widgets::{Block, Borders, Cell, Row, Table},
  text::{Span},
  layout::{Constraint},
};
use chrono::{DateTime, Utc};

use crate::fetch::fetch_query_log::{Query, Question};

pub fn make_query_table<'a>(data: &'a [Query]) -> Table<'a> {
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

// Given a timestamp, return a string representing how long ago that was
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

// Return cell showing info about the request made in a given query
fn make_request_cell(q: &Question) -> Result<String, anyhow::Error> {
  Ok(format!("[{}] {} - {}", q.class, q.question_type, q.name))
}

// Return a cell showing the time taken for a query, and a color based on time
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

// Return color for a row, based on the allow/block reason
fn make_row_color(reason: &str) -> Color {
  return if reason == "NotFilteredNotFound" {
      Color::Green
  } else if reason == "FilteredBlackList" {
      Color::Red
  } else {
      Color::Yellow
  }
}

// Return text and color for the status cell based on allow/ block reason
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

