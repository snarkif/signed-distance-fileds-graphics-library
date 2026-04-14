
use glam::{Vec3, vec3};

const ROWS: i32 = 128;
const COLUMNS: i32 =64;
const ASPECT: f32 = ROWS/COLUMN;

struct ray{
    direction::Vec3,
    start::Vec3,
}

impl ray{
    pub fn march(&self)->bool{
        let pos=vec3(*&self.start.x,*&self.start.y,*&self.start.z);
        let distance_traveled: f32 = 0.0;
        

    }
}

Ray createRay(vec2 uv) {
    Ray ray;
    float uAspect = resolution.x / resolution.y; // Moved calculation here
    vec2 st = uv * 2.0 - 1.0;
    st.x *= uAspect;
    ray.direction = normalize(uForward + uRight * st.x + uUp * st.y).normalize();
    ray.start = uPos;
    ray.distance_traveled = 0.;
    ray.hits = 0;
    return ray;
  }

fn createRay(row:i32,column:i32)->ray{
    
    let clip=vec2(row/ROWS,column/COLUMNS)*2.0-1.0;//mapped to -1 to 1
    clip.x=clip*ASPECT;
    //camera vectors
    let forward=vec3(0.0,0.0,1.0);
    let right=vec3(1.0,0.0,0.0);
    let up=vec3(0.0,1.0,0.0);

    ray{direction:vec3(forward+right*clip.x+up*clip.y).normalize(),start:vec3(0.0,0.0,0.0)}
}

fn main() {
    for row in 0..ROWS{
        for column in 0..COLUMNS{

        }
    }
    println!("Hello, world!");
}
