use tui::{
  style::{Color, Style, Modifier},
  text::{Span, Spans},
  widgets::{Paragraph, Wrap, Borders, Block},
};

use crate::fetch::fetch_status::StatusResponse;
use crate::fetch::fetch_stats::StatsResponse;

pub fn render_status_paragraph<'a>(status: &'a StatusResponse, stats: &'a StatsResponse) -> Paragraph<'a> {

  let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            "Status",
            Style::default().add_modifier(Modifier::BOLD),
        ));

    let get_color = |enabled: bool| {
      if enabled { Color::Green } else { Color::Red }
    };

    let value_style = Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD);

    let coloured = |color: Color| {
      Style::default().fg(color).add_modifier(Modifier::BOLD)
    };
      

  let text = vec![
      Spans::from(vec![
        Span::styled("Running: ", Style::default()),
        Span::styled(
            format!("{}", status.running),
            Style::default().fg(get_color(status.running)).add_modifier(Modifier::BOLD)
        ),
    ]),
      Spans::from(vec![
        Span::styled("Protection Enabled: ", Style::default()),
        Span::styled(
            format!("{}", status.protection_enabled),
            Style::default().fg(get_color(status.protection_enabled)).add_modifier(Modifier::BOLD)
        ),
    ]),
      Spans::from(vec![
        Span::styled("DHCP Available: ", Style::default()),
        Span::styled(
            format!("{}", status.dhcp_available),
            Style::default().fg(get_color(status.dhcp_available)).add_modifier(Modifier::BOLD)
        ),
    ]),
    Spans::from(vec![
      Span::styled("Avg Processing Time: ", Style::default()),
      Span::styled(format!("{}ms", stats.avg_processing_time), value_style),
    ]),
    Spans::from(vec![
      Span::styled("Version: ", Style::default()),
      Span::styled(status.version.to_string(), value_style),
    ]),
    Spans::from(vec![
        Span::styled("Ports: ", Style::default()),
        Span::styled(format!(":{} (DNS), :{} (HTTP)", status.dns_port, status.http_port), value_style),
    ]),
    Spans::from(vec![
      Span::styled("Total Queries: ", Style::default()),
      Span::styled(stats.num_dns_queries.to_string(), coloured(Color::Green)),
    ]),
    Spans::from(vec![
      Span::styled("Filtered: ", Style::default()),
      Span::styled(stats.num_blocked_filtering.to_string(), coloured(Color::Yellow)),
    ]),
    Spans::from(vec![
      Span::styled("Malware Blocked: ", Style::default()),
      Span::styled(stats.num_replaced_safebrowsing.to_string(), coloured(Color::Red)),
    ]),
    Spans::from(vec![
      Span::styled("Parental Controls: ", Style::default()),
      Span::styled(stats.num_replaced_parental.to_string(), coloured(Color::Magenta)),
    ]),
    Spans::from(vec![
      Span::styled("Safe Search: ", Style::default()),
      Span::styled(stats.num_replaced_safesearch.to_string(), coloured(Color::Cyan)),
    ]),
  ];
  Paragraph::new(text)
    .wrap(Wrap { trim: true })
    .block(block)
}
