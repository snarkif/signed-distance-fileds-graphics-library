use std::ops::Mul;
use device_query::Keycode;
use glam::{vec2, vec3, Vec2, Vec3, Quat};
use crate::{COLUMNS, ROWS};
use crate::three_d::ray::Ray;
use crate::shared::input_devices;

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
    pub fn rotate_with_mouse(&mut self, mouse: &mut input_devices::Devices) {
        let delta=mouse.get_delta();
        self.rotate_horizontaly(delta.x /25.0);

        self.rotate_verticaly(delta.y /25.0);
    }
    pub fn move_forward(&mut self){
        self.position = self.position+self.forward.mul(0.01);
    }
    pub fn move_backwards(&mut self){
        self.position = self.position-self.forward.mul(0.01);
    }
    pub fn move_right(&mut self){
        self.position = self.position+self.forward.cross(self.up).normalize().mul(0.01);
    }
    pub fn move_left(&mut self){
        self.position = self.position-self.forward.cross(self.up).normalize().mul(0.01);
    }
    pub fn move_up(&mut self){
        self.position = self.position+self.up.mul(0.01);
    }
    pub fn move_down(&mut self){
        self.position = self.position-self.up.mul(0.01);
    }
    pub fn move_with_keyboard(&mut self, keyboard: &mut input_devices::Devices) {
        let keys=keyboard.keyboard_liten();
        if(keys.contains(&Keycode::A)){
            self.move_left();
        }
        if(keys.contains(&Keycode::S)){
            self.move_backwards();
        }
        if(keys.contains(&Keycode::W)){
            self.move_forward();
        }
        if(keys.contains(&Keycode::D)){
            self.move_right();
        }
        if(keys.contains(&Keycode::Space)){
            self.move_up();
        }
        if(keys.contains(&Keycode::LControl)){
            self.move_down();
        }
    }
}