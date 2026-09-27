use crate::math::Vec3;
use crate::scene::{Cube, Scene};

pub fn add_gamabunta(scene: &mut Scene, base: Vec3) {
    let skin = 2;
    let robe = 5;
    let pale = 15;
    let ink = 10;
    let eye = 14;
    let rope = 16;

    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.45, 0.72, 0.92),
            Vec3::new(1.55, 0.62, 1.48),
            0.34,
            skin,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 2.75, -0.05),
        Vec3::new(3.05, 2.05, 1.72),
        0.34,
        skin,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 2.85, 1.58),
        Vec3::new(1.72, 1.52, 0.30),
        0.27,
        pale,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.50, 3.35, 0.45),
            Vec3::new(0.78, 1.88, 1.18),
            0.30,
            robe,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 5.78, 0.48),
        Vec3::new(3.05, 1.38, 1.52),
        0.30,
        skin,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 6.86, 0.30),
        Vec3::new(2.28, 0.48, 1.08),
        0.28,
        skin,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.78, 5.62, 0.58),
            Vec3::new(0.48, 0.82, 1.08),
            0.26,
            skin,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 5.25, 1.84),
        Vec3::new(2.22, 0.62, 0.52),
        0.25,
        skin,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.48, 6.30, 1.87),
            Vec3::new(0.58, 0.38, 0.20),
            0.16,
            eye,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.92, 6.58, 2.02),
            base + Vec3::new(side * 2.00, 6.72, 1.92),
            0.18,
            ink,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-1.48, 6.30, 2.10),
        Vec3::new(0.18, 0.48, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.48, 6.30, 2.10),
        Vec3::new(0.18, 0.48, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(-0.64, 5.54, 2.38),
        Vec3::new(0.22, 0.18, 0.10),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.64, 5.54, 2.38),
        Vec3::new(0.22, 0.18, 0.10),
        ink,
    );
    add_voxel_segment(
        scene,
        base + Vec3::new(-1.58, 4.82, 2.18),
        base + Vec3::new(1.58, 4.82, 2.18),
        0.16,
        ink,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 6.98, -0.02),
        Vec3::new(0.58, 0.34, 0.62),
        0.22,
        skin,
    );
    for y in [2.12, 2.78, 3.44] {
        add_voxel_segment(
            scene,
            base + Vec3::new(-1.18, y, 1.91),
            base + Vec3::new(1.18, y, 1.91),
            0.10,
            rope,
        );
    }
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
    add_gamabunta_limbs(scene, base, skin);
}

pub fn add_gamaken(scene: &mut Scene, base: Vec3) {
    let skin = 3;
    let robe = 5;
    let pale = 15;
    let ink = 10;
    let metal = 11;
    let eye = 14;

    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.72, 0.70, 0.76),
            Vec3::new(1.12, 0.58, 1.28),
            0.29,
            skin,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 2.65, -0.05),
        Vec3::new(2.35, 1.90, 1.42),
        0.30,
        skin,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 2.90, 1.34),
        Vec3::new(1.28, 1.38, 0.27),
        0.25,
        pale,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.88, 3.02, 0.44),
            Vec3::new(0.67, 1.72, 0.94),
            0.27,
            robe,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 5.40, 0.48),
        Vec3::new(2.26, 1.23, 1.34),
        0.27,
        skin,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 6.42, 0.28),
        Vec3::new(1.68, 0.43, 0.94),
        0.25,
        skin,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.08, 5.30, 0.55),
            Vec3::new(0.42, 0.72, 0.92),
            0.23,
            skin,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 4.95, 1.63),
        Vec3::new(1.65, 0.56, 0.48),
        0.23,
        pale,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.08, 5.86, 1.69),
            Vec3::new(0.44, 0.31, 0.18),
            0.14,
            eye,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.70, 6.10, 1.78),
            base + Vec3::new(side * 1.50, 6.25, 1.65),
            0.16,
            ink,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-1.08, 5.86, 1.91),
        Vec3::new(0.15, 0.38, 0.10),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.08, 5.86, 1.91),
        Vec3::new(0.15, 0.38, 0.10),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 4.55, 2.14),
        Vec3::new(2.2, 0.16, 0.12),
        ink,
    );

    add_voxel_segment(
        scene,
        base + Vec3::new(-3.15, 0.42, 1.55),
        base + Vec3::new(-3.15, 8.18, 1.55),
        0.20,
        metal,
    );
    add_voxel_segment(
        scene,
        base + Vec3::new(-3.78, 8.18, 1.55),
        base + Vec3::new(-2.52, 8.18, 1.55),
        0.20,
        metal,
    );
    for x in [-3.72, -3.15, -2.58] {
        add_voxel_segment(
            scene,
            base + Vec3::new(x, 8.18, 1.55),
            base + Vec3::new(x + (x + 3.15) * 0.28, 9.20, 1.55),
            0.18,
            metal,
        );
    }
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
        base + Vec3::new(-0.55, 5.12, 2.14),
        Vec3::new(0.18, 0.18, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.55, 5.12, 2.14),
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
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(-3.12, 2.65, 1.42),
        Vec3::new(0.42, 0.35, 0.42),
        0.19,
        skin,
    );
    add_gamaken_limbs(scene, base, skin, metal);
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

    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.82, 0.72, 0.80),
            Vec3::new(1.22, 0.60, 1.32),
            0.30,
            skin,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 2.72, -0.04),
        Vec3::new(2.50, 1.98, 1.48),
        0.30,
        skin,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 2.58, 1.42),
        Vec3::new(1.38, 1.40, 0.28),
        0.25,
        pale,
    );
    add_voxel_segment(
        scene,
        base + Vec3::new(-1.72, 1.73, 1.73),
        base + Vec3::new(1.72, 1.73, 1.73),
        0.34,
        orange,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.00, 3.08, 0.45),
            Vec3::new(0.69, 1.78, 0.98),
            0.27,
            robe,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 5.60, 0.46),
        Vec3::new(2.38, 1.27, 1.39),
        0.27,
        skin,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 6.66, 0.27),
        Vec3::new(1.76, 0.44, 0.98),
        0.25,
        skin,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.18, 5.48, 0.52),
            Vec3::new(0.44, 0.74, 0.96),
            0.23,
            skin,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 5.08, 1.68),
        Vec3::new(1.72, 0.58, 0.50),
        0.23,
        pale,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.13, 6.03, 1.74),
            Vec3::new(0.46, 0.32, 0.18),
            0.14,
            eye,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.74, 6.28, 1.82),
            base + Vec3::new(side * 1.56, 6.40, 1.68),
            0.16,
            robe,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-1.13, 6.03, 1.96),
        Vec3::new(0.15, 0.39, 0.10),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.13, 6.03, 1.96),
        Vec3::new(0.15, 0.39, 0.10),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 4.67, 2.18),
        Vec3::new(2.35, 0.16, 0.12),
        ink,
    );

    for side in [-1.0, 1.0] {
        let hilt_bottom = Vec3::new(side * 1.42, 5.25, -0.78);
        let guard = Vec3::new(side * 1.68, 6.58, -0.78);
        let blade_tip = Vec3::new(side * 2.30, 9.40, -0.78);
        add_voxel_segment(scene, base + hilt_bottom, base + guard, 0.28, wood);
        add_voxel_segment(scene, base + guard, base + blade_tip, 0.19, metal);
        add_voxel_segment(
            scene,
            base + guard + Vec3::new(-0.52, 0.0, 0.0),
            base + guard + Vec3::new(0.52, 0.0, 0.0),
            0.19,
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
        base + Vec3::new(-0.58, 5.28, 2.18),
        Vec3::new(0.18, 0.18, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.58, 5.28, 2.18),
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
    add_gamahiro_limbs(scene, base, skin);
}

fn add_gamabunta_limbs(scene: &mut Scene, base: Vec3, skin: usize) {
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.92, 3.42, 0.46),
            Vec3::new(0.72, 1.22, 0.78),
            0.28,
            skin,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 3.10, 2.18, 1.08),
            Vec3::new(0.62, 0.86, 0.76),
            0.26,
            skin,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 3.00, 1.34, 1.82),
            Vec3::new(0.90, 0.42, 0.72),
            0.24,
            skin,
        );
        for finger in [-0.48, 0.0, 0.48] {
            add_voxel_segment(
                scene,
                base + Vec3::new(side * 3.00 + finger, 1.20, 2.10),
                base + Vec3::new(side * 3.00 + finger, 0.92, 2.82),
                0.24,
                skin,
            );
        }
    }
}

fn add_gamaken_limbs(scene: &mut Scene, base: Vec3, skin: usize, metal: usize) {
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.18, 3.20, 0.42),
            Vec3::new(0.58, 1.04, 0.66),
            0.25,
            skin,
        );
        let forearm_end = if side < 0.0 {
            Vec3::new(-3.00, 2.72, 1.36)
        } else {
            Vec3::new(2.48, 1.95, 1.02)
        };
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 2.28, 2.72, 0.72),
            base + forearm_end,
            0.48,
            skin,
        );
        if side > 0.0 {
            add_voxel_ellipsoid(
                scene,
                base + Vec3::new(2.42, 1.42, 1.68),
                Vec3::new(0.72, 0.36, 0.62),
                0.22,
                skin,
            );
            for finger in [-0.36, 0.0, 0.36] {
                add_voxel_segment(
                    scene,
                    base + Vec3::new(2.42 + finger, 1.30, 1.90),
                    base + Vec3::new(2.42 + finger, 1.04, 2.50),
                    0.20,
                    skin,
                );
            }
        } else {
            for y in [2.48, 2.68, 2.88] {
                add_block(
                    scene,
                    base + Vec3::new(-3.12, y, 1.42),
                    Vec3::new(0.56, 0.13, 0.56),
                    skin,
                );
            }
            add_block(
                scene,
                base + Vec3::new(-3.12, 2.67, 1.60),
                Vec3::new(0.24, 0.72, 0.12),
                metal,
            );
        }
    }
}

fn add_gamahiro_limbs(scene: &mut Scene, base: Vec3, skin: usize) {
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.35, 3.30, 0.40),
            Vec3::new(0.62, 1.08, 0.70),
            0.25,
            skin,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.58, 2.10, 1.02),
            Vec3::new(0.54, 0.78, 0.68),
            0.24,
            skin,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.48, 1.36, 1.72),
            Vec3::new(0.76, 0.37, 0.64),
            0.22,
            skin,
        );
        for finger in [-0.38, 0.0, 0.38] {
            add_voxel_segment(
                scene,
                base + Vec3::new(side * 2.48 + finger, 1.24, 1.94),
                base + Vec3::new(side * 2.48 + finger, 0.98, 2.56),
                0.20,
                skin,
            );
        }
    }
}

pub fn add_naruto_sage(scene: &mut Scene, base: Vec3) {
    let first_cube = scene.cubes.len();
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

    const NARUTO_SCALE: f32 = 0.68;
    for cube in &mut scene.cubes[first_cube..] {
        cube.min = base + (cube.min - base) * NARUTO_SCALE;
        cube.max = base + (cube.max - base) * NARUTO_SCALE;
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
