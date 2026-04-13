use glam::{Vec2, vec2};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{};
use std::thread;
use std::time::Duration;


const ROWS: usize = 64;
const COLUMNS: usize = 128;

enum ShapeType {
    Circle(f32),
    //Rectangle(f32, f32),
}

struct Shape {
    shape_type: ShapeType, 
    position: Vec2,
}

impl Shape {
  
    fn sdf(&self, pixel: Vec2) -> f32 {
        match self.shape_type {
            ShapeType::Circle(radius) => (self.position - pixel).length() - radius,
            
        }
    }
}


fn write_pixel(arr: &mut [bool], x: usize, y: usize) {
    arr[y * COLUMNS + x] = true;
}

fn render(space: &Vec<Shape>, arr: &mut [bool]) {
    for y in 0..ROWS {
        for x in 0..COLUMNS {
          
            let pixel_pos = vec2(x as f32, y as f32);
            for shape in space.iter() {
                if shape.sdf(pixel_pos) < 0.0 {
                    write_pixel(arr, x*2, y);
                    break;
                }
            }
        }
    }
}

fn clear(arr: &mut [bool]) {
    for i in 0..arr.len() {
        arr[i] = false;
    }
}

fn pack_bits_to_bytes(grid: &[bool]) -> Vec<u8> {
    grid.chunks(8)
        .map(|chunk| {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit {
                    byte |= 1 << i;
                }
            }
            byte
        })
        .collect()
}

use std::io::{self, Write};

fn printArr(arr: &[bool]) {
    for rows in 0..ROWS {
        for columns in 0..COLUMNS {
            
            let index = rows * COLUMNS + columns;
            
            if arr[index] {
                print!("1");
            } else {
                print!("0"); 
            }
        }
       
        println!(); 
    }
   
    let _ = io::stdout().flush();
}

fn main() -> std::io::Result<()> {
    let mut space = Vec::<Shape>::new();
    
    
    let mut screen = vec![false; ROWS * COLUMNS];

    space.push(Shape {
        shape_type: ShapeType::Circle(10.0),
        position: vec2(30.0, 30.0),
    });

        let mut file = OpenOptions::new()
        .write(true)
        .create(true) 
        .truncate(true) 
        .open("oled_output.txt")?; 

    loop {
        file.rewind()?;
        clear(&mut screen);
        render(&space, &mut screen);
        printArr(&mut screen);
        
        let packed_data = pack_bits_to_bytes(&screen);
        file.write_all(&packed_data)?;  
        file.flush()?; 
        thread::sleep(Duration::from_millis(1000));

    }

}
