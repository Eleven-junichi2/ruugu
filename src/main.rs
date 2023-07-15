mod models;

use std::error::Error;

use crossterm::{
    cursor,
    event::{read, KeyEvent, KeyCode, Event},
    execute,
    style::Print,
    terminal,
};

use models::*;

fn main() -> Result<(), Box<dyn Error>> {
    
    loop {
        if let Event::Key(KeyEvent { code, modifiers, kind, state}) = read()? {
            dbg!(code, modifiers, kind, state);
        }
        // dbg!(read()?);
    }
    Ok(())
}
