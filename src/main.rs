mod models;

use std::error::Error;

use crossterm::{
    cursor,
    event,
    execute,
    style::Print,
    terminal,
};

use models::*;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        dbg!(event::read()?);
    }
    Ok(())
}
