use glam::{vec2, vec3};
use crate::shared::{screen_buffer};
use crate::three_d::{ray,shapes};
use crate::three_d::shapes::*;
use crate::three_d::ray::*;

const MAX_DISTANCE: f32 =1500.0;
const ROWS: usize = 64;
const COLUMNS: usize = 128;
const ALPHA: f32 = 0.01;
struct Space {
    space: Vec<shapes::Sphere>,
}
impl Space{
    fn new() -> Space {
        Space {
            space: Vec::new(),
        }
    }
    fn march(&self,ray: &ray::Ray)->bool{
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
    fn render(&self,arr:&mut [bool]){

        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let ray=Ray::create_ray(vec2(row as f32,col as f32));
                if(self.march(&ray)){
                    screen_buffer.write_pixel_generic(arr,row,col);
                }
            }
        }
    }
}