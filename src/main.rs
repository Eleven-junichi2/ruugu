use std::error::Error;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal,
};

use ruugu::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut worldmap = WorldMap::from_size(9, 9);
    worldmap.mapchip_to_display_dict.insert(0, ' ');
    worldmap.mapchip_to_display_dict.insert(1, '.');
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;
    loop {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = read()?
        {
            if code == KeyCode::Esc {
                break;
            }
            dbg!(code, modifiers, kind, state);
        }
        // dbg!(read()?);
        execute!(
            stdout,
            Print(worldmap.render_to_lines())
        )?;
    }
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

// mod models;

// use std::error::Error;

// use crossterm::{
//     cursor,
//     event::{Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind},
//     execute,
//     style::Print,
//     terminal,
// };

// use models::*;

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut player = Mob {
//         coordinate: DungeonMapCoordinate { x: 0, y: 0 },
//         status: Status {
//             hp: Hp {
//                 max: 50,
//                 current: 50,
//             },
//             exp: Exp {
//                 next: 100,
//                 current: 0,
//                 carrying: 0,
//             },
//             level: Level {
//                 max: 100,
//                 current: 1,
//             },
//             strength: Strength { max: 3, current: 3 },
//             defense: Defense { max: 1, current: 1 },
//             agility: Agility { max: 3, current: 3 },
//         },
//         appearance_char: '@',
//     };

//     let mut stdout = std::io::stdout();
//     terminal::enable_raw_mode()?;
//     execute!(stdout, terminal::EnterAlternateScreen)?;
//     loop {
//         execute!(stdout, cursor::Hide, cursor::MoveTo(0, 0),)?;
//         // --handle events--
//         match crossterm::event::read()? {
//             Event::Key(event) => {
//                 execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
//                 match event {
//                     KeyEvent {
//                         code: KeyCode::Esc, ..
//                     }
//                     | KeyEvent {
//                         code: KeyCode::Char('c'),
//                         modifiers: KeyModifiers::CONTROL,
//                         ..
//                     } => break,
//                     KeyEvent {
//                         code: KeyCode::Left,
//                         kind: KeyEventKind::Press,
//                         ..
//                     } => {
//                         player.coordinate.x -= 1;
//                         println!("go left")
//                     }
//                     KeyEvent {
//                         code: KeyCode::Up,
//                         kind: KeyEventKind::Press,
//                         ..
//                     } => {
//                         player.coordinate.y -= 1;
//                         println!("go up")
//                     }
//                     KeyEvent {
//                         code: KeyCode::Right,
//                         kind: KeyEventKind::Press,
//                         ..
//                     } => {
//                         player.coordinate.x += 1;
//                         println!("go right")
//                     }
//                     KeyEvent {
//                         code: KeyCode::Down,
//                         kind: KeyEventKind::Press,
//                         ..
//                     } => {
//                         player.coordinate.y += 1;
//                         println!("go down")
//                     }
//                     _ => println!("{:?}", event),
//                 }
//             }
//             _ => {}
//         }
//         execute!(
//             stdout,
//             cursor::MoveTo(player.coordinate.x as u16, player.coordinate.y as u16),
//             Print(player.appearance_char)
//         )?;
//         // ----
//     }
//     execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
//     terminal::disable_raw_mode()?;
//     Ok(())
// }
