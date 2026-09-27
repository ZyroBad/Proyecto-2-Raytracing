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
    CarvedStone,
    GamahiroMarking,
    GamakichiSkin,
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
                let pattern = scale_pattern(p, 2.2) * 0.18;
                let spots = stripe(p.x * 0.8 + p.y * 0.35, 1.6) * noise(p * 3.0) * 0.12;
                self.albedo * (0.84 + noise(p * 4.0) * 0.12 - spots - pattern)
            }
            MaterialKind::GamakenSkin => {
                let scales = scale_pattern(p + Vec3::new(0.3, 0.0, 0.0), 2.8) * 0.15;
                let mottled = noise(p * 4.8) * 0.15 + checker * 0.035;
                self.albedo * (0.81 + mottled - scales)
            }
            MaterialKind::GamahiroSkin => {
                let bands = stripe(p.y + p.x * 0.12, 1.8) * 0.08;
                let scales = scale_pattern(p + Vec3::new(0.6, 0.0, 0.0), 2.5) * 0.13;
                self.albedo * (0.88 + noise(p * 3.2) * 0.10 - bands - scales)
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
            MaterialKind::CarvedStone => {
                let erosion = noise(p * 5.2) * 0.16;
                let veins = stripe(p.x * 0.65 + p.y * 0.22, 2.6) * 0.08;
                self.albedo * (0.80 + erosion - veins)
            }
            MaterialKind::GamahiroMarking => {
                let mottled = noise(p * 6.4) * 0.12;
                self.albedo * (0.82 + mottled)
            }
            MaterialKind::GamakichiSkin => {
                let scales = scale_pattern(p + Vec3::new(0.15, 0.0, 0.0), 3.8) * 0.16;
                let freckles = noise(p * 7.0) * 0.10;
                self.albedo * (0.90 + freckles - scales)
            }
        }
        .clamp01()
    }
}

pub fn scene_materials() -> Vec<Material> {
    vec![
        material(MaterialKind::Rock, [0.43, 0.33, 0.23], 0.12, 0.0, 0.04, 1.0),
        material(
            MaterialKind::Cloud,
            [0.88, 0.92, 0.98],
            0.32,
            0.06,
            0.08,
            1.05,
        ),
        material(
            MaterialKind::GamabuntaSkin,
            [0.56, 0.18, 0.13],
            0.24,
            0.0,
            0.04,
            1.0,
        ),
        material(
            MaterialKind::GamakenSkin,
            [0.62, 0.16, 0.40],
            0.22,
            0.0,
            0.03,
            1.0,
        ),
        material(
            MaterialKind::GamahiroSkin,
            [0.42, 0.72, 0.70],
            0.30,
            0.0,
            0.05,
            1.0,
        ),
        material(MaterialKind::Robe, [0.07, 0.10, 0.16], 0.16, 0.0, 0.05, 1.0),
        material(
            MaterialKind::OrangeCloth,
            [0.93, 0.30, 0.035],
            0.10,
            0.0,
            0.02,
            1.0,
        ),
        material(
            MaterialKind::RedCloak,
            [0.66, 0.055, 0.04],
            0.14,
            0.0,
            0.03,
            1.0,
        ),
        material(
            MaterialKind::NarutoSkin,
            [0.91, 0.58, 0.38],
            0.20,
            0.0,
            0.02,
            1.0,
        ),
        material(
            MaterialKind::NarutoHair,
            [1.00, 0.72, 0.06],
            0.34,
            0.0,
            0.05,
            1.0,
        ),
        material(
            MaterialKind::Ink,
            [0.018, 0.022, 0.030],
            0.12,
            0.0,
            0.04,
            1.0,
        ),
        material(
            MaterialKind::Metal,
            [0.62, 0.68, 0.74],
            0.95,
            0.0,
            0.58,
            1.0,
        ),
        material(
            MaterialKind::Chakra,
            [0.18, 0.55, 0.96],
            0.82,
            0.48,
            0.18,
            1.18,
        ),
        material(
            MaterialKind::Wood,
            [0.30, 0.12, 0.055],
            0.12,
            0.0,
            0.03,
            1.0,
        ),
        material(
            MaterialKind::EyeGold,
            [0.96, 0.68, 0.08],
            0.70,
            0.0,
            0.16,
            1.0,
        ),
        material(
            MaterialKind::Belly,
            [0.78, 0.68, 0.53],
            0.18,
            0.0,
            0.025,
            1.0,
        ),
        material(MaterialKind::Rope, [0.56, 0.37, 0.17], 0.10, 0.0, 0.02, 1.0),
        material(
            MaterialKind::CraterEarth,
            [0.39, 0.22, 0.12],
            0.07,
            0.0,
            0.015,
            1.0,
        ),
        material(
            MaterialKind::RuinStone,
            [0.46, 0.43, 0.38],
            0.15,
            0.0,
            0.045,
            1.0,
        ),
        material(
            MaterialKind::RoofTile,
            [0.30, 0.09, 0.055],
            0.20,
            0.0,
            0.06,
            1.0,
        ),
        material(
            MaterialKind::DustSmoke,
            [0.50, 0.39, 0.31],
            0.05,
            0.22,
            0.015,
            1.03,
        ),
        material(
            MaterialKind::CarvedStone,
            [0.68, 0.59, 0.45],
            0.18,
            0.0,
            0.05,
            1.0,
        ),
        material(
            MaterialKind::GamahiroMarking,
            [0.055, 0.25, 0.24],
            0.26,
            0.0,
            0.045,
            1.0,
        ),
        material(
            MaterialKind::GamakichiSkin,
            [0.92, 0.31, 0.055],
            0.24,
            0.0,
            0.035,
            1.0,
        ),
    ]
}

fn material(
    kind: MaterialKind,
    color: [f32; 3],
    specular: f32,
    transparency: f32,
    reflectivity: f32,
    refractive_index: f32,
) -> Material {
    Material {
        kind,
        albedo: Color::new(color[0], color[1], color[2]),
        specular,
        transparency,
        reflectivity,
        refractive_index,
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

fn scale_pattern(p: Vec3, frequency: f32) -> f32 {
    let row = (p.y * frequency).floor() as i32;
    let stagger = if row & 1 == 0 { 0.0 } else { 0.5 };
    let x = (p.x * frequency + stagger).fract().abs();
    let y = (p.y * frequency).fract().abs();
    let centered_x = (x - 0.5).abs() * 2.0;
    let arc = centered_x * centered_x + (y - 0.22).powi(2) * 2.8;
    if (0.58..0.86).contains(&arc) {
        1.0
    } else {
        0.0
    }
}
