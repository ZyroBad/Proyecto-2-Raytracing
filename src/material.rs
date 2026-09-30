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
    ForestCanopy,
    DistantMountain,
    PainSkin,
    Rinnegan,
    PainHair,
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
                let broad = noise(p * 0.72) * 0.12;
                let scars = stripe(p.x * 0.31 + p.y * 0.17 + p.z * 0.11, 0.72) * 0.045;
                self.albedo * (0.82 + broad + noise(p * 2.4) * 0.045 - scars)
            }
            MaterialKind::GamakenSkin => {
                let mottled = noise((p + Vec3::new(0.3, 0.0, 0.0)) * 0.86) * 0.15;
                let secondary = noise(p * 2.1) * 0.055;
                self.albedo * (0.79 + mottled + secondary)
            }
            MaterialKind::GamahiroSkin => {
                let bands = stripe(p.y + p.x * 0.12, 1.8) * 0.08;
                let broad = noise((p + Vec3::new(0.6, 0.0, 0.0)) * 0.78) * 0.12;
                self.albedo * (0.86 + broad + noise(p * 2.0) * 0.04 - bands)
            }
            MaterialKind::Robe => {
                let weave = noise(p * 14.0) * 0.055;
                let folds = (p.x * 2.1 + (p.y * 0.32).sin()).sin().abs() * 0.055;
                self.albedo * (0.78 + weave + folds)
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
                let bend_a = p.x * 0.42 + p.z * 0.68 + (p.z * 0.23).sin() * 1.6;
                let bend_b = p.x * 0.61 - p.z * 0.34 + (p.x * 0.19).sin() * 1.3;
                let crack_a = ((bend_a.sin().abs() - 0.94).max(0.0) * 2.3).min(0.14);
                let crack_b = ((bend_b.sin().abs() - 0.965).max(0.0) * 2.0).min(0.09);
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
                let freckles = noise(p * 2.7) * 0.08;
                self.albedo * (0.88 + freckles + noise(p * 0.9) * 0.06)
            }
            MaterialKind::ForestCanopy => {
                let clusters = noise(p * 3.4) * 0.22;
                let leaves = noise(p * 11.0) * 0.10;
                let sunlight = normal.y.max(0.0) * 0.12;
                self.albedo * (0.68 + clusters + leaves + sunlight)
            }
            MaterialKind::DistantMountain => {
                let strata = (p.y * 0.55 + p.x * 0.08).sin().abs() * 0.10;
                let erosion = noise(p * 1.8) * 0.18;
                self.albedo * (0.72 + erosion - strata)
            }
            MaterialKind::PainSkin => {
                let pores = noise(p * 11.0) * 0.055;
                let warm_variation = noise((p + Vec3::new(0.7, 0.2, 0.4)) * 3.0) * 0.05;
                self.albedo * (0.86 + pores + warm_variation)
            }
            MaterialKind::Rinnegan => {
                let rings = ((p.x * p.x + p.y * p.y).sqrt() * 22.0).sin().abs() * 0.13;
                self.albedo * (0.82 + rings + normal.z.max(0.0) * 0.08)
            }
            MaterialKind::PainHair => {
                let strands = stripe(p.x * 0.7 - p.y, 5.2) * 0.14;
                self.albedo * (0.78 + strands + noise(p * 7.0) * 0.08)
            }
        }
        .clamp01()
    }

    pub fn detailed_normal(self, p: Vec3, normal: Vec3) -> Vec3 {
        let (frequency, strength) = match self.kind {
            MaterialKind::Rock => (4.0, 0.045),
            MaterialKind::CraterEarth => (5.5, 0.060),
            MaterialKind::RuinStone => (4.8, 0.050),
            MaterialKind::RoofTile => (6.0, 0.035),
            MaterialKind::CarvedStone => (4.5, 0.042),
            MaterialKind::DistantMountain => (1.8, 0.032),
            MaterialKind::ForestCanopy => (6.5, 0.045),
            MaterialKind::GamabuntaSkin
            | MaterialKind::GamakenSkin
            | MaterialKind::GamahiroSkin
            | MaterialKind::GamakichiSkin => (6.0, 0.028),
            MaterialKind::Belly => (4.5, 0.020),
            MaterialKind::NarutoSkin | MaterialKind::PainSkin => (8.0, 0.008),
            MaterialKind::Robe | MaterialKind::OrangeCloth | MaterialKind::RedCloak => (9.0, 0.012),
            MaterialKind::Wood | MaterialKind::Rope => (7.0, 0.026),
            MaterialKind::NarutoHair | MaterialKind::PainHair => (8.0, 0.018),
            MaterialKind::Cloud | MaterialKind::DustSmoke => (2.2, 0.012),
            MaterialKind::Metal | MaterialKind::GamahiroMarking => (10.0, 0.006),
            MaterialKind::Chakra
            | MaterialKind::EyeGold
            | MaterialKind::Ink
            | MaterialKind::Rinnegan => (1.0, 0.0),
        };
        if strength == 0.0 {
            return normal;
        }

        let reference = if normal.y.abs() < 0.92 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let tangent = reference.cross(normal).normalized();
        let bitangent = normal.cross(tangent).normalized();
        let epsilon = 0.018;
        let du = (detail_height(p + tangent * epsilon, frequency)
            - detail_height(p - tangent * epsilon, frequency))
            / (epsilon * 2.0);
        let dv = (detail_height(p + bitangent * epsilon, frequency)
            - detail_height(p - bitangent * epsilon, frequency))
            / (epsilon * 2.0);
        (normal - tangent * du * strength - bitangent * dv * strength).normalized()
    }

    pub fn shininess(self) -> f32 {
        match self.kind {
            MaterialKind::Metal => 180.0,
            MaterialKind::EyeGold | MaterialKind::Rinnegan | MaterialKind::Chakra => 96.0,
            MaterialKind::GamabuntaSkin
            | MaterialKind::GamakenSkin
            | MaterialKind::GamahiroSkin
            | MaterialKind::GamakichiSkin => 42.0,
            MaterialKind::NarutoSkin | MaterialKind::PainSkin => 54.0,
            MaterialKind::Cloud | MaterialKind::DustSmoke => 22.0,
            MaterialKind::Rock
            | MaterialKind::CraterEarth
            | MaterialKind::RuinStone
            | MaterialKind::CarvedStone
            | MaterialKind::DistantMountain => 16.0,
            MaterialKind::Wood | MaterialKind::Rope | MaterialKind::ForestCanopy => 24.0,
            _ => 34.0,
        }
    }
}

fn detail_height(p: Vec3, frequency: f32) -> f32 {
    let q = p * frequency;
    let broad = (q.x * 0.73).sin() * (q.y * 0.41).cos() * (q.z * 0.59).sin();
    let crossed = (q.x * 1.31 + (q.z * 0.37).sin()).cos() * (q.y * 0.87 + (q.x * 0.29).sin()).sin();
    let fine = (q.x * 1.67).sin() * (q.y * 1.43).sin() * (q.z * 1.91).cos();
    broad * 0.52 + crossed * 0.31 + fine * 0.17
}

pub fn scene_materials() -> Vec<Material> {
    vec![
        material(MaterialKind::Rock, [0.43, 0.33, 0.23], 0.12, 0.0, 0.04, 1.0),
        material(
            MaterialKind::Cloud,
            [0.72, 0.78, 0.88],
            0.24,
            0.18,
            0.10,
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
        material(
            MaterialKind::ForestCanopy,
            [0.12, 0.34, 0.16],
            0.10,
            0.0,
            0.025,
            1.0,
        ),
        material(
            MaterialKind::DistantMountain,
            [0.31, 0.39, 0.34],
            0.08,
            0.0,
            0.02,
            1.0,
        ),
        material(
            MaterialKind::PainSkin,
            [0.78, 0.54, 0.40],
            0.20,
            0.0,
            0.025,
            1.0,
        ),
        material(
            MaterialKind::Rinnegan,
            [0.56, 0.42, 0.74],
            0.72,
            0.0,
            0.18,
            1.0,
        ),
        material(
            MaterialKind::PainHair,
            [0.92, 0.28, 0.035],
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
