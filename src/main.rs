use std::error::Error;

use crossterm::{
    cursor,
    event::{read, KeyEvent, KeyCode, Event},
    execute,
    style::Print,
    terminal,
};

use ruugu::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut worldmap = WorldMap::from_size(9, 9);
    worldmap.mapchip_to_display_dict.insert(0, ' ');
    worldmap.mapchip_to_display_dict.insert(1, '.');
    loop {
        if let Event::Key(KeyEvent { code, modifiers, kind, state}) = read()? {
            dbg!(code, modifiers, kind, state);
        }
        // dbg!(read()?);
    }
    Ok(())
}
