use crate::material::Material;
use crate::math::{Color, Vec3};

#[derive(Clone, Copy)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: usize,
}

pub struct Scene {
    pub cubes: Vec<Cube>,
    pub materials: Vec<Material>,
    pub light_dir: Vec3,
    pub light_color: Color,
}
