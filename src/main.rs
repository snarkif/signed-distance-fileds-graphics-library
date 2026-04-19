use glam::{Vec2, vec2, vec3};
//use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use minifb::{Window, WindowOptions};
use three_d::shapes::{Shape, ShapeType, Sphere};

const DEVICE_FILE_PATH: &str ="oled_output.txt";//todo change to /dev/oled
const ROWS: usize = 64;
const COLUMNS: usize = 128;
const ASPECT_RATIO: usize = 2;

//mod two_d;
mod shared;
mod three_d;

use shared::screen_buffer;
use shared::mouse;
//use two_d::shape::*;
//use two_d::space;
use three_d::space::*;
use three_d::shapes;
use crate::three_d::shapes::BoxFrame;

fn add_stuff_3d(space: &mut three_d::space::Space) ->Vec<i32>{
    let mut shape_handles =Vec::<i32>::new();
    shape_handles.push(space.add(
        Shape::new(ShapeType::BoxFrame(BoxFrame::new(vec3(0.0,0.0,1.0),vec3(0.5,0.5,0.5),0.02)))
    ));
    shape_handles.push(space.add(
        Shape::new(ShapeType::Sphere(Sphere::new(vec3(0.0,0.0,1.0),0.5)))
    ));



    shape_handles


}

fn main() -> std::io::Result<()> {
//2D TESTING
    let mut space = Space::new();
    let mut mouse =mouse::Mouse::new();
    let shape_handles=add_stuff_3d(&mut space);

    let mut screen = screen_buffer::ScreenBuffer::new();//todo transfer to heap
    let mut window = Window::new("oled_emulator", COLUMNS, ROWS, WindowOptions::default()).unwrap();


//todo file operations commented out for now , use window to emulate/debug
//     let mut file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open(DEVICE_FILE_PATH)?;//do not change!


    loop {
        space.camera.rotate_with_mouse(&mut mouse);
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