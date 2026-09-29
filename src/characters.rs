use crate::math::Vec3;
use crate::scene::{Capsule, Cube, Ellipsoid, Scene, Triangle};
use std::f32::consts::PI;

pub fn add_gamabunta(scene: &mut Scene, base: Vec3) {
    let first_cube = scene.cubes.len();
    let first_ellipsoid = scene.ellipsoids.len();
    let first_capsule = scene.capsules.len();
    let first_triangle = scene.triangles.len();
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
    add_sculpted_toad_volume(
        scene,
        base + Vec3::new(0.0, 2.75, -0.05),
        Vec3::new(3.34, 1.92, 1.78),
        skin,
        SculptProfile::body(0.16, 0.045),
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
    add_sculpted_toad_volume(
        scene,
        base + Vec3::new(0.0, 5.66, 0.48),
        Vec3::new(3.36, 1.32, 1.56),
        skin,
        SculptProfile::head(0.56, 0.28, 0.065),
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 5.25, 1.84),
        Vec3::new(2.52, 0.68, 0.56),
        0.25,
        skin,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.56, 6.20, 1.89),
            Vec3::new(0.43, 0.25, 0.17),
            0.16,
            eye,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.92, 6.58, 2.02),
            base + Vec3::new(side * 2.00, 6.72, 1.92),
            0.18,
            skin,
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
    add_voxel_curve(
        scene,
        base + Vec3::new(-1.58, 4.82, 2.18),
        base + Vec3::new(0.0, 4.58, 2.26),
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
    add_gamabunta_details(scene, base, skin, robe, pale, ink, eye, 6);
    add_mature_toad_face(
        scene,
        base,
        skin,
        ink,
        6.20,
        1.56,
        2.08,
        ToadFaceStyle::Scarred,
    );
    add_gamabunta_limbs(scene, base, skin);
    add_toad_haori_back(
        scene,
        base + Vec3::new(0.0, 3.45, -1.66),
        Vec3::new(2.62, 1.78, 0.24),
        robe,
        pale,
        ink,
        0.68,
    );
    scale_added_geometry(
        scene,
        first_cube,
        first_ellipsoid,
        first_capsule,
        first_triangle,
        base,
        Vec3::new(1.78, 1.92, 1.78),
    );
}

pub fn add_gamaken(scene: &mut Scene, base: Vec3) {
    let first_cube = scene.cubes.len();
    let first_ellipsoid = scene.ellipsoids.len();
    let first_capsule = scene.capsules.len();
    let first_triangle = scene.triangles.len();
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
    add_sculpted_toad_volume(
        scene,
        base + Vec3::new(0.0, 2.65, -0.05),
        Vec3::new(2.62, 1.78, 1.52),
        skin,
        SculptProfile::body(0.22, 0.070),
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
    add_sculpted_toad_volume(
        scene,
        base + Vec3::new(0.0, 5.28, 0.48),
        Vec3::new(2.58, 1.20, 1.40),
        skin,
        SculptProfile::head(0.62, 0.24, 0.095),
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 4.95, 1.63),
        Vec3::new(1.92, 0.62, 0.51),
        0.23,
        skin,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.16, 5.78, 1.72),
            Vec3::new(0.34, 0.22, 0.15),
            0.14,
            eye,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.70, 6.10, 1.78),
            base + Vec3::new(side * 1.50, 6.25, 1.65),
            0.16,
            skin,
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
    add_voxel_curve(
        scene,
        base + Vec3::new(-1.10, 4.62, 2.14),
        base + Vec3::new(0.0, 4.42, 2.20),
        base + Vec3::new(1.10, 4.62, 2.14),
        0.14,
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
    add_gamaken_details(scene, base, skin, robe, pale, ink, eye);
    add_mature_toad_face(
        scene,
        base,
        skin,
        ink,
        5.78,
        1.16,
        1.98,
        ToadFaceStyle::Warty,
    );
    add_gamaken_limbs(scene, base, skin, metal);
    add_toad_haori_back(
        scene,
        base + Vec3::new(0.0, 3.30, -1.42),
        Vec3::new(2.02, 1.62, 0.22),
        robe,
        pale,
        ink,
        0.56,
    );
    scale_added_geometry(
        scene,
        first_cube,
        first_ellipsoid,
        first_capsule,
        first_triangle,
        base,
        Vec3::new(1.78, 1.92, 1.78),
    );
    rotate_added_geometry(
        scene,
        first_cube,
        first_ellipsoid,
        first_capsule,
        first_triangle,
        base,
        0.10,
    );
}

pub fn add_gamahiro(scene: &mut Scene, base: Vec3) {
    let first_cube = scene.cubes.len();
    let first_ellipsoid = scene.ellipsoids.len();
    let first_capsule = scene.capsules.len();
    let first_triangle = scene.triangles.len();
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
    add_sculpted_toad_volume(
        scene,
        base + Vec3::new(0.0, 2.72, -0.04),
        Vec3::new(2.78, 1.86, 1.56),
        skin,
        SculptProfile::body(0.12, 0.030),
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
    add_sculpted_toad_volume(
        scene,
        base + Vec3::new(0.0, 5.52, 0.46),
        Vec3::new(2.70, 1.24, 1.45),
        skin,
        SculptProfile::head(0.46, 0.20, 0.045),
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 5.08, 1.68),
        Vec3::new(2.02, 0.64, 0.53),
        0.23,
        skin,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.22, 5.98, 1.77),
            Vec3::new(0.35, 0.22, 0.15),
            0.14,
            eye,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.74, 6.28, 1.82),
            base + Vec3::new(side * 1.56, 6.40, 1.68),
            0.16,
            skin,
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
    add_voxel_curve(
        scene,
        base + Vec3::new(-1.18, 4.75, 2.18),
        base + Vec3::new(0.0, 4.54, 2.24),
        base + Vec3::new(1.18, 4.75, 2.18),
        0.14,
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
    add_gamahiro_details(scene, base, skin, pale, ink, eye, orange, 22);
    add_mature_toad_face(
        scene,
        base,
        skin,
        ink,
        5.98,
        1.22,
        2.03,
        ToadFaceStyle::Stern,
    );
    add_gamahiro_limbs(scene, base, skin);
    add_toad_haori_back(
        scene,
        base + Vec3::new(0.0, 3.36, -1.48),
        Vec3::new(2.14, 1.68, 0.23),
        robe,
        pale,
        ink,
        0.58,
    );
    scale_added_geometry(
        scene,
        first_cube,
        first_ellipsoid,
        first_capsule,
        first_triangle,
        base,
        Vec3::new(1.78, 1.92, 1.78),
    );
    rotate_added_geometry(
        scene,
        first_cube,
        first_ellipsoid,
        first_capsule,
        first_triangle,
        base,
        -0.10,
    );
}

fn add_gamabunta_details(
    scene: &mut Scene,
    base: Vec3,
    skin: usize,
    robe: usize,
    pale: usize,
    ink: usize,
    eye: usize,
    orange: usize,
) {
    // Pipa larga de Gamabunta, inclinada desde la comisura izquierda.
    add_voxel_segment(
        scene,
        base + Vec3::new(-1.42, 4.84, 2.22),
        base + Vec3::new(-3.22, 4.08, 2.38),
        0.16,
        13,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(-3.40, 4.02, 2.39),
        Vec3::new(0.34, 0.42, 0.32),
        0.13,
        13,
    );
    add_block(
        scene,
        base + Vec3::new(-3.40, 4.39, 2.39),
        Vec3::new(0.43, 0.13, 0.43),
        ink,
    );

    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 7.02, 1.28),
        Vec3::new(0.30, 0.42, 0.26),
        0.11,
        orange,
    );
    for y in [6.82, 7.02, 7.22] {
        let width = 0.48 - (y - 6.82) * 0.42;
        add_block(
            scene,
            base + Vec3::new(0.0, y, 1.53),
            Vec3::new(width, 0.10, 0.10),
            ink,
        );
    }

    for side in [-1.0, 1.0] {
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 2.18, 4.72, 1.42),
            base + Vec3::new(side * 1.02, 3.62, 1.86),
            0.22,
            pale,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.92, 6.05, 2.12),
            base + Vec3::new(side * 1.96, 6.00, 2.06),
            0.13,
            skin,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 1.02, 5.30, 2.31),
            base + Vec3::new(side * 1.82, 5.05, 2.20),
            0.09,
            ink,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 2.48, 4.30, 1.65),
            Vec3::new(0.36, 0.42, 0.16),
            0.13,
            robe,
        );
        add_block(
            scene,
            base + Vec3::new(side * 2.48, 4.30, 1.84),
            Vec3::new(0.22, 0.22, 0.08),
            pale,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.48, 6.30, 2.01),
            Vec3::new(0.34, 0.22, 0.10),
            0.11,
            eye,
        );
        add_block(
            scene,
            base + Vec3::new(side * 1.48, 6.30, 2.13),
            Vec3::new(0.12, 0.34, 0.07),
            ink,
        );
    }
}

fn add_gamaken_details(
    scene: &mut Scene,
    base: Vec3,
    skin: usize,
    robe: usize,
    pale: usize,
    ink: usize,
    eye: usize,
) {
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 4.78, 1.93),
        Vec3::new(1.28, 0.34, 0.26),
        0.17,
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 4.53, 2.18),
        Vec3::new(1.55, 0.11, 0.08),
        ink,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 6.47, 1.18),
        Vec3::new(0.28, 0.24, 0.22),
        0.11,
        pale,
    );
    for side in [-1.0, 1.0] {
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 1.78, 4.48, 1.34),
            base + Vec3::new(side * 0.82, 3.55, 1.68),
            0.19,
            pale,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.58, 5.60, 2.04),
            base + Vec3::new(side * 1.52, 5.42, 1.99),
            0.09,
            ink,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.08, 5.86, 1.86),
            Vec3::new(0.30, 0.19, 0.10),
            0.10,
            eye,
        );
        add_block(
            scene,
            base + Vec3::new(side * 1.08, 5.86, 1.97),
            Vec3::new(0.10, 0.30, 0.06),
            ink,
        );
        add_block(
            scene,
            base + Vec3::new(side * 1.80, 3.76, 1.58),
            Vec3::new(0.18, 1.12, 0.10),
            robe,
        );
    }
}

fn add_gamahiro_details(
    scene: &mut Scene,
    base: Vec3,
    skin: usize,
    pale: usize,
    ink: usize,
    eye: usize,
    orange: usize,
    marking: usize,
) {
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 4.90, 1.96),
        Vec3::new(1.42, 0.36, 0.27),
        0.17,
        skin,
    );
    add_voxel_segment(
        scene,
        base + Vec3::new(-1.30, 4.65, 2.19),
        base + Vec3::new(1.30, 4.65, 2.19),
        0.11,
        ink,
    );
    add_voxel_segment(
        scene,
        base + Vec3::new(-0.90, 4.48, 2.08),
        base + Vec3::new(0.90, 4.48, 2.08),
        0.10,
        pale,
    );

    add_block(
        scene,
        base + Vec3::new(0.0, 6.12, 1.93),
        Vec3::new(0.62, 0.22, 0.11),
        marking,
    );
    for side in [-1.0, 1.0] {
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.28, 6.15, 1.94),
            base + Vec3::new(side * 1.62, 6.30, 1.86),
            0.24,
            marking,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 1.62, 6.30, 1.84),
            base + Vec3::new(side * 1.90, 5.77, 1.83),
            0.20,
            marking,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.13, 6.03, 2.01),
            Vec3::new(0.31, 0.20, 0.10),
            0.10,
            eye,
        );
        add_block(
            scene,
            base + Vec3::new(side * 1.13, 6.03, 2.13),
            Vec3::new(0.10, 0.31, 0.06),
            ink,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 1.85, 4.62, 1.38),
            base + Vec3::new(side * 0.88, 3.62, 1.72),
            0.19,
            pale,
        );
    }
    for x in [-0.72, -0.24, 0.24, 0.72] {
        add_block(
            scene,
            base + Vec3::new(x, 1.73, 1.96),
            Vec3::new(0.18, 0.42, 0.12),
            orange,
        );
    }
}

#[derive(Clone, Copy)]
enum ToadFaceStyle {
    Scarred,
    Warty,
    Stern,
}

fn add_mature_toad_face(
    scene: &mut Scene,
    base: Vec3,
    skin: usize,
    ink: usize,
    eye_y: f32,
    eye_x: f32,
    front_z: f32,
    style: ToadFaceStyle,
) {
    // Heavy asymmetric eyelids reduce the round mascot-like eyes and create a stern gaze.
    for side in [-1.0, 1.0] {
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.48, eye_y + 0.08, front_z + 0.01),
            base + Vec3::new(side * (eye_x + 0.62), eye_y + 0.42, front_z - 0.03),
            0.25,
            skin,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.72, eye_y - 0.24, front_z + 0.02),
            base + Vec3::new(side * (eye_x + 0.42), eye_y - 0.18, front_z - 0.02),
            0.14,
            skin,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * (eye_x + 0.88), eye_y - 0.88, front_z - 0.28),
            Vec3::new(0.74, 0.76, 0.34),
            0.14,
            skin,
        );
    }

    // Nostrils, double mouth fold and hanging jaw give the face weight at close range.
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 0.62, eye_y - 0.75, front_z + 0.20),
            Vec3::new(0.16, 0.10, 0.055),
            0.06,
            ink,
        );
    }
    add_voxel_curve(
        scene,
        base + Vec3::new(-2.02, eye_y - 1.34, front_z + 0.08),
        base + Vec3::new(0.0, eye_y - 1.62, front_z + 0.15),
        base + Vec3::new(2.02, eye_y - 1.34, front_z + 0.08),
        0.12,
        ink,
    );
    add_voxel_curve(
        scene,
        base + Vec3::new(-1.55, eye_y - 1.55, front_z),
        base + Vec3::new(0.0, eye_y - 1.76, front_z + 0.06),
        base + Vec3::new(1.55, eye_y - 1.55, front_z),
        0.065,
        ink,
    );

    let warts = match style {
        ToadFaceStyle::Scarred => [
            (-2.34, 0.34, 0.22),
            (-1.82, 0.78, 0.16),
            (0.12, 0.88, 0.18),
            (2.18, 0.52, 0.20),
            (2.55, -0.18, 0.15),
            (-2.62, -0.26, 0.17),
        ],
        ToadFaceStyle::Warty => [
            (-1.98, 0.48, 0.24),
            (-1.46, 0.96, 0.18),
            (-0.18, 0.78, 0.20),
            (1.14, 0.92, 0.17),
            (1.94, 0.38, 0.22),
            (2.18, -0.38, 0.16),
        ],
        ToadFaceStyle::Stern => [
            (-2.18, 0.16, 0.17),
            (-1.70, 0.72, 0.14),
            (-0.30, 0.92, 0.15),
            (0.72, 0.82, 0.13),
            (1.84, 0.56, 0.16),
            (2.28, -0.22, 0.14),
        ],
    };
    for (x, y, radius) in warts {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(x, eye_y + y, front_z - 0.02),
            Vec3::new(radius, radius * 0.78, radius * 0.55),
            0.07,
            skin,
        );
    }

    match style {
        ToadFaceStyle::Scarred => {
            add_voxel_segment(
                scene,
                base + Vec3::new(0.70, eye_y + 0.58, front_z + 0.10),
                base + Vec3::new(0.42, eye_y - 0.08, front_z + 0.16),
                0.075,
                ink,
            );
            add_voxel_segment(
                scene,
                base + Vec3::new(0.42, eye_y - 0.08, front_z + 0.16),
                base + Vec3::new(0.86, eye_y - 0.62, front_z + 0.12),
                0.065,
                ink,
            );
        }
        ToadFaceStyle::Warty => {
            for side in [-1.0, 1.0] {
                add_voxel_ellipsoid(
                    scene,
                    base + Vec3::new(side * 1.78, eye_y + 0.68, front_z - 0.20),
                    Vec3::new(0.30, 0.46, 0.26),
                    0.10,
                    skin,
                );
            }
        }
        ToadFaceStyle::Stern => {
            for side in [-1.0, 1.0] {
                add_voxel_segment(
                    scene,
                    base + Vec3::new(side * 1.70, eye_y - 0.64, front_z + 0.03),
                    base + Vec3::new(side * 2.18, eye_y - 1.18, front_z - 0.08),
                    0.10,
                    skin,
                );
            }
        }
    }
}

fn add_gamabunta_limbs(scene: &mut Scene, base: Vec3, skin: usize) {
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 3.00, 3.42, 0.72),
            Vec3::new(0.84, 1.28, 0.88),
            0.28,
            skin,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 3.18, 2.12, 1.36),
            Vec3::new(0.70, 0.92, 0.84),
            0.26,
            skin,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 3.06, 1.24, 2.02),
            Vec3::new(1.02, 0.46, 0.80),
            0.24,
            skin,
        );
        for finger in [-0.48, 0.0, 0.48] {
            add_voxel_segment(
                scene,
                base + Vec3::new(side * 3.06 + finger, 1.10, 2.28),
                base + Vec3::new(side * 3.06 + finger, 0.82, 3.10),
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

pub fn add_gamakichi(scene: &mut Scene, base: Vec3) {
    let first_cube = scene.cubes.len();
    let first_ellipsoid = scene.ellipsoids.len();
    let first_capsule = scene.capsules.len();
    let first_triangle = scene.triangles.len();
    let skin = 23;
    let robe = 5;
    let pale = 15;
    let ink = 10;
    let eye = 14;

    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 0.72, 0.30, 0.38),
            Vec3::new(0.58, 0.28, 0.62),
            0.14,
            skin,
        );
    }
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 1.02, 0.0),
        Vec3::new(1.12, 0.88, 0.76),
        0.16,
        skin,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 1.02, 0.73),
        Vec3::new(0.62, 0.58, 0.18),
        0.12,
        pale,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 0.91, 1.12, 0.22),
            Vec3::new(0.34, 0.68, 0.44),
            0.13,
            robe,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.93, 1.12, 0.48),
            base + Vec3::new(side * 1.08, 0.52, 0.78),
            0.20,
            skin,
        );
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 1.02, 0.43, 0.86),
            Vec3::new(0.40, 0.20, 0.34),
            0.11,
            skin,
        );
    }

    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 2.10, 0.16),
        Vec3::new(1.20, 0.68, 0.76),
        0.14,
        skin,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 1.90, 0.87),
        Vec3::new(0.86, 0.36, 0.30),
        0.12,
        skin,
    );
    for side in [-1.0, 1.0] {
        add_voxel_ellipsoid(
            scene,
            base + Vec3::new(side * 0.58, 2.34, 0.86),
            Vec3::new(0.30, 0.22, 0.13),
            0.09,
            eye,
        );
        add_voxel_curve(
            scene,
            base + Vec3::new(side * 0.30, 2.50, 0.94),
            base + Vec3::new(side * 0.58, 2.62, 0.96),
            base + Vec3::new(side * 0.88, 2.48, 0.90),
            0.09,
            ink,
        );
        add_block(
            scene,
            base + Vec3::new(side * 0.58, 2.34, 1.01),
            Vec3::new(0.08, 0.22, 0.06),
            ink,
        );
        add_block(
            scene,
            base + Vec3::new(side * 0.33, 2.02, 1.15),
            Vec3::new(0.10, 0.09, 0.06),
            ink,
        );
    }
    add_voxel_curve(
        scene,
        base + Vec3::new(-0.72, 1.76, 1.09),
        base + Vec3::new(0.0, 1.62, 1.14),
        base + Vec3::new(0.72, 1.76, 1.09),
        0.10,
        ink,
    );
    add_voxel_ellipsoid(
        scene,
        base + Vec3::new(0.0, 2.72, 0.02),
        Vec3::new(0.76, 0.20, 0.58),
        0.11,
        skin,
    );
    add_toad_haori_back(
        scene,
        base + Vec3::new(0.0, 1.12, -0.72),
        Vec3::new(0.92, 0.72, 0.13),
        robe,
        pale,
        ink,
        0.28,
    );
    scale_added_geometry(
        scene,
        first_cube,
        first_ellipsoid,
        first_capsule,
        first_triangle,
        base,
        Vec3::new(1.32, 1.45, 1.32),
    );
}

fn add_toad_haori_back(
    scene: &mut Scene,
    center: Vec3,
    radii: Vec3,
    robe: usize,
    emblem: usize,
    ink: usize,
    emblem_radius: f32,
) {
    add_voxel_ellipsoid(scene, center, radii, 0.18, robe);

    let surface_z = center.z - radii.z - 0.035;
    add_voxel_ellipsoid(
        scene,
        Vec3::new(center.x, center.y, surface_z),
        Vec3::new(emblem_radius, emblem_radius * 0.92, 0.065),
        0.09,
        emblem,
    );
    add_voxel_segment(
        scene,
        Vec3::new(center.x, center.y - emblem_radius * 0.48, surface_z - 0.08),
        Vec3::new(center.x, center.y + emblem_radius * 0.52, surface_z - 0.08),
        0.085,
        ink,
    );
    add_voxel_segment(
        scene,
        Vec3::new(
            center.x - emblem_radius * 0.42,
            center.y + emblem_radius * 0.12,
            surface_z - 0.08,
        ),
        Vec3::new(
            center.x + emblem_radius * 0.42,
            center.y + emblem_radius * 0.12,
            surface_z - 0.08,
        ),
        0.085,
        ink,
    );

    for side in [-1.0, 1.0] {
        add_voxel_segment(
            scene,
            Vec3::new(
                center.x + side * radii.x * 0.72,
                center.y + radii.y * 0.72,
                surface_z,
            ),
            Vec3::new(
                center.x + side * radii.x * 0.24,
                center.y + radii.y * 0.42,
                surface_z - 0.04,
            ),
            0.11,
            emblem,
        );
    }
}

pub fn add_naruto_sage(scene: &mut Scene, base: Vec3) {
    let first_cube = scene.cubes.len();
    let first_ellipsoid = scene.ellipsoids.len();
    let first_capsule = scene.capsules.len();
    let first_triangle = scene.triangles.len();
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

    // Translucent chakra wisps keep refraction visible without hiding the silhouette.
    for (start, end) in [
        (Vec3::new(-1.06, 0.78, -0.18), Vec3::new(-1.16, 1.55, -0.14)),
        (Vec3::new(1.10, 1.48, -0.12), Vec3::new(1.18, 2.18, -0.08)),
        (Vec3::new(-0.83, 2.70, -0.10), Vec3::new(-0.92, 3.32, -0.06)),
        (Vec3::new(0.62, 3.72, -0.12), Vec3::new(0.52, 4.12, -0.10)),
        (Vec3::new(-1.18, 1.82, -0.16), Vec3::new(-1.34, 2.48, -0.08)),
        (Vec3::new(0.96, 2.52, -0.15), Vec3::new(1.08, 3.18, -0.06)),
    ] {
        add_voxel_segment(scene, base + start, base + end, 0.15, chakra);
    }

    add_naruto_costume_details(scene, base, orange, cloak, ink, metal, hair, rope);

    const NARUTO_SCALE: f32 = 0.50;
    for cube in &mut scene.cubes[first_cube..] {
        cube.min = base + (cube.min - base) * NARUTO_SCALE;
        cube.max = base + (cube.max - base) * NARUTO_SCALE;
    }
    for ellipsoid in &mut scene.ellipsoids[first_ellipsoid..] {
        ellipsoid.center = base + (ellipsoid.center - base) * NARUTO_SCALE;
        ellipsoid.radii = ellipsoid.radii * NARUTO_SCALE;
    }
    for capsule in &mut scene.capsules[first_capsule..] {
        capsule.start = base + (capsule.start - base) * NARUTO_SCALE;
        capsule.end = base + (capsule.end - base) * NARUTO_SCALE;
        capsule.radius *= NARUTO_SCALE;
    }
    for triangle in &mut scene.triangles[first_triangle..] {
        for vertex in &mut triangle.vertices {
            *vertex = base + (*vertex - base) * NARUTO_SCALE;
        }
    }
}

fn add_naruto_costume_details(
    scene: &mut Scene,
    base: Vec3,
    orange: usize,
    cloak: usize,
    ink: usize,
    metal: usize,
    hair: usize,
    rope: usize,
) {
    // Emblema espiral de Konoha sobre la placa metalica del protector.
    let leaf_center = base + Vec3::new(0.0, 3.40, 0.655);
    for index in 0..10 {
        let angle = 2.0 * PI * index as f32 / 10.0;
        add_block(
            scene,
            leaf_center + Vec3::new(angle.cos() * 0.105, angle.sin() * 0.075, 0.0),
            Vec3::new(0.055, 0.055, 0.025),
            ink,
        );
    }
    add_voxel_segment(
        scene,
        leaf_center + Vec3::new(0.08, -0.02, 0.0),
        leaf_center + Vec3::new(0.22, 0.08, 0.0),
        0.045,
        ink,
    );

    // Ribete de llamas de la capa de Hokage, construido con piezas pequenas.
    for index in -5i32..=5 {
        let x = index as f32 * 0.15;
        let flame_height = if index.abs() % 2 == 0 { 0.25 } else { 0.15 };
        add_block(
            scene,
            base + Vec3::new(x, 0.56 + flame_height * 0.5, -0.535),
            Vec3::new(0.12, flame_height, 0.055),
            orange,
        );
    }

    for side in [-1.0, 1.0] {
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.32, 2.52, 0.24),
            base + Vec3::new(side * 0.68, 2.78, 0.10),
            0.17,
            cloak,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.62, 3.42, -0.12),
            base + Vec3::new(side * 0.92, 3.20, -0.38),
            0.11,
            ink,
        );
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.58, 3.55, -0.06),
            base + Vec3::new(side * 0.82, 3.80, -0.14),
            0.13,
            hair,
        );
        add_block(
            scene,
            base + Vec3::new(side * 0.43, 0.70, 0.36),
            Vec3::new(0.34, 0.10, 0.08),
            ink,
        );
    }

    for x in [-0.66, -0.40, 0.40, 0.66] {
        add_voxel_segment(
            scene,
            base + Vec3::new(x, 0.54, -0.53),
            base + Vec3::new(x * 0.82, 0.90, -0.53),
            0.10,
            ink,
        );
    }
    add_block(
        scene,
        base + Vec3::new(0.0, 1.36, 0.43),
        Vec3::new(0.82, 0.15, 0.10),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 1.36, 0.50),
        Vec3::new(0.22, 0.20, 0.06),
        metal,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 1.62, 0.49),
        Vec3::new(0.32, 0.42, 0.07),
        orange,
    );
    for x in [-0.62, 0.0, 0.62] {
        add_block(
            scene,
            base + Vec3::new(x, 1.92, -0.91),
            Vec3::new(0.12, 0.58, 0.12),
            rope,
        );
    }

    // Broche frontal y costuras del cuello para conservar detalle al reducirlo.
    add_block(
        scene,
        base + Vec3::new(0.0, 2.34, 0.46),
        Vec3::new(0.18, 0.18, 0.07),
        metal,
    );
    for side in [-1.0, 1.0] {
        add_voxel_segment(
            scene,
            base + Vec3::new(side * 0.12, 2.48, 0.43),
            base + Vec3::new(side * 0.42, 2.22, 0.45),
            0.045,
            orange,
        );
    }
}

pub fn add_summoning_clouds(scene: &mut Scene) {
    add_summoning_seal(scene);

    for (x, y, z, scale) in [
        (-10.0, 0.65, 2.8, 1.2),
        (-7.0, 0.65, 3.5, 1.4),
        (-3.8, 0.65, 3.8, 1.15),
        (0.0, 0.65, 4.0, 1.45),
        (3.7, 0.65, 3.7, 1.2),
        (7.0, 0.65, 3.4, 1.4),
        (10.0, 0.65, 2.7, 1.2),
        (-9.4, 0.65, -2.5, 0.9),
        (9.4, 0.65, -2.5, 0.9),
        (-12.2, 1.00, 4.1, 1.20),
        (-8.8, 1.10, 5.1, 1.35),
        (-5.1, 1.20, 5.5, 1.42),
        (-1.8, 1.05, 5.8, 1.28),
        (1.7, 1.12, 5.7, 1.34),
        (5.2, 1.18, 5.4, 1.40),
        (8.9, 1.08, 4.9, 1.34),
        (12.1, 0.98, 4.0, 1.18),
    ] {
        add_cloud_cluster(scene, Vec3::new(x, y, z), scale);
    }
}

fn add_summoning_seal(scene: &mut Scene) {
    let chakra = 12;
    let center = Vec3::new(0.0, 0.30, -0.45);

    for index in 0..160 {
        let angle = 2.0 * PI * index as f32 / 160.0;
        let radius = if index % 2 == 0 { 11.15 } else { 11.05 };
        add_block(
            scene,
            center + Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius),
            Vec3::new(0.13, 0.040, 0.13),
            chakra,
        );
    }

    for spoke in 0..8 {
        let angle = 2.0 * PI * spoke as f32 / 8.0 + PI * 0.125;
        let direction = Vec3::new(angle.cos(), 0.0, angle.sin());
        add_voxel_segment(
            scene,
            center + direction * 8.8,
            center + direction * 10.6,
            0.10,
            chakra,
        );
    }
}

fn add_cloud_cluster(scene: &mut Scene, base: Vec3, scale: f32) {
    for (offset, radii) in [
        (Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.55, 0.48, 0.90)),
        (Vec3::new(-1.10, 0.28, 0.08), Vec3::new(0.92, 0.58, 0.76)),
        (Vec3::new(1.05, 0.34, -0.06), Vec3::new(1.00, 0.64, 0.80)),
        (Vec3::new(-0.38, 0.68, 0.02), Vec3::new(0.88, 0.62, 0.70)),
        (Vec3::new(0.55, 0.76, -0.04), Vec3::new(0.82, 0.68, 0.66)),
        (Vec3::new(0.0, 1.10, 0.0), Vec3::new(0.56, 0.72, 0.52)),
    ] {
        add_voxel_ellipsoid(scene, base + offset * scale, radii * scale, 0.28 * scale, 1);
    }
}

#[derive(Clone, Copy)]
struct SculptProfile {
    jaw: f32,
    muzzle: f32,
    asymmetry: f32,
    lower_weight: f32,
}

impl SculptProfile {
    fn body(lower_weight: f32, asymmetry: f32) -> Self {
        Self {
            jaw: 0.08,
            muzzle: 0.04,
            asymmetry,
            lower_weight,
        }
    }

    fn head(jaw: f32, muzzle: f32, asymmetry: f32) -> Self {
        Self {
            jaw,
            muzzle,
            asymmetry,
            lower_weight: 0.10,
        }
    }
}

fn add_sculpted_toad_volume(
    scene: &mut Scene,
    center: Vec3,
    radii: Vec3,
    material: usize,
    profile: SculptProfile,
) {
    const LATITUDE_STEPS: usize = 16;
    const LONGITUDE_STEPS: usize = 30;
    let row = LONGITUDE_STEPS + 1;
    let mut vertices = Vec::with_capacity((LATITUDE_STEPS + 1) * row);

    for latitude in 0..=LATITUDE_STEPS {
        let v = -PI * 0.5 + PI * latitude as f32 / LATITUDE_STEPS as f32;
        let vertical = v.sin();
        let ring = v.cos();
        for longitude in 0..=LONGITUDE_STEPS {
            let u = 2.0 * PI * longitude as f32 / LONGITUDE_STEPS as f32;
            let unit = Vec3::new(ring * u.cos(), vertical, ring * u.sin());
            let front = unit.z.max(0.0).powf(1.35);
            let lower = (-unit.y).max(0.0).powf(1.25);
            let upper = unit.y.max(0.0);
            let jaw_width = 1.0 + profile.jaw * front * lower;
            let weighted_base = 1.0 + profile.lower_weight * lower * (1.0 - front * 0.25);
            let uneven = 1.0
                + profile.asymmetry
                    * (unit.x * 4.7 + unit.y * 3.1 + unit.z * 2.3).sin()
                    * (0.35 + front * 0.65);
            let flattened_y = unit.y.signum() * unit.y.abs().powf(0.88);
            let local = Vec3::new(
                unit.x * radii.x * jaw_width * weighted_base * uneven,
                flattened_y * radii.y * (1.0 - upper * 0.05 + lower * 0.04),
                unit.z * radii.z * (1.0 + profile.muzzle * front * (0.70 + lower * 0.30)),
            );
            vertices.push(center + local);
        }
    }

    let mut faces = Vec::with_capacity(LATITUDE_STEPS * LONGITUDE_STEPS * 2);
    let mut normals = vec![Vec3::default(); vertices.len()];
    for latitude in 0..LATITUDE_STEPS {
        for longitude in 0..LONGITUDE_STEPS {
            let a = latitude * row + longitude;
            let b = a + 1;
            let c = a + row;
            let d = c + 1;
            add_sculpted_face(&vertices, &mut normals, &mut faces, center, [a, c, b]);
            add_sculpted_face(&vertices, &mut normals, &mut faces, center, [b, c, d]);
        }
    }

    for (index, normal) in normals.iter_mut().enumerate() {
        if normal.length() < 0.0001 {
            *normal = (vertices[index] - center).normalized();
        } else {
            *normal = normal.normalized();
        }
    }
    for face in faces {
        scene.triangles.push(Triangle {
            vertices: [vertices[face[0]], vertices[face[1]], vertices[face[2]]],
            normals: [normals[face[0]], normals[face[1]], normals[face[2]]],
            material,
        });
    }
}

fn add_sculpted_face(
    vertices: &[Vec3],
    normals: &mut [Vec3],
    faces: &mut Vec<[usize; 3]>,
    center: Vec3,
    mut face: [usize; 3],
) {
    let mut normal =
        (vertices[face[1]] - vertices[face[0]]).cross(vertices[face[2]] - vertices[face[0]]);
    let face_center = (vertices[face[0]] + vertices[face[1]] + vertices[face[2]]) / 3.0;
    if normal.dot(face_center - center) < 0.0 {
        face.swap(1, 2);
        normal = -normal;
    }
    if normal.length() < 0.000_001 {
        return;
    }
    let normal = normal.normalized();
    for index in face {
        normals[index] += normal;
    }
    faces.push(face);
}

fn add_voxel_ellipsoid(scene: &mut Scene, center: Vec3, radii: Vec3, cell: f32, material: usize) {
    let _detail_hint = cell;
    scene.ellipsoids.push(Ellipsoid {
        center,
        radii,
        material,
    });
}

fn add_voxel_segment(scene: &mut Scene, start: Vec3, end: Vec3, thickness: f32, material: usize) {
    scene.capsules.push(Capsule {
        start,
        end,
        radius: thickness * 0.5,
        material,
    });
}

fn add_voxel_curve(
    scene: &mut Scene,
    start: Vec3,
    control: Vec3,
    end: Vec3,
    thickness: f32,
    material: usize,
) {
    let estimated_length = (control - start).length() + (end - control).length();
    let steps = (estimated_length / (thickness * 0.62)).ceil().max(2.0) as usize;
    let mut previous = start;
    for step in 1..=steps {
        let t = step as f32 / steps as f32;
        let inverse = 1.0 - t;
        let point = start * (inverse * inverse) + control * (2.0 * inverse * t) + end * (t * t);
        scene.capsules.push(Capsule {
            start: previous,
            end: point,
            radius: thickness * 0.5,
            material,
        });
        previous = point;
    }
}

fn add_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    scene.cubes.push(Cube {
        min: center - size * 0.5,
        max: center + size * 0.5,
        material,
        smooth_normal: None,
    });
}

fn scale_added_geometry(
    scene: &mut Scene,
    first_cube: usize,
    first_ellipsoid: usize,
    first_capsule: usize,
    first_triangle: usize,
    origin: Vec3,
    scale: Vec3,
) {
    for cube in &mut scene.cubes[first_cube..] {
        cube.min = origin + (cube.min - origin).hadamard(scale);
        cube.max = origin + (cube.max - origin).hadamard(scale);
    }
    for ellipsoid in &mut scene.ellipsoids[first_ellipsoid..] {
        ellipsoid.center = origin + (ellipsoid.center - origin).hadamard(scale);
        ellipsoid.radii = ellipsoid.radii.hadamard(scale);
    }
    let radius_scale = (scale.x + scale.y + scale.z) / 3.0;
    for capsule in &mut scene.capsules[first_capsule..] {
        capsule.start = origin + (capsule.start - origin).hadamard(scale);
        capsule.end = origin + (capsule.end - origin).hadamard(scale);
        capsule.radius *= radius_scale;
    }
    for triangle in &mut scene.triangles[first_triangle..] {
        for vertex in &mut triangle.vertices {
            *vertex = origin + (*vertex - origin).hadamard(scale);
        }
        for normal in &mut triangle.normals {
            *normal =
                Vec3::new(normal.x / scale.x, normal.y / scale.y, normal.z / scale.z).normalized();
        }
    }
}

fn rotate_added_geometry(
    scene: &mut Scene,
    first_cube: usize,
    first_ellipsoid: usize,
    first_capsule: usize,
    first_triangle: usize,
    origin: Vec3,
    yaw: f32,
) {
    let cosine = yaw.cos();
    let sine = yaw.sin();
    let rotate_point = |point: Vec3| {
        let local = point - origin;
        origin
            + Vec3::new(
                local.x * cosine + local.z * sine,
                local.y,
                -local.x * sine + local.z * cosine,
            )
    };
    let rotate_normal = |normal: Vec3| {
        Vec3::new(
            normal.x * cosine + normal.z * sine,
            normal.y,
            -normal.x * sine + normal.z * cosine,
        )
        .normalized()
    };

    for cube in &mut scene.cubes[first_cube..] {
        let center = rotate_point((cube.min + cube.max) * 0.5);
        let half_size = (cube.max - cube.min) * 0.5;
        cube.min = center - half_size;
        cube.max = center + half_size;
        if let Some(normal) = &mut cube.smooth_normal {
            *normal = rotate_normal(*normal);
        }
    }
    for ellipsoid in &mut scene.ellipsoids[first_ellipsoid..] {
        ellipsoid.center = rotate_point(ellipsoid.center);
    }
    for capsule in &mut scene.capsules[first_capsule..] {
        capsule.start = rotate_point(capsule.start);
        capsule.end = rotate_point(capsule.end);
    }
    for triangle in &mut scene.triangles[first_triangle..] {
        for vertex in &mut triangle.vertices {
            *vertex = rotate_point(*vertex);
        }
        for normal in &mut triangle.normals {
            *normal = rotate_normal(*normal);
        }
    }
}
