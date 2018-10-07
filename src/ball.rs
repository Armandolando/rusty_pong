use std::collections::LinkedList;

use piston_window::Context;
use piston_window::G2d;
use piston_window::types::Color;
use bar::Block;

use draw::draw_block;

const S_COLOR : Color = [0.20, 0.70, 0.30, 1.0];


#[derive(Clone, Copy, PartialEq)]
pub enum BallDir{
	Up,
	Down,
	Right,
	Left
}

impl BallDir{
	pub fn opposite_dir(&self) -> BallDir{
		match *self{
			BallDir::Up => BallDir::Down,
			BallDir::Down => BallDir::Up,
			BallDir::Right => BallDir::Left,
			BallDir::Left => BallDir::Right
		}
	}
}

pub struct Ball{
	vertical_dir : BallDir,
	horizontal_dir : BallDir,
	ball_block : LinkedList<Block>
}

impl Ball{
	pub fn new(init_x: i32, init_y: i32, vertical_dir: BallDir, horizontal_dir: BallDir) -> Ball{
		let mut ball_block : LinkedList<Block> = LinkedList::new();
		
		ball_block.push_back(Block{
			x: init_x,
			y: init_y
		});
		
		Ball{
			vertical_dir,
			horizontal_dir,
			ball_block
		}
	}
	
	pub fn draw(&self, con: &Context, g: &mut G2d){
		for block in &self.ball_block{
			draw_block(S_COLOR, block.x, block.y, con, g);
		}
	}
	
	pub fn move_forward(&mut self, ver_dir: Option<BallDir>, hor_dir: Option<BallDir>){
		match ver_dir{
			Some(d) => self.vertical_dir = d,
			None => {}
		}
		
		match hor_dir{
			Some(d) => self.horizontal_dir = d,
			None => {}
		}
		
		let (last_x, last_y): (i32, i32) = self.head_pos();
		
		let new_ver = match self.vertical_dir {
            BallDir::Up => last_y - 1,
           
            BallDir::Down => last_y + 1,
            
            _ => last_y
        };
        
        let new_hor = match self.horizontal_dir{
        	BallDir::Left => last_x - 1,
                
            BallDir::Right => last_x + 1,
            
            _ => last_x
        };
        
        let new_block = Block{
        	x: new_hor,
        	y: new_ver
        };
        self.ball_block.push_front(new_block);
        self.ball_block.pop_back().unwrap();
		 
	}
	
	pub fn next_head_pos(&self, ver_dir: Option<BallDir>, hor_dir: Option<BallDir>) -> (i32,i32){
		let (head_x, head_y) = self.head_pos();
		
		let mut hor_direction = self.horizontal_dir;
		let mut ver_direction = self.vertical_dir;
		match ver_dir {
			Some(d) => ver_direction = d,
			None => {}
		}
		
		match hor_dir{
			Some(d) => hor_direction = d,
			None => {}
		}
		
		let new_ball_x = match hor_direction{
			BallDir::Left => head_x - 1,
			BallDir::Right => head_x + 1,
			_ => head_x
		};
		
		let new_ball_y = match ver_direction{
			BallDir::Up => head_y - 1,
			BallDir::Down => head_y + 1,
			_=> head_y
		};
		
		(new_ball_x, new_ball_y)
	}
	
	pub fn head_pos(&self)->(i32,i32){
		let pos = self.ball_block.front().unwrap();
		(pos.x, pos.y)
	}
	
	pub fn vertical_direction(&self)->BallDir{
		self.vertical_dir
	}
	
	pub fn horizontal_direction(&self)->BallDir{
		self.horizontal_dir
	}
}
