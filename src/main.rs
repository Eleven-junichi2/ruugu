mod models;

use std::error::Error;

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind},
    execute,
    style::Print,
    terminal,
};

use models::*;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
