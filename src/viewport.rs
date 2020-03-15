use crate::events::ViewportEvent;
use crate::world::{DOWN, LEFT, RIGHT, UP};
use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::input::{input, InputEvent, KeyEvent, TerminalInput};
use crossterm::screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen};
use crossterm::terminal;
use crossterm::{execute, queue, Output};
use log::debug;
use std::io::{stdout, Write};

use crate::entities::{Entity, Player};
use crate::state::State;
use crate::tiling::tile_to_str;
use crate::world::Point;

pub trait ViewPort {
    fn render_state(&mut self, state: &State);
    fn wait_input(&mut self) -> Option<ViewportEvent>;
}

pub struct CrossTermViewPort {
    xsize: usize,
    ysize: usize,
    raw: RawScreen,
    input: TerminalInput,
    start: Point,
}

impl CrossTermViewPort {
    pub fn new() -> CrossTermViewPort {
        // Initialise the terminal, the raw alternate mode allows direct character
        // seeking and hides the prompt.
        let term_size = terminal::size().unwrap();
        execute!(stdout(), EnterAlternateScreen).unwrap();
        execute!(stdout(), cursor::Hide).unwrap();
        let raw = RawScreen::into_raw_mode().unwrap();

        // Initialise state, create the player and dungeon
        let xsize = term_size.0 as usize;
        let ysize = (term_size.1 - 2) as usize;

        let input = input();

        CrossTermViewPort {
            xsize,
            ysize,
            raw,
            input,
            start: (0, 0),
        }
    }

    fn draw_level(&self, state: &State) {
        let mut sout = stdout();
        let grid = state.get_grid().unwrap();
        execute!(sout, MoveTo(0, 0)).unwrap();
        for (linenum, line) in grid.raw_data().iter().enumerate() {
            debug!("Drawing linenum {} -- {:?}", linenum, line);
            let linestr = line.iter().map(tile_to_str).collect::<Vec<&str>>();
            let mut linestr2 = String::from("");
            for chr in linestr {
                linestr2.push_str(chr);
            }
            queue!(sout, Output(linestr2), MoveTo(0, linenum as u16)).unwrap();
            sout.flush().unwrap();
        }
    }

    fn draw_entity(&self, state: &State, entity: &dyn Entity) {
        if !entity.is_visible() || !entity.is_dirty() {
            return;
        }
        let grid = state.get_grid().unwrap();
        let dirt = entity.previous_location();
        let background = grid.block_at(dirt.0, dirt.1);
        let mut sout = stdout();
        queue!(
            sout,
            MoveTo(dirt.0 as u16, dirt.1 as u16),
            Output(tile_to_str(background)),
            MoveTo(entity.location().0 as u16, entity.location().1 as u16),
            Output(tile_to_str(entity.tile()))
        )
        .unwrap();
        sout.flush().unwrap();
    }

    fn draw_entities(&self, state: &State) {
        for e in state.current_level().entities.iter() {
            self.draw_entity(state, &**e);
        }
    }

    fn draw_player(&mut self, state: &State) {
        self.draw_entity(state, state.get_player());
    }

    fn ui_state_position(&self) -> MoveTo {
        MoveTo(0, (self.ysize) as u16)
    }

    fn ui_notification_position(&self) -> MoveTo {
        MoveTo(0, (self.ysize + 1) as u16)
    }

    fn draw_ui(&self, state: &State) {
        let mut sout = stdout();
        queue!(
            sout,
            self.ui_state_position(),
            Output(state.get_player().stats())
        )
        .unwrap();
        sout.flush().unwrap();
    }

    pub fn notify(&self, message: String) {
        let mut sout = stdout();
        queue!(
            sout,
            self.ui_notification_position(),
            Output(" ".repeat(self.xsize)),
            self.ui_notification_position(),
            Output(message)
        )
        .unwrap();
        sout.flush().unwrap();
    }

    pub fn ui_help(&self) {
        self.notify(String::from(
            "quit: q, movement{up(k), down(j), left(h), right(l)}",
        ))
    }
}

impl ViewPort for CrossTermViewPort {
    fn render_state(&mut self, state: &State) {
        self.draw_level(state);
        self.draw_entities(state);
        self.draw_player(state);
        self.draw_ui(state);
    }

    fn wait_input(&mut self) -> Option<ViewportEvent> {
        let mut reader = self.input.read_sync();

        if let Some(event) = reader.next() {
            return match event {
                InputEvent::Keyboard(KeyEvent::Char('q')) => Some(ViewportEvent::Quit),
                InputEvent::Keyboard(KeyEvent::Char('?')) => {
                    self.ui_help();
                    None
                }
                InputEvent::Keyboard(KeyEvent::Char('j')) => Some(ViewportEvent::MovePlayer(DOWN)),
                InputEvent::Keyboard(KeyEvent::Char('k')) => Some(ViewportEvent::MovePlayer(UP)),
                InputEvent::Keyboard(KeyEvent::Char('h')) => Some(ViewportEvent::MovePlayer(LEFT)),
                InputEvent::Keyboard(KeyEvent::Char('l')) => Some(ViewportEvent::MovePlayer(RIGHT)),
                // Arrow keys for noobs
                InputEvent::Keyboard(KeyEvent::Down) => Some(ViewportEvent::MovePlayer(DOWN)),
                InputEvent::Keyboard(KeyEvent::Up) => Some(ViewportEvent::MovePlayer(UP)),
                InputEvent::Keyboard(KeyEvent::Left) => Some(ViewportEvent::MovePlayer(LEFT)),
                InputEvent::Keyboard(KeyEvent::Right) => Some(ViewportEvent::MovePlayer(RIGHT)),

                // Stairs
                InputEvent::Keyboard(KeyEvent::Char('>')) => Some(ViewportEvent::DownStairs),
                InputEvent::Keyboard(KeyEvent::Char('<')) => Some(ViewportEvent::UpStairs),

                // No match
                _ => None,
            };
        }
        None
    }
}

impl Drop for CrossTermViewPort {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        execute!(stdout(), cursor::Show).unwrap();
    }
}
