use glam::{Vec2, vec2};
//use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use minifb::{Window, WindowOptions};
const DEVICE_FILE_PATH: &str ="oled_output.txt";//todo change to /dev/oled
const ROWS: usize = 64;
const COLUMNS: usize = 128;
const ASPECT_RATIO: usize = 2;

mod two_d;
mod shared;

use shared::screen_buffer;
use two_d::shape::*;
use two_d::space;


fn add_stuff(space: &mut space::Space)->Vec<i32>{
    let mut shape_handles =Vec::<i32>::new();
    shape_handles.push(space.add(Shape {//todo move these space.add calls to a seperate function! this is bloating main
        shape_type: ShapeType::Circle(Circle::new(vec2(0.5,0.5),0.3)),

    }));

    shape_handles.push(space.add(Shape {
        shape_type: ShapeType::Triangle(vec2(0.7,0.8),vec2(0.8,0.9),vec2(0.6,0.8)),

    }));
    shape_handles
}



fn main() -> std::io::Result<()> {

    let mut space = space::Space::new();//todo transfer to heap//vectors are stored in the heap corny ahh

    let shape_handles=add_stuff(&mut space);

    let mut screen = screen_buffer::ScreenBuffer::new();//todo transfer to heap
    let mut window = Window::new("oled_emulator", COLUMNS, ROWS, WindowOptions::default()).unwrap();


//todo file operations commented out for now , use window to emulate/debug
//     let mut file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open(DEVICE_FILE_PATH)?;//do not change!

    let mut r=0.05;
    let mut delta=0.05;
    loop {

        let obj=space.borrow(0);
        if let ShapeType::Circle(circle) = &mut obj.shape_type {
            circle.center = vec2(0.5, 0.5);
            circle.radius = r;
        }
       

        if(r>0.5||r<0.05){
            delta*=-1.0;
        }
        r+=delta;
        screen.clear();
        space.render(&mut screen.screen);


        let packed_data = screen.pack_bits_to_bytes();

        let window_buffer = screen_buffer::ScreenBuffer::buf_to_window_vec(packed_data.as_slice());
        window.update_with_buffer(window_buffer.as_ref(), COLUMNS, ROWS).unwrap();

        // file.write_all(&packed_data)?;
        // file.flush()?;
        thread::sleep(Duration::from_millis(100));

    }

}