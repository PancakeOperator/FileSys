use tui::Terminal;
use crossterm::{
    event::{self ,Event, KeyCode}, terminal::EnterAlternateScreen, execute,
};
use std::io::Result;
use tui::{ backend::Backend, backend::CrosstermBackend,layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, BorderType, Tabs}, Frame,
};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{ List, ListItem, Paragraph},
};
use std::{io, thread, time::Duration};
pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            titles: vec![],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {} 
            }   
        }
    }

}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::Black));
    f.render_widget(block, size);
    
    Tabitha::draw_tabs(f, chunks[0], app);
    let inner = match app.index {
        0 => Block::default().title("Inner 0").borders(Borders::ALL),
        1 => Block::default().title("Inner 1").borders(Borders::ALL),
        2 => Block::default().title("Inner 2").borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
    let blocksa = Block::default().title("calc").borders(Borders::ALL);

    let chunked = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);
    f.render_widget(blocksa, chunked[1]);
}


pub struct Tabitha {
    pub index: usize,
}


impl Tabitha {
    const titles: Vec<vec![]> = ["News", "Sports", "Stocks"].iter().cloned().map(Spans::from).collect();
    pub fn draw_tabs<B: Backend>(f: &mut Frame<B> ,area: Rect, app: &App) {
        let horizontal_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
                    .split(area); 
        
        
        
        let tabby = Tabs::new(Tabitha::titles)
                .block(Block::default().borders(Borders::ALL).title("Menu"))
                .select(app.index)
                .style(Style::default().fg(Color::DarkGray));
        
        f.render_widget(tabby, horizontal_chunks[0]);
    }
    
    pub fn nexts(&mut self) {
        self.index = (self.index + 1) % Tabitha::titles.len();
    }
    
}
