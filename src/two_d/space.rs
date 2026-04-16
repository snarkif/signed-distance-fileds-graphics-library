use glam::vec2;
use crate::{COLUMNS, ROWS};
use crate::two_d::{shape};
use crate::shared::{screen_buffer};

pub struct Space{
    space: Vec<shape::Shape>,
}

impl Space{
    pub fn new() -> Self {
        Self{space: Vec::new()}
    }
    pub fn add(&mut self, shape: shape::Shape)->i32 {
        self.space.push(shape);
        return  self.space.len() as i32;
    }

    pub fn borrow(&mut self, index:i32)->&mut shape::Shape{
        &mut self.space[index as usize] 
    }

    pub fn remove(&mut self,index:i32) {
        self.space.remove(index as usize);
    }

    pub fn render(&mut self,arr:&mut [bool]) {
        for col in 0..COLUMNS {       // 0 TO 128
            for row in 0..ROWS {      // 0 to 64
                let uv = vec2(
                    col as f32 / ROWS as f32,
                    row as f32 / ROWS as f32,
                );
                for shape in self.space.iter() {
                    if shape.sdf(uv) < 0.0 {
                        screen_buffer::ScreenBuffer::write_pixel_generic(arr, col, row);
                        break;
                    }
                }
            }
        }
    }


}

