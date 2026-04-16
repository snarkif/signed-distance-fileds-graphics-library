use glam::{Vec2, Vec3};
use crate::two_d::shape::Circle;

pub trait Sdf {
    fn sdf(&self,point: Vec3)->f32;
}
///////////shape structures start
pub struct Sphere {
    pos : Vec3,
    radius : f32,
}
impl Sphere{
    pub fn new(pos:Vec3, radius:f32)->Self{
        Self { pos, radius }
    }
}
impl Sdf for Sphere{
    fn sdf(&self, point: Vec3)->f32{
        self.radius
    }
}

////////////shape structures end
pub enum ShapeType {
    Shpere(Sphere),
    //add more
}
pub struct Shape {
    pub shape_type:ShapeType ,
}
impl Shape {
    pub fn new(shape_type:ShapeType)->Self{
        Self{shape_type}
    }
}
