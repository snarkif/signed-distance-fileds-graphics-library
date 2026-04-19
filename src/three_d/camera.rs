use std::ops::Mul;
use glam::{vec2, vec3, Vec2, Vec3,Quat};
use crate::{COLUMNS, ROWS};
use crate::three_d::ray::Ray;
use crate::shared::mouse;
const FOV: f32 =90.0;
pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,      // where it is looking at
    pub up: Vec3,          // world up (0,1,0)

}

impl Camera {
    pub(crate) fn new(position: Vec3 ) -> Camera {
        Self{position, forward: vec3(0.0,0.0,0.0)-position, up: vec3(1.0,1.0,0.0)}
    }
    pub fn create_ray(&self, pixel: Vec2) -> Ray {
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


    pub fn rotate_horizontaly(&mut self, mut angle: f32) {
        angle=angle.to_radians();
        let right = self.forward.cross(self.up).normalize().mul(-1.0);
        let axis = right.cross(self.forward).normalize();//the local up vector
        let q = Quat::from_axis_angle(axis,angle);
        self.forward=q*self.forward;
    }
    pub fn rotate_verticaly(&mut self, mut angle: f32) {
        angle=angle.to_radians();
        let axis = self.forward.cross(self.up).normalize();
        let q = Quat::from_axis_angle(axis,angle);
        self.forward=q*self.forward;
    }
    pub fn rotate_with_mouse(&mut self, mouse: &mut mouse::Mouse) {
        let delta=mouse.get_delta();
        self.rotate_horizontaly(delta.x /50.0);

        self.rotate_verticaly(delta.y /50.0);
    }
}