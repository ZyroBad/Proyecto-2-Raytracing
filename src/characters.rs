use crate::math::Vec3;
use crate::scene::{Cube, Scene};

pub fn add_gamabunta(scene: &mut Scene, base: Vec3) {
    let skin = 2;
    let robe = 5;
    let pale = 15;
    let ink = 10;
    let eye = 14;
    let rope = 16;

    add_block(
        scene,
        base + Vec3::new(-2.5, 0.8, 1.1),
        Vec3::new(3.2, 1.2, 3.2),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(2.5, 0.8, 1.1),
        Vec3::new(3.2, 1.2, 3.2),
        skin,
    );
    add_rounded_mass(
        scene,
        base + Vec3::new(0.0, 2.8, 0.0),
        Vec3::new(6.6, 4.0, 3.8),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.9, 2.0),
        Vec3::new(3.6, 2.8, 0.35),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-2.7, 3.4, 0.9),
        Vec3::new(1.35, 3.8, 2.2),
        robe,
    );
    add_block(
        scene,
        base + Vec3::new(2.7, 3.4, 0.9),
        Vec3::new(1.35, 3.8, 2.2),
        robe,
    );
    add_rounded_mass(
        scene,
        base + Vec3::new(0.0, 5.8, 0.6),
        Vec3::new(6.2, 2.6, 3.2),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 7.0, 0.35),
        Vec3::new(4.7, 0.55, 2.35),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(-3.0, 5.65, 0.65),
        Vec3::new(0.65, 1.55, 2.25),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(3.0, 5.65, 0.65),
        Vec3::new(0.65, 1.55, 2.25),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 5.35, 2.2),
        Vec3::new(4.6, 1.0, 1.1),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-1.55, 6.35, 2.25),
        Vec3::new(1.05, 0.64, 0.28),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(1.55, 6.35, 2.25),
        Vec3::new(1.05, 0.64, 0.28),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(-1.55, 6.34, 2.42),
        Vec3::new(0.24, 0.52, 0.16),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.55, 6.34, 2.42),
        Vec3::new(0.24, 0.52, 0.16),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(-0.72, 5.62, 2.80),
        Vec3::new(0.22, 0.20, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.72, 5.62, 2.80),
        Vec3::new(0.22, 0.20, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 4.85, 2.77),
        Vec3::new(2.7, 0.18, 0.14),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 6.95, 0.0),
        Vec3::new(1.1, 0.55, 1.2),
        skin,
    );
    for x in [-2.45, -1.95, 1.95, 2.45] {
        add_block(
            scene,
            base + Vec3::new(x, 6.78, 2.18),
            Vec3::new(0.42, 0.22, 0.18),
            ink,
        );
    }
    for (x, y) in [(-2.25, 5.62), (-1.20, 6.98), (1.10, 6.96), (2.45, 5.82)] {
        add_block(
            scene,
            base + Vec3::new(x, y, 2.13),
            Vec3::new(0.28, 0.22, 0.22),
            skin,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-1.95, 4.30, 2.10),
        Vec3::new(0.32, 0.32, 0.24),
        rope,
    );
    add_block(
        scene,
        base + Vec3::new(-1.25, 4.08, 2.12),
        Vec3::new(0.32, 0.32, 0.24),
        rope,
    );
    add_block(
        scene,
        base + Vec3::new(-0.55, 3.96, 2.14),
        Vec3::new(0.32, 0.32, 0.24),
        rope,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 3.88, 2.16),
        Vec3::new(0.48, 0.56, 0.30),
        rope,
    );
    add_block(
        scene,
        base + Vec3::new(0.55, 3.96, 2.14),
        Vec3::new(0.32, 0.32, 0.24),
        rope,
    );
    add_block(
        scene,
        base + Vec3::new(1.25, 4.08, 2.12),
        Vec3::new(0.32, 0.32, 0.24),
        rope,
    );
    add_block(
        scene,
        base + Vec3::new(1.95, 4.30, 2.10),
        Vec3::new(0.32, 0.32, 0.24),
        rope,
    );
    add_toad_limbs(scene, base, skin, 3.05, 1.0);
}

pub fn add_gamaken(scene: &mut Scene, base: Vec3) {
    let skin = 3;
    let robe = 5;
    let pale = 15;
    let ink = 10;
    let metal = 11;
    let eye = 14;

    add_block(
        scene,
        base + Vec3::new(-1.7, 0.8, 0.8),
        Vec3::new(2.3, 1.1, 2.6),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(1.7, 0.8, 0.8),
        Vec3::new(2.3, 1.1, 2.6),
        skin,
    );
    add_rounded_mass(
        scene,
        base + Vec3::new(0.0, 2.7, 0.0),
        Vec3::new(4.8, 3.8, 3.0),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 3.0, 1.65),
        Vec3::new(2.6, 2.5, 0.3),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-1.9, 3.0, 0.6),
        Vec3::new(1.15, 3.5, 1.8),
        robe,
    );
    add_block(
        scene,
        base + Vec3::new(1.9, 3.0, 0.6),
        Vec3::new(1.15, 3.5, 1.8),
        robe,
    );
    add_rounded_mass(
        scene,
        base + Vec3::new(0.0, 5.45, 0.55),
        Vec3::new(4.5, 2.4, 2.8),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 6.55, 0.35),
        Vec3::new(3.4, 0.50, 2.0),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(-2.18, 5.35, 0.62),
        Vec3::new(0.55, 1.45, 1.95),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(2.18, 5.35, 0.62),
        Vec3::new(0.55, 1.45, 1.95),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 5.0, 1.94),
        Vec3::new(3.35, 0.92, 0.95),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-1.1, 5.9, 1.98),
        Vec3::new(0.78, 0.52, 0.24),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(1.1, 5.9, 1.98),
        Vec3::new(0.78, 0.52, 0.24),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(-1.1, 5.9, 2.12),
        Vec3::new(0.18, 0.40, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.1, 5.9, 2.12),
        Vec3::new(0.18, 0.40, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 4.58, 2.43),
        Vec3::new(2.2, 0.16, 0.12),
        ink,
    );

    add_block(
        scene,
        base + Vec3::new(-3.15, 4.5, 0.3),
        Vec3::new(0.32, 8.8, 0.32),
        metal,
    );
    for x in [-3.75, -3.15, -2.55] {
        add_block(
            scene,
            base + Vec3::new(x, 8.85, 0.3),
            Vec3::new(0.24, 1.6, 0.24),
            metal,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-3.15, 8.15, 0.3),
        Vec3::new(1.45, 0.25, 0.25),
        metal,
    );
    add_block(
        scene,
        base + Vec3::new(-1.48, 6.28, 1.92),
        Vec3::new(0.72, 0.20, 0.20),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.48, 6.28, 1.92),
        Vec3::new(0.72, 0.20, 0.20),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(-0.58, 5.18, 2.48),
        Vec3::new(0.18, 0.18, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.58, 5.18, 2.48),
        Vec3::new(0.18, 0.18, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(-2.25, 3.58, 1.72),
        Vec3::new(0.35, 1.55, 0.22),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(2.25, 3.58, 1.72),
        Vec3::new(0.35, 1.55, 0.22),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-3.12, 2.65, 1.10),
        Vec3::new(0.72, 0.58, 0.72),
        skin,
    );
    add_toad_limbs(scene, base, skin, 2.25, 0.84);
}

pub fn add_gamahiro(scene: &mut Scene, base: Vec3) {
    let skin = 4;
    let robe = 5;
    let orange = 6;
    let pale = 15;
    let ink = 10;
    let metal = 11;
    let wood = 13;
    let eye = 14;

    add_block(
        scene,
        base + Vec3::new(-1.8, 0.8, 0.8),
        Vec3::new(2.5, 1.1, 2.7),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(1.8, 0.8, 0.8),
        Vec3::new(2.5, 1.1, 2.7),
        skin,
    );
    add_rounded_mass(
        scene,
        base + Vec3::new(0.0, 2.8, 0.0),
        Vec3::new(5.0, 4.0, 3.1),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.5, 1.7),
        Vec3::new(2.8, 2.6, 0.3),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 1.75, 1.95),
        Vec3::new(3.8, 0.55, 0.42),
        orange,
    );
    add_block(
        scene,
        base + Vec3::new(-2.0, 3.1, 0.6),
        Vec3::new(1.15, 3.6, 1.9),
        robe,
    );
    add_block(
        scene,
        base + Vec3::new(2.0, 3.1, 0.6),
        Vec3::new(1.15, 3.6, 1.9),
        robe,
    );
    add_rounded_mass(
        scene,
        base + Vec3::new(0.0, 5.65, 0.5),
        Vec3::new(4.7, 2.5, 2.9),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 6.80, 0.30),
        Vec3::new(3.55, 0.50, 2.05),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(-2.28, 5.55, 0.58),
        Vec3::new(0.55, 1.48, 2.0),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(2.28, 5.55, 0.58),
        Vec3::new(0.55, 1.48, 2.0),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 5.15, 1.95),
        Vec3::new(3.5, 0.95, 1.0),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-1.15, 6.08, 2.0),
        Vec3::new(0.80, 0.54, 0.24),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(1.15, 6.08, 2.0),
        Vec3::new(0.80, 0.54, 0.24),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(-1.15, 6.08, 2.14),
        Vec3::new(0.18, 0.42, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.15, 6.08, 2.14),
        Vec3::new(0.18, 0.42, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 4.70, 2.45),
        Vec3::new(2.35, 0.16, 0.12),
        ink,
    );

    for x in [-1.75, 1.75] {
        add_block(
            scene,
            base + Vec3::new(x, 7.9, -0.9),
            Vec3::new(0.48, 5.0, 0.48),
            wood,
        );
        add_block(
            scene,
            base + Vec3::new(x, 9.0, -0.9),
            Vec3::new(0.24, 3.3, 0.24),
            metal,
        );
        add_block(
            scene,
            base + Vec3::new(x, 6.65, -0.55),
            Vec3::new(1.25, 0.20, 0.65),
            metal,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-1.55, 6.46, 1.95),
        Vec3::new(0.72, 0.20, 0.20),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.55, 6.46, 1.95),
        Vec3::new(0.72, 0.20, 0.20),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(-0.62, 5.34, 2.50),
        Vec3::new(0.18, 0.18, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.62, 5.34, 2.50),
        Vec3::new(0.18, 0.18, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(-2.38, 3.72, 1.72),
        Vec3::new(0.35, 1.62, 0.22),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(2.38, 3.72, 1.72),
        Vec3::new(0.35, 1.62, 0.22),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 6.62, 1.48),
        Vec3::new(3.65, 0.24, 0.32),
        robe,
    );
    add_toad_limbs(scene, base, skin, 2.35, 0.88);
}

fn add_toad_limbs(scene: &mut Scene, base: Vec3, skin: usize, half_width: f32, scale: f32) {
    for side in [-1.0, 1.0] {
        add_block(
            scene,
            base + Vec3::new(side * (half_width + 0.18), 3.45, 0.45),
            Vec3::new(1.12, 2.25, 1.55) * scale,
            skin,
        );
        add_block(
            scene,
            base + Vec3::new(side * (half_width + 0.42), 2.05, 1.15),
            Vec3::new(1.0, 1.55, 1.45) * scale,
            skin,
        );
        add_block(
            scene,
            base + Vec3::new(side * (half_width + 0.28), 1.15, 2.02),
            Vec3::new(1.45, 0.62, 1.55) * scale,
            skin,
        );
        for finger in [-0.38, 0.0, 0.38] {
            add_block(
                scene,
                base + Vec3::new(side * (half_width + 0.28) + finger * scale, 0.78, 2.65),
                Vec3::new(0.30, 0.34, 0.72) * scale,
                skin,
            );
        }
    }
}

pub fn add_naruto_sage(scene: &mut Scene, base: Vec3) {
    let orange = 6;
    let cloak = 7;
    let skin = 8;
    let hair = 9;
    let ink = 10;
    let metal = 11;
    let chakra = 12;
    let wood = 13;
    let eye = 14;
    let rope = 16;

    add_block(
        scene,
        base + Vec3::new(-0.48, -0.02, 0.18),
        Vec3::new(0.72, 0.28, 0.92),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.48, -0.02, 0.18),
        Vec3::new(0.72, 0.28, 0.92),
        ink,
    );

    add_block(
        scene,
        base + Vec3::new(-0.48, 0.55, 0.0),
        Vec3::new(0.65, 1.25, 0.75),
        orange,
    );
    add_block(
        scene,
        base + Vec3::new(0.48, 0.55, 0.0),
        Vec3::new(0.65, 1.25, 0.75),
        orange,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 1.65, 0.0),
        Vec3::new(1.95, 1.50, 1.02),
        orange,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 1.65, -0.55),
        Vec3::new(2.45, 1.90, 0.34),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(-0.62, 0.85, -0.62),
        Vec3::new(0.72, 1.75, 0.24),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(0.62, 0.85, -0.62),
        Vec3::new(0.72, 1.75, 0.24),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.05, -0.82),
        Vec3::new(2.45, 0.62, 0.54),
        wood,
    );
    add_block(
        scene,
        base + Vec3::new(-0.82, 2.05, -1.10),
        Vec3::new(0.22, 0.68, 0.16),
        rope,
    );
    add_block(
        scene,
        base + Vec3::new(0.82, 2.05, -1.10),
        Vec3::new(0.22, 0.68, 0.16),
        rope,
    );
    add_block(
        scene,
        base + Vec3::new(-0.82, 1.82, 0.35),
        Vec3::new(0.38, 1.15, 0.42),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(0.82, 1.82, 0.35),
        Vec3::new(0.38, 1.15, 0.42),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(-0.38, 1.78, 0.72),
        Vec3::new(0.82, 0.30, 0.34),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.38, 1.78, 0.72),
        Vec3::new(0.82, 0.30, 0.34),
        skin,
    );
    add_rounded_mass(
        scene,
        base + Vec3::new(0.0, 2.72, 0.05),
        Vec3::new(1.55, 1.28, 1.05),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.94, 0.55),
        Vec3::new(1.62, 0.28, 0.18),
        metal,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.95, 0.65),
        Vec3::new(0.22, 0.13, 0.08),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(-0.78, 2.94, 0.42),
        Vec3::new(0.38, 0.13, 0.12),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(0.78, 2.94, 0.42),
        Vec3::new(0.38, 0.13, 0.12),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(-0.33, 2.76, 0.58),
        Vec3::new(0.30, 0.16, 0.10),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(0.33, 2.76, 0.58),
        Vec3::new(0.30, 0.16, 0.10),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(-0.33, 2.76, 0.65),
        Vec3::new(0.08, 0.13, 0.07),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.33, 2.76, 0.65),
        Vec3::new(0.08, 0.13, 0.07),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.48, 0.58),
        Vec3::new(0.42, 0.08, 0.07),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(-0.48, 2.76, 0.64),
        Vec3::new(0.20, 0.22, 0.06),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(0.48, 2.76, 0.64),
        Vec3::new(0.20, 0.22, 0.06),
        cloak,
    );
    for y in [2.58, 2.46, 2.34] {
        add_block(
            scene,
            base + Vec3::new(-0.48, y, 0.62),
            Vec3::new(0.25, 0.035, 0.06),
            ink,
        );
        add_block(
            scene,
            base + Vec3::new(0.48, y, 0.62),
            Vec3::new(0.25, 0.035, 0.06),
            ink,
        );
    }
    add_block(
        scene,
        base + Vec3::new(0.0, 2.15, 0.10),
        Vec3::new(0.72, 0.25, 0.76),
        ink,
    );
    for x in [-0.48, -0.24, 0.0, 0.24, 0.48] {
        let height = if x == 0.0 { 0.72 } else { 0.55 };
        add_block(
            scene,
            base + Vec3::new(x, 3.45, -0.02),
            Vec3::new(0.28, height, 0.52),
            hair,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-0.72, 3.35, -0.05),
        Vec3::new(0.34, 0.42, 0.46),
        hair,
    );
    add_block(
        scene,
        base + Vec3::new(0.72, 3.35, -0.05),
        Vec3::new(0.34, 0.42, 0.46),
        hair,
    );
    add_block(
        scene,
        base + Vec3::new(-0.94, 1.35, -0.15),
        Vec3::new(0.12, 0.38, 0.14),
        chakra,
    );
    add_block(
        scene,
        base + Vec3::new(1.02, 1.72, -0.10),
        Vec3::new(0.14, 0.46, 0.16),
        chakra,
    );
    add_block(
        scene,
        base + Vec3::new(-0.82, 2.65, -0.12),
        Vec3::new(0.13, 0.52, 0.15),
        chakra,
    );
    add_block(
        scene,
        base + Vec3::new(0.88, 3.05, -0.08),
        Vec3::new(0.15, 0.42, 0.17),
        chakra,
    );
    add_block(
        scene,
        base + Vec3::new(-0.42, 3.82, -0.10),
        Vec3::new(0.18, 0.26, 0.18),
        chakra,
    );
    add_block(
        scene,
        base + Vec3::new(0.38, 3.92, -0.10),
        Vec3::new(0.16, 0.22, 0.16),
        chakra,
    );
}

pub fn add_summoning_clouds(scene: &mut Scene) {
    for (x, z, scale) in [
        (-10.0, 2.8, 1.2),
        (-7.0, 3.5, 1.4),
        (-3.8, 3.8, 1.15),
        (0.0, 4.0, 1.45),
        (3.7, 3.7, 1.2),
        (7.0, 3.4, 1.4),
        (10.0, 2.7, 1.2),
        (-9.4, -2.5, 0.9),
        (9.4, -2.5, 0.9),
    ] {
        add_cloud_cluster(scene, Vec3::new(x, 0.65, z), scale);
    }
}

fn add_cloud_cluster(scene: &mut Scene, base: Vec3, scale: f32) {
    for (offset, size) in [
        (Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 1.1, 1.8)),
        (Vec3::new(-1.2, 0.35, 0.1), Vec3::new(1.8, 1.2, 1.5)),
        (Vec3::new(1.2, 0.42, -0.1), Vec3::new(1.9, 1.35, 1.6)),
        (Vec3::new(0.0, 0.65, 0.0), Vec3::new(1.7, 1.2, 1.4)),
    ] {
        add_block(scene, base + offset * scale, size * scale, 1);
    }
}

fn add_rounded_mass(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    const CELLS: i32 = 5;
    let cell_width = size.x / CELLS as f32;
    let cell_height = size.y / CELLS as f32;

    for row in -2..=2 {
        for column in -2..=2 {
            let nx = column as f32 / 2.35;
            let ny = row as f32 / 2.35;
            if nx * nx + ny * ny > 1.0 {
                continue;
            }

            add_block(
                scene,
                center + Vec3::new(column as f32 * cell_width, row as f32 * cell_height, 0.0),
                Vec3::new(cell_width * 1.08, cell_height * 1.08, size.z),
                material,
            );
        }
    }
}

fn add_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    scene.cubes.push(Cube {
        min: center - size * 0.5,
        max: center + size * 0.5,
        material,
    });
}
