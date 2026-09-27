use crate::math::Vec3;
use crate::scene::{Cube, Scene};

const ROBE: usize = 5;
const CLOUD_RED: usize = 7;
const INK: usize = 10;
const METAL: usize = 11;
const CLOUD_WHITE: usize = 15;
const PAIN_SKIN: usize = 26;
const RINNEGAN: usize = 27;
const PAIN_HAIR: usize = 28;

pub fn add_six_paths(scene: &mut Scene) {
    let paths = [
        (Vec3::new(-10.0, -0.38, 9.6), 0usize),
        (Vec3::new(-6.0, -0.38, 10.4), 1usize),
        (Vec3::new(-2.0, -0.38, 10.8), 2usize),
        (Vec3::new(2.0, -0.38, 10.8), 3usize),
        (Vec3::new(6.0, -0.38, 10.4), 4usize),
        (Vec3::new(10.0, -0.38, 9.6), 5usize),
    ];

    for (base, style) in paths {
        add_path(scene, base, style);
    }
}

fn add_path(scene: &mut Scene, base: Vec3, style: usize) {
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
        add_block(
            scene,
            base + Vec3::new(0.0, y, 0.0),
            Vec3::new(width, 0.27 * height_scale, 0.56),
            ROBE,
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

    add_akatsuki_cloud(scene, base + Vec3::new(0.0, 0.92 * height_scale, 0.315));

    // Collar, neck and head use progressively smaller voxels than the environment.
    add_block(
        scene,
        base + Vec3::new(0.0, 1.94 * height_scale, 0.02),
        Vec3::new(1.02, 0.56, 0.64),
        ROBE,
    );
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
        0.11,
        PAIN_SKIN,
    );

    add_face(scene, head_center, style, height_scale);
    add_hair(scene, head_center, style, height_scale);
}

fn add_face(scene: &mut Scene, head: Vec3, style: usize, scale: f32) {
    let front = head.z + 0.36;
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
                0.12,
                PAIN_HAIR,
            );
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.66 * scale, -0.05),
                Vec3::new(0.23, 0.25, 0.23),
                0.10,
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
                0.12,
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
                0.12,
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

fn add_akatsuki_cloud(scene: &mut Scene, center: Vec3) {
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
        Vec3::new(-0.21, 0.0, 0.065),
        Vec3::new(0.02, 0.09, 0.065),
        Vec3::new(0.23, 0.0, 0.065),
        Vec3::new(0.10, -0.12, 0.065),
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

fn add_ellipsoid(scene: &mut Scene, center: Vec3, radii: Vec3, cell: f32, material: usize) {
    let nx = (radii.x / cell).ceil() as i32;
    let ny = (radii.y / cell).ceil() as i32;
    let nz = (radii.z / cell).ceil() as i32;
    let shell = (cell / radii.x.min(radii.y).min(radii.z)).clamp(0.12, 0.48) * 2.2;

    for ix in -nx..=nx {
        for iy in -ny..=ny {
            for iz in -nz..=nz {
                let offset = Vec3::new(ix as f32 * cell, iy as f32 * cell, iz as f32 * cell);
                let q = Vec3::new(offset.x / radii.x, offset.y / radii.y, offset.z / radii.z);
                let distance = q.dot(q);
                if distance <= 1.0 && distance >= 1.0 - shell {
                    let normal = Vec3::new(
                        offset.x / (radii.x * radii.x),
                        offset.y / (radii.y * radii.y),
                        offset.z / (radii.z * radii.z),
                    )
                    .normalized();
                    scene.cubes.push(Cube {
                        min: center + offset - Vec3::new(cell, cell, cell) * 0.52,
                        max: center + offset + Vec3::new(cell, cell, cell) * 0.52,
                        material,
                        smooth_normal: Some(normal),
                    });
                }
            }
        }
    }
}

fn add_segment(scene: &mut Scene, start: Vec3, end: Vec3, thickness: f32, material: usize) {
    let delta = end - start;
    let steps = (delta.length() / (thickness * 0.72)).ceil().max(1.0) as usize;
    for index in 0..=steps {
        let t = index as f32 / steps as f32;
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
        smooth_normal: None,
    });
}
