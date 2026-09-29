use crate::material::Material;
use crate::math::{Color, Vec3};

#[derive(Clone, Copy)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: usize,
    pub smooth_normal: Option<Vec3>,
}

#[derive(Clone, Copy)]
pub struct Ellipsoid {
    pub center: Vec3,
    pub radii: Vec3,
    pub material: usize,
}

#[derive(Clone, Copy)]
pub struct Capsule {
    pub start: Vec3,
    pub end: Vec3,
    pub radius: f32,
    pub material: usize,
}

pub struct Scene {
    pub cubes: Vec<Cube>,
    pub ellipsoids: Vec<Ellipsoid>,
    pub capsules: Vec<Capsule>,
    pub materials: Vec<Material>,
    pub light_dir: Vec3,
    pub light_color: Color,
}
