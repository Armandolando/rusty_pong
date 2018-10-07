use piston_window::*;
use piston_window::types::Color;

use bar::{Bar, Direction};
use draw::draw_rectange;
use ball::{Ball, BallDir};

const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.30, 0.24, 0.5];

const MOVING_PERIOD: f64 = 0.1; 
const RESTART_TIME: f64 = 1.0;

pub struct Game{
	bar1: Bar,
	bar2: Bar,
	ball: Ball,
	waiting_time: f64,
	width: i32,
	height: i32,
	
}

impl Game{
	pub fn new(width: i32, height: i32) -> Game{
		Game{
			bar1: Bar::new(2, 2),
			bar2: Bar::new(37, 2),
			ball: Ball::new(20, 12, BallDir::Up, BallDir::Right),
            waiting_time: 0.0,
            width,
            height
		}
	}
	
	pub fn key_pressed(&mut self, key: Key){
		if !self.is_game_over(){
			return;
		}
		
		let (dir, bar_flag) = match key{
			Key::Up => (Some(Direction::Up), true),
			Key::Down => (Some(Direction::Down), true),
			Key::W => (Some(Direction::Up), false),
			Key::S => (Some(Direction::Down), false),
			_ => (None, true)
		};
		
		if bar_flag{
			self.update_bar2(dir);
		}else{
			self.update_bar1(dir);
		}
	}
	
	pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.bar1.draw(con, g);
        self.bar2.draw(con, g);
        self.ball.draw(con, g);
        
        draw_rectange(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectange(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectange(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectange(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        
        if !self.is_game_over() {
            draw_rectange(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }
    
     pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        
        if !self.is_game_over() {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        
        if self.waiting_time > MOVING_PERIOD {
        	let opposite_ver = self.ball.vertical_direction().opposite_dir();
        	let opposite_hor = self.ball.horizontal_direction().opposite_dir();
        	
        	match self.check_bounce(Some(self.ball.horizontal_direction()), Some(self.ball.vertical_direction())){
        	(true, true) => self.update_ball(None, None),
        	(false, true) => self.update_ball(Some(opposite_ver), None),
        	(true, false) => self.update_ball(None, Some(opposite_hor)),
        	(false, false) => self.update_ball(Some(opposite_ver), Some(opposite_hor))
        	};
            
        }
    }
    
    fn update_ball(&mut self, hor_dir: Option<BallDir>, ver_dir: Option<BallDir>) {
        
        self.ball.move_forward(hor_dir, ver_dir);
      
        self.waiting_time = 0.0;
    }
    
    pub fn check_bounce(&self, hor_dir: Option<BallDir>, ver_dir: Option<BallDir>)->(bool,bool){
    	
    	let (next_x, next_y) = self.ball.next_head_pos(ver_dir, hor_dir);
    	
    	let ver = next_y < self.height - 1 && next_y > 0;
    	
    	let hor = self.bar1.bounce(next_x, next_y) && self.bar2.bounce(next_x, next_y);
    	
    	(ver, hor)  
    }
    
    fn update_bar1(&mut self, dir: Option<Direction>) {
    	let (next_head, next_tail) = self.bar1.next_pos(dir);
    	if next_head < self.height - 3 && next_tail > 2 {
        	self.bar1.move_bar(dir);
        }
        
    }
    
    fn update_bar2(&mut self, dir: Option<Direction>) {
        let (next_head, next_tail) = self.bar2.next_pos(dir);
        if next_head < self.height - 3 && next_tail > 2 {
        	self.bar2.move_bar(dir);
        }
    }
    
    fn is_game_over(&self) -> bool{
    	let (next_x, _next_y) = self.ball.next_head_pos(Some(self.ball.vertical_direction()), Some(self.ball.horizontal_direction()));
    	
    	next_x > 0 && next_x < self.width - 1
    }
    
    fn restart(&mut self){
    	let ver = self.ball.vertical_direction().opposite_dir();
    	let hor = self.ball.horizontal_direction().opposite_dir();
    	
    	self.bar1= Bar::new(2, 2);
		self.bar2= Bar::new(37, 2);
		self.ball= Ball::new(20, 12, ver, hor);
        self.waiting_time= 0.0;
        
    }
}	


