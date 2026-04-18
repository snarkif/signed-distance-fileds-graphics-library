use glam::{vec2, vec3};
use crate::shared::{screen_buffer};
use crate::three_d::{ray,shapes};
use crate::three_d::camera::Camera;
use crate::three_d::shapes::*;
use crate::three_d::ray::*;


const MAX_DISTANCE: f32 =1500.0;
const ROWS: usize = 64;
const COLUMNS: usize = 128;
const ALPHA: f32 = 0.01;
pub(crate) struct Space {
    space: Vec<Shape>,
    pub(crate) camera: Camera,
}
impl Space{
    pub fn new() -> Self {
        Self{space: Vec::new(),camera: Camera::new(vec3(0.0,3.0,0.0))}
    }
    pub fn add(&mut self, shape: Shape)->i32 {
        self.space.push(shape);
        self.space.len() as i32//this functions as a handle
    }

    pub fn borrow(&mut self, index:i32)->&mut Shape{
        &mut self.space[index as usize]
    }

    pub fn remove(&mut self,index:i32) {
        self.space.remove(index as usize);
    }
    fn march(&self,ray: Ray)->bool{
        let mut position =vec3(ray.origin.x, ray.origin.y, ray.origin.z);
        let mut min =MAX_DISTANCE;
        let mut distance_traveled =0.0;
        for i in 0..150 {//repeat 150 times tops, to not consume to much computation power
            if distance_traveled>MAX_DISTANCE {//to save computation power
                break;
            }
            for shape in self.space.iter() {
                let dist = shape.sdf(position);
                if (dist < min) {
                    min = dist;
                }
            }
            if(min<ALPHA){
                return true;//if you're close enough to the object return true(collision)
            }
            position=position+ray.direction*min;//march the ray
            distance_traveled+=min;
            min=MAX_DISTANCE;
        }
        false
    }
    pub(crate) fn render(&self, arr:&mut [bool]){

        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let ray=self.camera.create_ray(vec2(col as f32,row as f32));
                if(self.march(ray)){
                    screen_buffer::ScreenBuffer::write_pixel_generic(arr, col, row);
                }
            }
        }
    }
}