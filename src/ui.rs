use crate::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, List, ListItem, Paragraph
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App){
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(90), Constraint::Percentage(5), Constraint::Percentage(5)].as_ref())
        .split(f.size());
    let guesses: Vec<ListItem> = app
        .guesses
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, g)))];
            ListItem::new(content)
        })
        .collect();

    let guesses =
        List::new(guesses).block(Block::default().borders(Borders::ALL).title("Guesses"));
    f.render_widget(guesses, chunks[0]);

    let res = Paragraph::new(app.last_return.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Last Return Code"));
    f.render_widget(res, chunks[1]);

    let p = Paragraph::new(app.input.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(p, chunks[2]);
}
