use crate::math::Vec3;
use crate::scene::{Capsule, Cube, Ellipsoid, Scene};

const ROBE: usize = 5;
const CLOUD_RED: usize = 7;
const INK: usize = 10;
const METAL: usize = 11;
const CLOUD_WHITE: usize = 15;
const PAIN_SKIN: usize = 26;
const RINNEGAN: usize = 27;
const PAIN_HAIR: usize = 28;
const CRATER_EARTH: usize = 17;

pub fn add_six_paths(scene: &mut Scene) {
    let paths = [
        (Vec3::new(-12.0, -7.72, 6.6), 0usize),
        (Vec3::new(-7.4, -7.72, 10.6), 1usize),
        (Vec3::new(-2.5, -7.72, 12.6), 2usize),
        (Vec3::new(2.5, -7.72, 12.6), 3usize),
        (Vec3::new(7.4, -7.72, 10.6), 4usize),
        (Vec3::new(12.0, -7.72, 6.6), 5usize),
    ];

    clear_staging_areas(scene, &paths);
    for (base, style) in paths {
        add_block(
            scene,
            base + Vec3::new(0.0, -0.09, 0.0),
            Vec3::new(1.72, 0.18, 1.52),
            CRATER_EARTH,
        );
        add_path(scene, base, style);
    }
}

fn clear_staging_areas(scene: &mut Scene, paths: &[(Vec3, usize); 6]) {
    scene.cubes.retain(|cube| {
        let size = cube.max - cube.min;
        if size.x > 12.0 || size.z > 12.0 {
            return true;
        }
        let center = (cube.min + cube.max) * 0.5;
        !paths.iter().any(|(base, _)| {
            let dx = center.x - base.x;
            let dz = center.z - base.z;
            dx * dx + dz * dz < 1.35 * 1.35 && cube.max.y > base.y - 0.12
        })
    });
}

fn add_path(scene: &mut Scene, base: Vec3, style: usize) {
    let first_cube = scene.cubes.len();
    let first_ellipsoid = scene.ellipsoids.len();
    let first_capsule = scene.capsules.len();
    let height_scale = match style {
        1 => 0.91,
        4 => 1.05,
        _ => 1.0,
    };

    // Separate feet and a layered cloak preserve the human silhouette at a distance.
    for side in [-1.0, 1.0] {
        add_block(
            scene,
            base + Vec3::new(side * 0.22, 0.10, 0.08),
            Vec3::new(0.30, 0.20, 0.48),
            INK,
        );
    }

    for row in 0..8 {
        let y = 0.30 + row as f32 * 0.25 * height_scale;
        let width = 1.26 - row as f32 * 0.045;
        for column in -2i32..=2 {
            add_block(
                scene,
                base + Vec3::new(column as f32 * width / 5.0, y, 0.0),
                Vec3::new(width / 5.0 + 0.035, 0.27 * height_scale, 0.56),
                ROBE,
            );
        }
    }

    for z in [-0.30, 0.30] {
        add_block(
            scene,
            base + Vec3::new(0.0, 1.12 * height_scale, z),
            Vec3::new(0.055, 1.62 * height_scale, 0.035),
            CLOUD_RED,
        );
    }

    for side in [-1.0, 1.0] {
        add_segment(
            scene,
            base + Vec3::new(side * 0.52, 1.68 * height_scale, 0.0),
            base + Vec3::new(side * 0.68, 0.68 * height_scale, 0.05),
            0.20,
            ROBE,
        );
        add_block(
            scene,
            base + Vec3::new(side * 0.69, 0.59 * height_scale, 0.07),
            Vec3::new(0.22, 0.32, 0.34),
            ROBE,
        );
    }

    add_akatsuki_cloud(
        scene,
        base + Vec3::new(0.0, 0.92 * height_scale, 0.315),
        1.0,
    );
    add_akatsuki_cloud(
        scene,
        base + Vec3::new(0.0, 0.92 * height_scale, -0.315),
        -1.0,
    );

    // Collar, neck and head use progressively smaller voxels than the environment.
    add_block(
        scene,
        base + Vec3::new(0.0, 1.94 * height_scale, 0.02),
        Vec3::new(1.02, 0.56, 0.64),
        ROBE,
    );
    for z in [-0.335, 0.335] {
        add_block(
            scene,
            base + Vec3::new(0.0, 1.93 * height_scale, z),
            Vec3::new(0.82, 0.055, 0.045),
            CLOUD_RED,
        );
    }
    add_block(
        scene,
        base + Vec3::new(0.0, 2.14 * height_scale, 0.17),
        Vec3::new(0.35, 0.34, 0.30),
        PAIN_SKIN,
    );
    let head_center = base + Vec3::new(0.0, 2.48 * height_scale, 0.04);
    add_ellipsoid(
        scene,
        head_center,
        Vec3::new(0.43, 0.50 * height_scale, 0.37),
        0.08,
        PAIN_SKIN,
    );

    add_face(scene, head_center, style, height_scale);
    add_hair(scene, head_center, style, height_scale);

    let toward_toads = Vec3::new(0.0, base.y, -1.4) - base;
    rotate_added_geometry(
        scene,
        first_cube,
        first_ellipsoid,
        first_capsule,
        base,
        toward_toads.x.atan2(toward_toads.z),
    );
}

fn add_face(scene: &mut Scene, head: Vec3, style: usize, scale: f32) {
    let front = head.z + 0.36;
    for side in [-1.0, 1.0] {
        add_ellipsoid(
            scene,
            head + Vec3::new(side * 0.43, -0.02 * scale, 0.0),
            Vec3::new(0.10, 0.18, 0.11),
            0.055,
            PAIN_SKIN,
        );
    }
    add_block(
        scene,
        Vec3::new(head.x, head.y + 0.25 * scale, front + 0.02),
        Vec3::new(0.70, 0.17, 0.075),
        INK,
    );
    add_block(
        scene,
        Vec3::new(head.x, head.y + 0.26 * scale, front + 0.075),
        Vec3::new(0.48, 0.18, 0.055),
        METAL,
    );
    for side in [-1.0, 1.0] {
        let eye_x = head.x + side * 0.17;
        add_block(
            scene,
            Vec3::new(eye_x, head.y + 0.05 * scale, front + 0.07),
            Vec3::new(0.18, 0.09, 0.045),
            RINNEGAN,
        );
        add_block(
            scene,
            Vec3::new(eye_x, head.y + 0.05 * scale, front + 0.10),
            Vec3::new(0.035, 0.055, 0.025),
            INK,
        );
    }
    add_block(
        scene,
        Vec3::new(head.x, head.y - 0.23 * scale, front + 0.06),
        Vec3::new(0.23, 0.045, 0.04),
        INK,
    );
    add_block(
        scene,
        Vec3::new(head.x, head.y - 0.03 * scale, front + 0.085),
        Vec3::new(0.075, 0.22, 0.055),
        PAIN_SKIN,
    );

    // Piercings vary per body so the six Paths are not simple duplicates.
    match style {
        0 | 2 => {
            for offset in [-0.13, 0.0, 0.13] {
                add_block(
                    scene,
                    Vec3::new(head.x, head.y - 0.08 * scale + offset, front + 0.085),
                    Vec3::new(0.04, 0.04, 0.035),
                    METAL,
                );
            }
        }
        1 | 5 => {
            for side in [-1.0, 1.0] {
                add_block(
                    scene,
                    Vec3::new(head.x + side * 0.29, head.y - 0.06 * scale, front + 0.04),
                    Vec3::new(0.045, 0.13, 0.04),
                    METAL,
                );
            }
        }
        _ => {
            add_block(
                scene,
                Vec3::new(head.x, head.y + 0.02 * scale, front + 0.09),
                Vec3::new(0.045, 0.20, 0.04),
                METAL,
            );
        }
    }
}

fn add_hair(scene: &mut Scene, head: Vec3, style: usize, scale: f32) {
    match style {
        1 => {
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.38 * scale, -0.04),
                Vec3::new(0.42, 0.22, 0.34),
                0.09,
                PAIN_HAIR,
            );
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.66 * scale, -0.05),
                Vec3::new(0.23, 0.25, 0.23),
                0.075,
                PAIN_HAIR,
            );
        }
        4 => {
            add_block(
                scene,
                head + Vec3::new(0.0, -0.18 * scale, -0.20),
                Vec3::new(0.82, 1.42 * scale, 0.30),
                PAIN_HAIR,
            );
            for side in [-1.0, 1.0] {
                add_block(
                    scene,
                    head + Vec3::new(side * 0.36, -0.33 * scale, 0.08),
                    Vec3::new(0.16, 1.18 * scale, 0.20),
                    PAIN_HAIR,
                );
            }
        }
        3 => {
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.34 * scale, -0.10),
                Vec3::new(0.43, 0.23, 0.35),
                0.09,
                PAIN_HAIR,
            );
            for side in [-1.0, 1.0] {
                add_segment(
                    scene,
                    head + Vec3::new(side * 0.22, 0.42 * scale, -0.02),
                    head + Vec3::new(side * 0.32, 0.72 * scale, -0.05),
                    0.10,
                    INK,
                );
            }
        }
        _ => {
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.34 * scale, -0.08),
                Vec3::new(0.44, 0.25, 0.36),
                0.09,
                PAIN_HAIR,
            );
            for index in -3i32..=3 {
                let x = index as f32 * 0.12;
                let tip_y = 0.67 + (3 - index.abs()) as f32 * 0.035;
                add_segment(
                    scene,
                    head + Vec3::new(x * 0.72, 0.41 * scale, -0.03),
                    head + Vec3::new(x, tip_y * scale, -0.05),
                    0.105,
                    PAIN_HAIR,
                );
            }
        }
    }
}

fn add_akatsuki_cloud(scene: &mut Scene, center: Vec3, facing: f32) {
    for offset in [
        Vec3::new(-0.23, 0.0, 0.0),
        Vec3::new(0.02, 0.10, 0.0),
        Vec3::new(0.25, 0.0, 0.0),
        Vec3::new(0.10, -0.13, 0.0),
    ] {
        add_ellipsoid(
            scene,
            center + offset,
            Vec3::new(0.22, 0.18, 0.055),
            0.08,
            CLOUD_WHITE,
        );
    }
    for offset in [
        Vec3::new(-0.21, 0.0, 0.065 * facing),
        Vec3::new(0.02, 0.09, 0.065 * facing),
        Vec3::new(0.23, 0.0, 0.065 * facing),
        Vec3::new(0.10, -0.12, 0.065 * facing),
    ] {
        add_ellipsoid(
            scene,
            center + offset,
            Vec3::new(0.16, 0.12, 0.04),
            0.07,
            CLOUD_RED,
        );
    }
}

fn rotate_added_geometry(
    scene: &mut Scene,
    first_cube: usize,
    first_ellipsoid: usize,
    first_capsule: usize,
    origin: Vec3,
    yaw: f32,
) {
    let cosine = yaw.cos();
    let sine = yaw.sin();
    for cube in &mut scene.cubes[first_cube..] {
        let center = (cube.min + cube.max) * 0.5;
        let half_size = (cube.max - cube.min) * 0.5;
        let local = center - origin;
        let rotated_center = origin
            + Vec3::new(
                local.x * cosine + local.z * sine,
                local.y,
                -local.x * sine + local.z * cosine,
            );
        cube.min = rotated_center - half_size;
        cube.max = rotated_center + half_size;
        if let Some(normal) = &mut cube.smooth_normal {
            let old_x = normal.x;
            let old_z = normal.z;
            normal.x = old_x * cosine + old_z * sine;
            normal.z = -old_x * sine + old_z * cosine;
        }
    }
    for ellipsoid in &mut scene.ellipsoids[first_ellipsoid..] {
        let local = ellipsoid.center - origin;
        ellipsoid.center = origin
            + Vec3::new(
                local.x * cosine + local.z * sine,
                local.y,
                -local.x * sine + local.z * cosine,
            );
    }
    for capsule in &mut scene.capsules[first_capsule..] {
        for point in [&mut capsule.start, &mut capsule.end] {
            let local = *point - origin;
            *point = origin
                + Vec3::new(
                    local.x * cosine + local.z * sine,
                    local.y,
                    -local.x * sine + local.z * cosine,
                );
        }
    }
}

fn add_ellipsoid(scene: &mut Scene, center: Vec3, radii: Vec3, cell: f32, material: usize) {
    let _detail_hint = cell;
    scene.ellipsoids.push(Ellipsoid {
        center,
        radii,
        material,
    });
}

fn add_segment(scene: &mut Scene, start: Vec3, end: Vec3, thickness: f32, material: usize) {
    scene.capsules.push(Capsule {
        start,
        end,
        radius: thickness * 0.5,
        material,
    });
}

fn add_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    scene.cubes.push(Cube {
        min: center - size * 0.5,
        max: center + size * 0.5,
        material,
        smooth_normal: None,
    });
}
