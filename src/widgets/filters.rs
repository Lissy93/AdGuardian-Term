// filters.rs

use tui::{
  layout::Rect,
  text::{Span, Spans},
  widgets::{Block, Borders, List, ListItem},
  style::{Color, Style, Modifier},
  backend::CrosstermBackend,
  Frame,
};

use crate::fetch::fetch_filters::Filter;

fn truncate(text: &str, width: usize) -> String {
  if text.chars().count() <= width {
      text.to_string()
  } else {
      text.chars().take(width - 3).collect::<String>() + "..."
  }
}

pub fn make_filters_list(filters: &[Filter], width: u16) -> List {
  let items: Vec<ListItem> = filters
    .iter()
    .map(|filter| {
        let (status_text, color) = if filter.enabled {
            ("✔", Color::Green)
        } else {
            ("✘", Color::Red)
        };
        let status = Span::styled(status_text, Style::default().fg(color));
        let rule_count = Span::styled(format!(" ({})", filter.rules_count), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD));
        let name = Span::raw(format!(" {}", truncate(&filter.name, width as usize / 4 - 14)));
        let content = Spans::from(vec![status, name, rule_count]);
        ListItem::new(content)
    })
    .collect();

  List::new(items)
      .block(
          Block::default()
              .borders(Borders::ALL)
              .title(Span::styled(
                "Filters",
                Style::default().add_modifier(Modifier::BOLD),
              )),
      )
}
