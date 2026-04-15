use glam::{Vec2, vec2};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};


const ROWS: usize = 64;
const COLUMNS: usize = 128;
const ASPECT_RATIO: usize = 2;

struct FileHandler {
    screen: Vec<bool>,
}
impl FileHandler {
    fn new() -> Self {
        Self {
            screen: vec![false; ROWS * COLUMNS],
        }
    }
    fn write_pixel(&mut self, x: usize, y: usize) {
        let page=y/8;
        self.screen[(page*1024)+(8*x)+(y-8*page)] = true;//updated
    }
    fn write_pixel_generic(arr:&mut [bool], x: usize, y: usize) {
        let page=y/8;
        arr[(page*1024)+(8*x)+(y-8*page)] = true;//updated
    }
    fn clear(&mut self ) {
        for i in 0..self.screen.len() {
            self.screen[i] = false;
        }
    }

    fn print_arr(&self) {
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let page=row/8;
                let index =  (page * 1024) + (8 * col) + (row - 8 * page);//using the updated writing
                if(self.screen[index]){
                    print!("1");
                }
                else{
                    print!("0");
                }
            }
            println!();
        }
        let _ = io::stdout().flush();
    }

    fn pack_bits_to_bytes(&self) -> Vec<u8> {
        self.screen.chunks(8)
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

}

enum ShapeType {
    Circle(f32),
    Triangle(Vec2,Vec2,Vec2),
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

            ShapeType::Triangle(p0,p1,p2) => {
                let e0 = p1 - p0;
                let e1 = p2 - p1;
                let e2 = p0 - p2;
                let v0 = pixel - p0;
                let v1 = pixel - p1;
                let v2 = pixel - p2;

                let pq0 = v0 - e0 * (v0.dot(e0) / e0.dot(e0)).clamp(0.0, 1.0);
                let pq1 = v1 - e1 * (v1.dot(e1) / e1.dot(e1)).clamp(0.0, 1.0);
                let pq2 = v2 - e2 * (v2.dot(e2) / e2.dot(e2)).clamp(0.0, 1.0);

                let s = (e0.x * e2.y - e0.y * e2.x).signum();

                let d = vec2(pq0.dot(pq0), s * (v0.x * e0.y - v0.y * e0.x))
                    .min(vec2(pq1.dot(pq1), s * (v1.x * e1.y - v1.y * e1.x)))
                    .min(vec2(pq2.dot(pq2), s * (v2.x * e2.y - v2.y * e2.x)));

                -d.x.sqrt() * d.y.signum()
            },

        }
    }
}

struct Space{
    space: Vec<Shape>,
}

impl Space{
    fn new() -> Self {
        Self{space: Vec::new()}
    }
    fn add(&mut self, shape: Shape)->i32 {
        self.space.push(shape);
        return  self.space.len() as i32;
    }

    fn update(&mut self, index:i32, shape:Shape){
        self.space[index as usize] = shape;
    }

    fn remove(&mut self,index:i32) {
        self.space.remove(index as usize);
    }

    fn render(&mut self,arr:&mut [bool]) {
        for col in 0..COLUMNS {       // 0 TO 128
            for row in 0..ROWS {      // 0 to 64
                let uv = vec2(
                    col as f32 / ROWS as f32,
                    row as f32 / ROWS as f32,
                );
                for shape in self.space.iter() {
                    if shape.sdf(uv) < 0.0 {
                        FileHandler::write_pixel_generic(arr, col, row);
                        break;
                    }
                }
            }
        }
    }


}



fn main() -> std::io::Result<()> {

    let mut space = Space::new();

    let mut filehandler = FileHandler::new();



    let circle=space.add(Shape {
        shape_type: ShapeType::Circle(0.1),
        position: vec2(0.5, 0.5), // centered
    });

    space.add(Shape {
        shape_type: ShapeType::Triangle(vec2(0.7,0.8),vec2(0.8,0.9),vec2(0.6,0.8)),
        position: vec2(0.5, 0.5), // centered
    });

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("oled_output.txt")?;

    let mut r=0.05;
    let mut delta=0.05;

    loop {
        if(r>0.5||r<0.05){
            delta*=-1.0;
        }
        r+=delta;
        space.update(circle,Shape{
            shape_type:ShapeType::Circle(r),
            position:vec2(0.5,0.5),
        });
        file.rewind()?;
        filehandler.clear();
        space.render( &mut filehandler.screen);
        filehandler.print_arr();

        let packed_data = filehandler.pack_bits_to_bytes();
        file.write_all(&packed_data)?;
        file.flush()?;
        thread::sleep(Duration::from_millis(1000));

    }

}
