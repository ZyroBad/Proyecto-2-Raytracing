use crate::math::{Color, Vec3};

#[derive(Clone, Copy)]
pub enum MaterialKind {
    Rock,
    Cloud,
    GamabuntaSkin,
    GamakenSkin,
    GamahiroSkin,
    Robe,
    OrangeCloth,
    RedCloak,
    NarutoSkin,
    NarutoHair,
    Ink,
    Metal,
    Chakra,
    Wood,
    EyeGold,
    Belly,
    Rope,
    CraterEarth,
    RuinStone,
    RoofTile,
    DustSmoke,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub kind: MaterialKind,
    pub albedo: Color,
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub refractive_index: f32,
}

impl Material {
    pub fn texture(self, p: Vec3, normal: Vec3) -> Color {
        let checker = ((p.x.floor() as i32 + p.z.floor() as i32 + p.y.floor() as i32) & 1) as f32;
        match self.kind {
            MaterialKind::Rock => {
                let cracks = stripe(p.x * 0.55 + p.z * 0.9, 0.45) * 0.16;
                self.albedo * (0.72 + noise(p * 2.8) * 0.28 - cracks)
            }
            MaterialKind::Cloud => {
                let soft = noise(p * 1.7) * 0.10 + if normal.y > 0.5 { 0.08 } else { 0.0 };
                self.albedo * (0.88 + soft)
            }
            MaterialKind::GamabuntaSkin => {
                let spots = stripe(p.x * 0.8 + p.y * 0.35, 1.6) * noise(p * 3.0) * 0.20;
                self.albedo * (0.82 + noise(p * 4.0) * 0.16 - spots)
            }
            MaterialKind::GamakenSkin => {
                let mottled = noise(p * 4.8) * 0.18 + checker * 0.04;
                self.albedo * (0.78 + mottled)
            }
            MaterialKind::GamahiroSkin => {
                let bands = stripe(p.y + p.x * 0.12, 1.8) * 0.08;
                self.albedo * (0.84 + noise(p * 3.2) * 0.12 - bands)
            }
            MaterialKind::Robe => {
                let weave = checker * 0.05 + stripe(p.y, 4.0) * 0.04;
                self.albedo * (0.78 + weave)
            }
            MaterialKind::OrangeCloth => {
                let weave = noise(p * 10.0) * 0.10;
                self.albedo * (0.86 + weave)
            }
            MaterialKind::RedCloak => {
                let fold = stripe(p.x + p.y * 0.2, 1.7) * 0.10;
                self.albedo * (0.78 + noise(p * 5.0) * 0.10 + fold)
            }
            MaterialKind::NarutoSkin => self.albedo * (0.90 + noise(p * 8.0) * 0.08),
            MaterialKind::NarutoHair => {
                let highlight = stripe(p.x - p.y, 3.8) * 0.18;
                self.albedo * (0.82 + highlight)
            }
            MaterialKind::Ink => self.albedo * (0.78 + noise(p * 7.0) * 0.12),
            MaterialKind::Metal => {
                let scratch = stripe(p.y + p.x * 0.3, 7.0) * 0.16;
                self.albedo * (0.74 + scratch)
            }
            MaterialKind::Chakra => {
                let pulse = (p.x * 5.0 + p.y * 3.0 + p.z * 4.0).sin() * 0.08;
                Color::new(0.20 + pulse, 0.58 + pulse, 0.96)
            }
            MaterialKind::Wood => {
                let grain = ((p.x * 8.0).sin() + (p.y * 11.0).sin()) * 0.06;
                self.albedo * (0.80 + grain + checker * 0.06)
            }
            MaterialKind::EyeGold => {
                let glow = if normal.z > 0.5 { 0.18 } else { 0.02 };
                self.albedo * (0.82 + glow + noise(p * 9.0) * 0.05)
            }
            MaterialKind::Belly => {
                let pores = noise(p * 6.0) * 0.10;
                let folds = stripe(p.y + p.x * 0.08, 2.8) * 0.06;
                self.albedo * (0.84 + pores - folds)
            }
            MaterialKind::Rope => {
                let twist = stripe(p.y + p.x * 0.8, 5.0) * 0.14;
                self.albedo * (0.78 + twist + noise(p * 7.0) * 0.06)
            }
            MaterialKind::CraterEarth => {
                let grit = noise(p * 7.5) * 0.20;
                let crack_a = stripe(p.x * 0.75 + p.z * 1.2, 1.7) * 0.14;
                let crack_b = stripe(p.x * 1.1 - p.z * 0.55, 2.2) * 0.10;
                self.albedo * (0.72 + grit - crack_a - crack_b)
            }
            MaterialKind::RuinStone => {
                let chips = noise(p * 5.5) * 0.22;
                let seams = (stripe(p.x, 1.8) + stripe(p.y, 2.4)) * 0.07;
                self.albedo * (0.70 + chips - seams)
            }
            MaterialKind::RoofTile => {
                let rows = stripe(p.y + p.z * 0.15, 4.5) * 0.16;
                let soot = noise(p * 4.0) * 0.18;
                self.albedo * (0.74 + rows - soot)
            }
            MaterialKind::DustSmoke => {
                let billow = noise(p * 1.8) * 0.20;
                self.albedo * (0.72 + billow)
            }
        }
        .clamp01()
    }
}

pub fn noise(p: Vec3) -> f32 {
    let n = (p.x * 12.9898 + p.y * 78.233 + p.z * 37.719).sin() * 43_758.547;
    n.fract().abs()
}

fn stripe(v: f32, frequency: f32) -> f32 {
    if (v * frequency).sin() > 0.72 {
        1.0
    } else {
        0.0
    }
}
