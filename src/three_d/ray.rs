const FOV: f32 =90.0;
use trig::Trig;
use glam::{Vec3, vec3, Vec2, vec2};
const ROWS: i32 =64;
const COLUMNS: i32 =128;
pub(crate) struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }
    pub fn create_ray(pixel: Vec2) -> Ray {//not dependent on the camera yet
        let forward = Vec3::new(0.0, 0.0, 1.0 / (FOV / 2.0).tan());
        let up = Vec3::new(0.0,1.0,0.0);
        let right = Vec3::new(1.0,0.0,0.0);
        let mut uv = vec2(
            pixel.x / COLUMNS as f32,
            pixel.y / ROWS as f32,
        ) * 2.0 - 1.0;
       uv.x*=2.0;
        Ray {
            origin: vec3(0.0,0.0,0.0),
            direction: (forward+right*uv.x+up*uv.y).normalize(),
        }
    }
}