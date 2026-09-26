use crate::material::noise;
use crate::math::Vec3;
use crate::scene::{Cube, Scene};
use std::f32::consts::PI;

pub fn add_destroyed_konoha(scene: &mut Scene) {
    add_block(
        scene,
        Vec3::new(0.0, -0.05, -7.2),
        Vec3::new(27.0, 0.32, 13.5),
        17,
    );

    add_crater_arc(scene, 7.2, 0.18, 1.15, 20);
    add_crater_arc(scene, 10.1, 0.58, 1.55, 24);
    add_crater_arc(scene, 13.2, 1.12, 2.0, 28);
    add_crater_wall(scene);

    for (x, z, width, height) in [
        (-10.4, -4.8, 2.7, 4.6),
        (-7.8, -8.6, 3.4, 3.0),
        (8.1, -8.4, 3.2, 3.4),
        (10.6, -4.5, 2.8, 4.9),
    ] {
        add_ruined_building(scene, Vec3::new(x, 0.0, z), width, height);
    }

    for (x, z, sx, sy, sz, material) in [
        (-8.5, -2.2, 1.8, 0.55, 0.9, 18),
        (-6.3, -4.0, 1.1, 0.42, 1.4, 20),
        (-4.5, -6.0, 1.5, 0.50, 0.75, 18),
        (-2.4, -4.6, 0.8, 0.36, 1.5, 19),
        (2.8, -5.2, 1.2, 0.48, 0.8, 18),
        (4.8, -7.1, 1.6, 0.45, 0.7, 20),
        (6.7, -3.8, 0.9, 0.38, 1.5, 19),
        (8.9, -2.4, 1.7, 0.52, 0.85, 18),
    ] {
        add_block(
            scene,
            Vec3::new(x, 0.42, z),
            Vec3::new(sx, sy, sz),
            material,
        );
    }

    add_crack_path(
        scene,
        Vec3::new(-0.8, 0.18, -1.8),
        Vec3::new(-7.0, 0.18, -8.5),
    );
    add_crack_path(
        scene,
        Vec3::new(1.2, 0.18, -2.2),
        Vec3::new(7.8, 0.18, -7.4),
    );
    add_crack_path(
        scene,
        Vec3::new(0.2, 0.18, -2.0),
        Vec3::new(-1.3, 0.18, -10.2),
    );

    add_dust_plume(scene, Vec3::new(-9.4, 1.2, -9.4), 1.15);
    add_dust_plume(scene, Vec3::new(9.2, 1.0, -9.0), 1.0);
    add_dust_plume(scene, Vec3::new(4.8, 0.8, -11.5), 0.72);
}

fn add_crater_arc(scene: &mut Scene, radius: f32, height: f32, block_size: f32, segments: usize) {
    for index in 0..=segments {
        let angle = PI + PI * index as f32 / segments as f32;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.2;
        let uneven = noise(Vec3::new(x, height, z));
        add_block(
            scene,
            Vec3::new(x, height + uneven * 0.35, z),
            Vec3::new(block_size, block_size * (0.65 + uneven * 0.45), block_size),
            if index % 5 == 0 { 18 } else { 17 },
        );
    }
}

fn add_crater_wall(scene: &mut Scene) {
    for index in -10i32..=10 {
        let x = index as f32 * 1.28;
        let edge = (index.abs() as f32 / 10.0).powf(1.5);
        let z = -12.2 + edge * 2.6;
        let variation = noise(Vec3::new(x, 2.0, z));
        let height = 3.4 + edge * 3.4 + variation * 1.2;
        add_block(
            scene,
            Vec3::new(x, height * 0.5 - 0.15, z),
            Vec3::new(1.42, height, 2.25),
            if index % 6 == 0 { 18 } else { 17 },
        );
        if index % 4 == 0 {
            add_block(
                scene,
                Vec3::new(x + 0.28, height + 0.22, z - 0.15),
                Vec3::new(0.72, 0.42, 1.35),
                18,
            );
        }
    }
}

fn add_ruined_building(scene: &mut Scene, base: Vec3, width: f32, height: f32) {
    let levels = height.ceil() as usize;
    for level in 0..levels {
        let y = 0.5 + level as f32;
        let taper = level as f32 * 0.12;
        if level % 3 != 1 {
            add_block(
                scene,
                base + Vec3::new(-width * 0.43 + taper, y, 0.0),
                Vec3::new(0.42, 0.92, 2.0 - taper * 0.3),
                18,
            );
        }
        if level % 4 != 2 {
            add_block(
                scene,
                base + Vec3::new(width * 0.43 - taper, y, 0.0),
                Vec3::new(0.42, 0.92, 2.0 - taper * 0.3),
                18,
            );
        }
        if level % 2 == 0 {
            add_block(
                scene,
                base + Vec3::new(0.0, y, -0.85),
                Vec3::new((width - taper).max(0.8), 0.38, 0.34),
                20,
            );
        }
    }
    add_block(
        scene,
        base + Vec3::new(width * 0.18, height + 0.18, 0.0),
        Vec3::new(width * 0.72, 0.30, 2.15),
        19,
    );
}

fn add_crack_path(scene: &mut Scene, start: Vec3, end: Vec3) {
    for step in 0..8 {
        let t = step as f32 / 7.0;
        let bend = (step as f32 * 2.1).sin() * 0.35;
        let point = start * (1.0 - t) + end * t + Vec3::new(bend, 0.0, 0.0);
        add_block(scene, point, Vec3::new(0.20, 0.06, 1.15), 0);
    }
}

fn add_dust_plume(scene: &mut Scene, base: Vec3, scale: f32) {
    for (offset, size) in [
        (Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.5, 1.2, 1.8)),
        (Vec3::new(-0.5, 1.0, 0.0), Vec3::new(2.0, 1.4, 1.5)),
        (Vec3::new(0.35, 2.0, -0.1), Vec3::new(1.7, 1.6, 1.4)),
        (Vec3::new(-0.2, 3.1, 0.0), Vec3::new(1.25, 1.7, 1.1)),
    ] {
        add_block(scene, base + offset * scale, size * scale, 20);
    }
}

fn add_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    scene.cubes.push(Cube {
        min: center - size * 0.5,
        max: center + size * 0.5,
        material,
    });
}
