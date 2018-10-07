extern crate piston_window;
extern crate rand;

mod bar;
mod game;
mod draw;
mod ball;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_gui_coord_u32;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

fn main() {
    let (width, heigth) = (40, 24);
    
    let mut window: PistonWindow = WindowSettings::new("Rusty Pong", [to_gui_coord_u32(width), to_gui_coord_u32(heigth)]).exit_on_esc(true).build().unwrap();
    
    let mut game = Game::new(width, heigth);
    
    while let Some(event) = window.next() {

        
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        
        event.update(|arg| {
            game.update(arg.dt);
        });
	}
}
