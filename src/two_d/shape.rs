//includes all the sdfs
use glam::{Vec2, vec2};
//s
const ROWS: usize = 64;
const COLUMNS: usize = 128;
const ASPECT_RATIO: usize = 2;

pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}
impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Circle{center, radius}
    }
}

pub enum ShapeType {
    Circle(Circle),
    Triangle(Vec2,Vec2,Vec2),
    //Rectangle(f32, f32),
}
pub struct Shape {
    pub shape_type: ShapeType,

}

impl Shape {

    pub fn sdf(&self, pixel: Vec2) -> f32 {
        match &self.shape_type {
            ShapeType::Circle(circle) => (circle.center - pixel).length() - circle.radius,

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