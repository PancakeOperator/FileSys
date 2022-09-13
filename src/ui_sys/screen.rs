use crossterm::cursor;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use crossterm::execute;
use crossterm::terminal;
use std::io;

pub fn setup_terminal() {
	let mut stdout = io::stdout();

	execute!(stdout, terminal::EnterAlternateScreen).unwrap();
	execute!(stdout, cursor::Hide).unwrap();

	execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

	terminal::enable_raw_mode().unwrap();
}

pub fn cleanup_terminal() {
	let mut stdout = io::stdout();

	
	execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
	execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

	execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
	execute!(stdout, cursor::Show).unwrap();

	terminal::disable_raw_mode().unwrap();
}

