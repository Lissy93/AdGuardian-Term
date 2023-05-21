
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Axis, Block, Borders, Dataset, Chart},
    text::{Span},
    symbols,
};

use crate::fetch::fetch_stats::{StatsResponse};


pub fn make_history_chart<'a>(stats: &'a StatsResponse) -> Chart<'a> {
    // Convert datasets into vector that can be consumed by chart
    let datasets = make_history_datasets(&stats);
    // Find uppermost x and y-axis bounds for chart
    let (x_bound, y_bound) = find_bounds(&stats);
    // Generate incremental labels from data's values, to render on axis
    let x_labels = generate_x_labels(stats.dns_queries.len() as i32, 5);
    let y_labels = generate_y_labels(y_bound as i32, 5);
    // Create chart
    let chart = Chart::new(datasets)
        .block(
            Block::default()
            .title(Span::styled(
                "History",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
        )
        .x_axis(
            Axis::default()
            .title("Time (Days ago)")
            .bounds([0.0, x_bound])
            .labels(x_labels),
        )
        .y_axis(Axis::default().title("Query Count").labels(y_labels).bounds([0.0, y_bound]));

    chart
}

// Returns a dataset that's consumable by the chart widget
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

// Determine the uppermost bounds for the x and y axis
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

// Generate periodic labels to render on the y-axis (query count)
fn generate_y_labels(max: i32, count: usize) -> Vec<Span<'static>> {
  let step = max / (count - 1) as i32;
  let labels = (0..count)
      .map(|x| Span::raw(format!("{}", x * step as usize)))
      .collect::<Vec<Span<'static>>>();
  labels
}

// Generate periodic labels to render on the x-axis (days ago)
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

// Formats vector data into a format that can be consumed by the chart widget
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
pub fn prepare_chart_data(stats: &mut StatsResponse) {
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
