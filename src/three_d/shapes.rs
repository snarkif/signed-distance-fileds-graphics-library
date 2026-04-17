use glam::{Vec2, Vec3, vec3};

pub trait Sdf {
    fn sdf(&self, point: Vec3) -> f32;
}

///////////shape structures start
//shpere start
pub struct Sphere {
    center: Vec3,
    radius: f32,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}
impl Sdf for Sphere {
    fn sdf(&self, point: Vec3) -> f32 {
        (point - self.center).length() - self.radius
    }
}
//boxFrame start
pub struct BoxFrame {
    center: Vec3,
    half_extents: Vec3,
    thickness: f32,
}
impl BoxFrame {
    pub fn new(center: Vec3, half_extents: Vec3, thickness: f32) -> Self {
        Self { center, half_extents, thickness }
    }
}
impl Sdf for BoxFrame {
    fn sdf(&self, point: Vec3) -> f32 {
        let p = (point - self.center).abs() - self.half_extents;
        let q = (p + self.thickness).abs() - self.thickness;

        let a = Vec3::new(p.x, q.y, q.z).max(Vec3::ZERO).length() + p.x.max(q.y.max(q.z)).min(0.0);
        let b = Vec3::new(q.x, p.y, q.z).max(Vec3::ZERO).length() + q.x.max(p.y.max(q.z)).min(0.0);
        let c = Vec3::new(q.x, q.y, p.z).max(Vec3::ZERO).length() + q.x.max(q.y.max(p.z)).min(0.0);

        a.min(b).min(c)
    }
}

////////////shape structures end
pub enum ShapeType {
    Sphere(Sphere),
    BoxFrame(BoxFrame),
}
pub struct Shape {
    pub shape_type: ShapeType,
}
impl Shape {
    pub fn new(shape_type: ShapeType) -> Self {
        Self { shape_type }
    }
}
impl Sdf for Shape {
    fn sdf(&self, point: Vec3) -> f32 {
        match &self.shape_type {
            ShapeType::Sphere(sphere) => sphere.sdf(point),
            ShapeType::BoxFrame(bf) => bf.sdf(point),
        }
    }
}