use std::ops::Mul;
use glam::{vec2, vec3, Vec2, Vec3};
use crate::{COLUMNS, ROWS};
use crate::three_d::ray::Ray;
const FOV: f32 =90.0;
pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,      // where it is looking at
    pub up: Vec3,          // world up (0,1,0)
    delta_x: f32//temporary, for the proof of concept
}

impl Camera {
    pub(crate) fn new(position: Vec3 ) -> Camera {
        Self{position, forward: vec3(0.0,0.0,0.0)-position, up: vec3(1.0,1.0,0.0),delta_x:0.5}
    }
    pub fn create_ray(&self, pixel: Vec2) -> Ray {
        // build camera axes from look direction
        let forward = self.forward.mul(1.0 / (FOV / 2.0).tan());
        let right = forward.cross(self.up).normalize();
        let up = right.cross(forward).normalize();

        let mut uv = vec2(
            pixel.x / COLUMNS as f32,
            pixel.y / ROWS as f32,
        ) * 2.0 - 1.0;
        uv.x *= 2.0; // aspect ratio

        Ray {
            origin: self.position,
            direction: (forward + right * uv.x + up * uv.y).normalize(),
        }
    }

    fn update_position(&mut self,trans:Vec3){//update the camera position
        self.position=self.position+trans;
    }

    fn target_middle(&mut self)  {//use this untill i add rotation with the mouse
        self.forward=vec3(0.0,0.0,0.0)-self.position;
    }

    pub(crate) fn do_both(&mut self){//moves the camera around
        if self.position.x.abs()>=3.0{
            self.delta_x*=-1.0;

        }
        self.update_position(vec3(self.delta_x,0.0,0.0));
        self.target_middle();
    }
}