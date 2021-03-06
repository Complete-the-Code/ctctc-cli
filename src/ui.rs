use crate::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(3, 4),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
            ]
            .as_ref(),
        )
        .split(f.size());

    let guesses: Vec<ListItem> = app
        .guesses
        .iter()
        .enumerate()
        .map(|(_i, g)| {
            let content = vec![Spans::from(Span::raw(g))];
            ListItem::new(content)
        })
        .collect();

    let guesses = List::new(guesses).block(Block::default().borders(Borders::ALL).title("Guesses"));
    f.render_widget(guesses, chunks[0]);

    let mut color = match app.return_code {
        x if x >= 400 && x < 500 => Color::Red,
        x if x >= 500 => Color::Yellow,
        x if x >= 200 && x < 400 => Color::Green,
        _ => Color::White,
    };
    if app.last_return == "Already guessed, dipshit." {
        color = Color::Red;
    }
    let res = Paragraph::new(app.last_return.as_ref())
        .style(Style::default().fg(color))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Last Return Code"),
        );
    f.render_widget(res, chunks[1]);

    let p = Paragraph::new(app.input.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(p, chunks[2]);
}
