
use crate::fetch::fetch_stats::DomainData;


use tui::{
  text::{Span, Spans},
  widgets::{Block, Borders, List, ListItem},
  style::{Color, Style, Modifier},
};

fn truncate(text: &str, width: usize) -> String {
  if text.chars().count() <= width {
      text.to_string()
  } else {
      text.chars().take(width - 3).collect::<String>() + "..."
  }
}

pub fn make_list<'a>(title: &'a str, data: &[DomainData], color: Color, width: u16) -> List<'a> {
  let items: Vec<ListItem> = data
      .iter()
      .map(|data| {

          let name = Span::raw(format!(" {}", truncate(&data.name, width as usize / 4 - 12)));
          let count = Span::styled(format!(" ({})", data.count), Style::default().fg(color).add_modifier(Modifier::BOLD));
          ListItem::new(Spans::from(vec![name, count]))
      })
      .collect();

  List::new(items)
      .block(Block::default().borders(Borders::ALL)
      .title(Span::styled(
        title,
        Style::default().add_modifier(Modifier::BOLD),
      )))
      .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}

