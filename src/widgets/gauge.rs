use tui::{
  style::{Color, Style, Modifier},
  widgets::{Block, Borders, Gauge},
  text::{Span},
};

use crate::fetch::fetch_stats::StatsResponse;

pub fn make_gauge(stats: &StatsResponse) -> Gauge {

  let total_blocked = stats.num_blocked_filtering
    + stats.num_replaced_parental
    + stats.num_replaced_safebrowsing
    + stats.num_replaced_safesearch;

  let percent = (total_blocked as f64 / stats.num_dns_queries as f64 * 100.0) as u16;

  let label = format!("Blocked {} out of {} ({}%)", total_blocked, stats.num_dns_queries, percent);

  Gauge::default()
      .block(
        Block::default()
        .title(Span::styled(
          "Block Percentage",
          Style::default().add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
    )
    .gauge_style(Style::default().fg(Color::Red).bg(Color::Green))
    .percent(percent)
    .label(label)
}
