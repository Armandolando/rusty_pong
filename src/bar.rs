use std::collections::LinkedList;

use piston_window::Context;
use piston_window::G2d;
use piston_window::types::Color;

use draw::draw_block;

const S_COLOR : Color = [0.20, 0.70, 0.30, 1.0];

#[derive(Clone, Copy, PartialEq)]
pub enum Direction{
	Up,
	Down
}

#[derive(Debug, Clone)]
pub struct Block{
	pub x:i32,
	pub y:i32
}

pub struct Bar{
	body: LinkedList<Block>
}

impl Bar{
	pub fn new(init_x:i32, init_y:i32)->Bar{
		let mut body : LinkedList<Block> = LinkedList::new();
		
		body.push_back(Block{
			x: init_x,
			y: init_y
		});
		
		body.push_back(Block{
			x: init_x,
			y: init_y + 1
		});
		
		body.push_back(Block{
			x: init_x,
			y: init_y + 2
		});
		
		Bar{
			body
		}
	}
	
	pub fn draw(&self, con: &Context, g: &mut G2d){
		for block in &self.body{
			draw_block(S_COLOR, block.x, block.y, con, g);
		}
	}
	
	pub fn move_bar(&mut self, dir: Option<Direction>){
		
		let direction = match dir{
			Some(d) => d,
			None => Direction::Down
		};
		
		let new_block = match direction {
            Direction::Up => {
            	
            	let (last_x, last_y): (i32, i32) = self.head_pos();
            	
            	Block {
                	x: last_x,
                	y: last_y - 1
            	}
            }
            Direction::Down => {
            		
            	let (last_x, last_y): (i32, i32) = self.tail_pos();
            		
            	Block {
                	x: last_x,
                	y: last_y + 1
            	}
            },
            
        };
        
        match dir {
            Some(Direction::Up) => {
                self.body.push_front(new_block);
                self.body.pop_back().unwrap();
            },
            Some(Direction::Down) => {
                self.body.push_back(new_block);
                self.body.pop_front().unwrap();
            },
            None => {}
        };
        
        
	}
	
	pub fn next_pos(&self, dir: Option<Direction>) -> (i32,i32){
		
		let (_head_x, head_y) = self.head_pos();
		let (_tail_x, tail_y) = self.tail_pos();
		let (head, tail) = (head_y, tail_y);
		
		let direction = match dir{
			Some(d) => d,
			None => Direction::Down
		};
		
		match direction {
            Direction::Up => (head - 1, tail - 1),
            Direction::Down => (head + 1, tail + 1)
        }
		
	}
	
	pub fn head_pos(&self)->(i32,i32){
		let pos = self.body.front().unwrap();
		(pos.x, pos.y)
	}
	
	pub fn tail_pos(&self)->(i32,i32){
		let pos = self.body.back().unwrap();
		(pos.x, pos.y)
	}
	
	pub fn bounce(&self, x: i32, y: i32)->bool{
		for block in &self.body{
			if x==block.x && y==block.y + 1 || x==block.x && y==block.y - 1 {
				return false;
			}
		}
		
		true
	}
}
