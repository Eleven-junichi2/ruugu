mod models;

use std::{error::Error};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, ModifierKeyCode, KeyModifiers},
    execute,
    style::Print,
    terminal,
};

use models::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;
    loop {
        execute!(stdout, cursor::Hide, cursor::MoveTo(0, 0),)?;
        // --handle events--
        match crossterm::event::read()? {
            Event::Key(event) => {
                execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
                match event {
                    KeyEvent {
                        code: KeyCode::Esc, ..
                    } | KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => break,
                    KeyEvent {
                        code: KeyCode::Left,
                        ..
                    } => println!("go left"),
                    KeyEvent {
                        code: KeyCode::Up, ..
                    } => println!("go up"),
                    KeyEvent {
                        code: KeyCode::Right,
                        ..
                    } => println!("go right"),
                    KeyEvent {
                        code: KeyCode::Down,
                        ..
                    } => println!("go down"),
                    _ => println!("{:?}", event),
                }
            }
            _ => {}
        }
        // ----
    }
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
