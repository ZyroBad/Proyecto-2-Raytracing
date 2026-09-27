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

    // Sandals, toes and separated legs give the stance a readable silhouette.
    for side in [-1.0, 1.0] {
        let x = side * 0.43;
        add_block(
            scene,
            base + Vec3::new(x, 0.10, 0.20),
            Vec3::new(0.50, 0.20, 0.82),
            ink,
        );
        add_block(
            scene,
            base + Vec3::new(x, 0.17, 0.58),
            Vec3::new(0.38, 0.10, 0.16),
            skin,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(x, 0.62, 0.02),
            Vec3::new(0.27, 0.54, 0.32),
            0.16,
            orange,
        );
        add_block(
            scene,
            base + Vec3::new(x, 0.62, 0.34),
            Vec3::new(0.48, 0.15, 0.10),
            ink,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 0.34, 1.25, 0.0),
            Vec3::new(0.34, 0.48, 0.37),
            0.17,
            orange,
        );
    }

    // Fine cloak tiles form two independent tails instead of one flat slab.
    for row in 0..9 {
        let y = 0.55 + row as f32 * 0.19;
        let half_width = 0.50 + row as f32 * 0.055;
        for side in [-1.0, 1.0] {
            for column in 0..3 {
                let x = side * (0.22 + column as f32 * half_width / 3.0);
                add_block(
                    scene,
                    base + Vec3::new(x, y, -0.43),
                    Vec3::new(0.20, 0.20, 0.16),
                    cloak,
                );
            }
        }
    }

    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 1.86, 0.0),
        Vec3::new(0.76, 0.72, 0.43),
        0.18,
        orange,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 1.72, 0.43),
        Vec3::new(1.05, 0.18, 0.10),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.08, 0.44),
        Vec3::new(0.92, 0.16, 0.10),
        ink,
    );

    // Horizontal summoning scroll, visible when the camera orbits behind him.
    add_voxel_segment(
        scene,
        base + Vec3::new(-0.90, 1.92, -0.66),
        base + Vec3::new(0.90, 1.92, -0.66),
        0.22,
        wood,
    );
    for side in [-1.0, 1.0] {
        add_block(
            scene,
            base + Vec3::new(side * 0.92, 1.92, -0.66),
            Vec3::new(0.18, 0.54, 0.54),
            rope,
        );
    }

    // Cloak shoulders and arms follow a relaxed, confident arrival pose.
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 0.70, 2.25, -0.03),
            Vec3::new(0.34, 0.32, 0.42),
            0.16,
            cloak,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.76, 2.18, 0.02),
            base + Vec3::new(side * 0.92, 1.42, 0.20),
            0.24,
            cloak,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 0.94, 1.27, 0.24),
            Vec3::new(0.22, 0.27, 0.22),
            0.13,
            skin,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-0.48, 2.37, 0.15),
        Vec3::new(0.56, 0.23, 0.62),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(0.48, 2.37, 0.15),
        Vec3::new(0.56, 0.23, 0.62),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.44, 0.06),
        Vec3::new(0.42, 0.34, 0.48),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.58, -0.02),
        Vec3::new(0.30, 0.28, 0.30),
        skin,
    );

    // Hair volume goes behind the face; individual stepped spikes break the cube silhouette.
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 3.30, -0.06),
        Vec3::new(0.67, 0.57, 0.47),
        0.16,
        hair,
    );
    for (start, end) in [
        (Vec3::new(-0.53, 3.45, -0.02), Vec3::new(-0.88, 3.78, -0.04)),
        (Vec3::new(-0.36, 3.61, -0.03), Vec3::new(-0.53, 4.03, -0.05)),
        (Vec3::new(-0.13, 3.67, -0.03), Vec3::new(-0.16, 4.15, -0.06)),
        (Vec3::new(0.10, 3.67, -0.03), Vec3::new(0.18, 4.14, -0.06)),
        (Vec3::new(0.34, 3.59, -0.03), Vec3::new(0.54, 4.00, -0.05)),
        (Vec3::new(0.52, 3.43, -0.02), Vec3::new(0.88, 3.75, -0.04)),
    ] {
        add_voxel_segment(scene, base + start, base + end, 0.17, hair);
    }

    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 3.12, 0.10),
        Vec3::new(0.57, 0.62, 0.45),
        0.14,
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(-0.62, 3.13, 0.04),
        Vec3::new(0.13, 0.34, 0.30),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.62, 3.13, 0.04),
        Vec3::new(0.13, 0.34, 0.30),
        skin,
    );

    // Forehead protector, Sage Mode pigmentation, golden irises and vertical pupils.
    add_block(
        scene,
        base + Vec3::new(0.0, 3.39, 0.49),
        Vec3::new(1.14, 0.22, 0.11),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 3.40, 0.57),
        Vec3::new(0.68, 0.24, 0.08),
        metal,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 3.40, 0.63),
        Vec3::new(0.16, 0.10, 0.05),
        ink,
    );
    for side in [-1.0, 1.0] {
        let x = side * 0.25;
        add_block(
            scene,
            base + Vec3::new(x, 3.13, 0.53),
            Vec3::new(0.43, 0.18, 0.08),
            cloak,
        );
        add_block(
            scene,
            base + Vec3::new(x, 3.13, 0.59),
            Vec3::new(0.24, 0.10, 0.07),
            eye,
        );
        add_block(
            scene,
            base + Vec3::new(x, 3.13, 0.65),
            Vec3::new(0.045, 0.11, 0.04),
            ink,
        );
        for (y, outward) in [(2.96, 0.0), (2.86, 0.03), (2.76, 0.06)] {
            add_block(
                scene,
                base + Vec3::new(side * (0.39 + outward), y, 0.58),
                Vec3::new(0.25, 0.028, 0.045),
                ink,
            );
        }
    }
    add_block(
        scene,
        base + Vec3::new(0.0, 2.88, 0.59),
        Vec3::new(0.08, 0.12, 0.06),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.72, 0.57),
        Vec3::new(0.30, 0.045, 0.05),
        ink,
    );

    // Sparse translucent chakra wisps preserve the raytraced refractive accent.
    for (start, end) in [
        (Vec3::new(-1.06, 0.78, -0.18), Vec3::new(-1.16, 1.55, -0.14)),
        (Vec3::new(1.10, 1.48, -0.12), Vec3::new(1.18, 2.18, -0.08)),
        (Vec3::new(-0.83, 2.70, -0.10), Vec3::new(-0.92, 3.32, -0.06)),
        (Vec3::new(0.62, 3.72, -0.12), Vec3::new(0.52, 4.12, -0.10)),
    ] {
        add_voxel_segment(scene, base + start, base + end, 0.09, chakra);
    }
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

fn add_voxel_ellipsoid(scene: &mut Scene, center: Vec3, radii: Vec3, cell: f32, material: usize) {
    let cells_x = (radii.x / cell).ceil() as i32;
    let cells_y = (radii.y / cell).ceil() as i32;
    let cells_z = (radii.z / cell).ceil() as i32;
    for y in -cells_y..=cells_y {
        for z in -cells_z..=cells_z {
            for x in -cells_x..=cells_x {
                let offset = Vec3::new(x as f32 * cell, y as f32 * cell, z as f32 * cell);
                let normalized = (offset.x / radii.x).powi(2)
                    + (offset.y / radii.y).powi(2)
                    + (offset.z / radii.z).powi(2);
                if normalized <= 1.0 {
                    add_block(
                        scene,
                        center + offset,
                        Vec3::new(cell, cell, cell),
                        material,
                    );
                }
            }
        }
    }
}

fn add_voxel_segment(scene: &mut Scene, start: Vec3, end: Vec3, thickness: f32, material: usize) {
    let delta = end - start;
    let steps = (delta.length() / (thickness * 0.72)).ceil().max(1.0) as usize;
    for step in 0..=steps {
        let t = step as f32 / steps as f32;
        add_block(
            scene,
            start + delta * t,
            Vec3::new(thickness, thickness, thickness),
            material,
        );
    }
}

fn add_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    scene.cubes.push(Cube {
        min: center - size * 0.5,
        max: center + size * 0.5,
        material,
    });
}
