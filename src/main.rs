mod background;
mod bvh;
mod camera;
mod characters;
mod config;
mod image;
mod material;
mod math;
mod scene;

use background::add_destroyed_konoha;
use bvh::Bvh;
use camera::{Camera, Ray};
use characters::{add_gamabunta, add_gamahiro, add_gamaken, add_naruto_sage, add_summoning_clouds};
use config::Config;
use image::{save_bmp, save_ppm};
use material::{scene_materials, Material};
use math::{Color, Vec3};
use scene::{Cube, Scene};
use std::f32::consts::PI;
use std::fs::create_dir_all;
use std::io::{self, Write};
use std::path::Path;
use std::thread;

const EPSILON: f32 = 0.001;

struct Hit {
    point: Vec3,
    normal: Vec3,
    material: Material,
    distance: f32,
}

fn main() -> std::io::Result<()> {
    let cfg = Config::from_args();
    let scene = build_scene();

    if cfg.summary {
        print_scene_summary(&scene);
    } else if cfg.interactive {
        run_interactive(&scene, cfg)?;
    } else if cfg.animate {
        create_dir_all("frames")?;
        for frame in 0..cfg.frames {
            let path = format!("frames/frame_{:04}.ppm", frame);
            render_to_file(&scene, &cfg, frame, &path)?;
            println!("rendered {}", path);
        }
    } else {
        render_to_file(&scene, &cfg, cfg.frame, &cfg.output)?;
        println!("rendered {}", cfg.output);
    }

    Ok(())
}

fn print_scene_summary(scene: &Scene) {
    println!("Resumen de escena");
    println!("Cubos: {}", scene.cubes.len());
    println!("Materiales: {}", scene.materials.len());
    println!("Efectos: sombras, specular, reflexion, refraccion y skybox procedural");
}

fn run_interactive(scene: &Scene, mut cfg: Config) -> std::io::Result<()> {
    create_dir_all("renders")?;
    cfg.output = "renders/interactive.bmp".to_string();
    cfg.angle_deg = cfg.angle_deg.or(Some(90.0));
    cfg.zoom = cfg.zoom.max(0.35);

    println!("Modo interactivo de camara");
    println!(
        "a/d: rotar | w/s: subir/bajar angulo orbital fino | +/-: zoom | r: render | q: salir"
    );
    println!("Cada render se guarda en renders/interactive.bmp");

    render_to_file(scene, &cfg, 0, &cfg.output)?;
    println!(
        "rendered {} | angle {:.1} | zoom {:.2}",
        cfg.output,
        cfg.angle_deg.unwrap_or(90.0),
        cfg.zoom
    );

    loop {
        print!("comando> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command = input.trim().to_ascii_lowercase();
        match command.as_str() {
            "a" => cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) - 12.0),
            "d" => cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) + 12.0),
            "w" => cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) + 4.0),
            "s" => cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) - 4.0),
            "+" | "=" => cfg.zoom = (cfg.zoom + 0.15).min(2.5),
            "-" | "_" => cfg.zoom = (cfg.zoom - 0.15).max(0.45),
            "r" | "" => {}
            "q" | "salir" => break,
            _ => {
                println!("Comando no reconocido. Usa a/d, w/s, +/-, r o q.");
                continue;
            }
        }

        render_to_file(scene, &cfg, 0, &cfg.output)?;
        println!(
            "rendered {} | angle {:.1} | zoom {:.2}",
            cfg.output,
            cfg.angle_deg.unwrap_or(90.0),
            cfg.zoom
        );
    }

    Ok(())
}

fn render_to_file(scene: &Scene, cfg: &Config, frame: usize, path: &str) -> std::io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            create_dir_all(parent)?;
        }
    }

    let aspect = cfg.width as f32 / cfg.height as f32;
    let t = frame as f32 / cfg.frames.max(1) as f32;
    let angle = cfg
        .angle_deg
        .map(|a| a.to_radians())
        .unwrap_or(t * 2.0 * PI + PI * 0.5);
    let zoom_wave = (t * 2.0 * PI).sin() * 0.18;
    let radius = (25.5 - zoom_wave * 5.5) / cfg.zoom.max(0.35);
    let camera_pos = Vec3::new(
        angle.cos() * radius,
        11.8 + zoom_wave * 2.5,
        angle.sin() * radius,
    );
    let camera = Camera::look_at(camera_pos, Vec3::new(0.0, 3.6, -1.0), 46.0, aspect);
    let bvh = Bvh::build(&scene.cubes);

    let mut pixels = vec![Color::default(); cfg.width * cfg.height];
    let worker_count = thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
        .min(cfg.height.max(1));
    let rows_per_worker = (cfg.height + worker_count - 1) / worker_count;
    let pixels_per_worker = rows_per_worker * cfg.width;

    thread::scope(|scope| {
        for (worker, pixel_chunk) in pixels.chunks_mut(pixels_per_worker).enumerate() {
            let start_y = worker * rows_per_worker;
            let camera = &camera;
            let bvh = &bvh;
            scope.spawn(move || {
                render_rows(scene, bvh, camera, cfg, start_y, pixel_chunk);
            });
        }
    });

    if path.to_ascii_lowercase().ends_with(".bmp") {
        save_bmp(path, cfg.width, cfg.height, &pixels)?;
    } else {
        save_ppm(path, cfg.width, cfg.height, &pixels)?;
    }

    Ok(())
}

fn render_rows(
    scene: &Scene,
    bvh: &Bvh,
    camera: &Camera,
    cfg: &Config,
    start_y: usize,
    pixels: &mut [Color],
) {
    for (local_y, row) in pixels.chunks_mut(cfg.width).enumerate() {
        let y = start_y + local_y;
        for x in 0..cfg.width {
            let mut color = Color::default();
            let samples = cfg.samples_per_axis;
            let inv_samples = 1.0 / samples as f32;
            for sy in 0..samples {
                for sx in 0..samples {
                    let u = (x as f32 + (sx as f32 + 0.5) * inv_samples) / (cfg.width - 1) as f32;
                    let v = 1.0
                        - (y as f32 + (sy as f32 + 0.5) * inv_samples) / (cfg.height - 1) as f32;
                    color += trace(scene, &bvh, camera.ray(u, v), 0, cfg.max_depth);
                }
            }
            color = color / (samples * samples) as f32;
            let nx = (x as f32 / cfg.width as f32 - 0.5) * 2.0;
            let ny = (y as f32 / cfg.height as f32 - 0.5) * 2.0;
            let vignette = (1.0 - (nx * nx + ny * ny) * 0.08).clamp(0.78, 1.0);
            color = tone_map(color) * vignette;
            row[x] = color;
        }
    }
}

fn trace(scene: &Scene, bvh: &Bvh, ray: Ray, depth: u32, max_depth: u32) -> Color {
    if depth >= max_depth {
        return skybox(ray.direction);
    }

    if let Some(hit) = intersect_scene(scene, bvh, ray) {
        shade(scene, bvh, ray, hit, depth, max_depth)
    } else {
        skybox(ray.direction)
    }
}

fn shade(scene: &Scene, bvh: &Bvh, ray: Ray, hit: Hit, depth: u32, max_depth: u32) -> Color {
    let view_dir = -ray.direction;
    let light_dir = -scene.light_dir.normalized();
    let base = hit.material.texture(hit.point, hit.normal);

    let visibility = soft_shadow(scene, bvh, hit.point, hit.normal, light_dir);
    let ndotl = hit.normal.dot(light_dir).max(0.0);
    let diffuse_strength = ndotl * (0.16 + visibility * 0.84);
    let diffuse = base.hadamard(scene.light_color) * diffuse_strength;

    let half_vec = (light_dir + view_dir).normalized();
    let spec = hit.normal.dot(half_vec).max(0.0).powf(48.0) * hit.material.specular * visibility;
    let specular = scene.light_color * spec;
    let sky_ambient = Color::new(0.20, 0.28, 0.42);
    let ambient = base * 0.15 + base.hadamard(sky_ambient) * (0.12 + hit.normal.y.max(0.0) * 0.08);
    let rim = (1.0 - hit.normal.dot(view_dir).max(0.0)).powf(3.0) * 0.13;
    let rim_light = Color::new(0.30, 0.46, 0.72) * rim;

    let mut color = ambient + diffuse + specular + rim_light;

    if hit.material.reflectivity > 0.0 {
        let reflected = ray.direction.reflect(hit.normal).normalized();
        let reflected_color = trace(
            scene,
            bvh,
            Ray {
                origin: hit.point + hit.normal * EPSILON,
                direction: reflected,
            },
            depth + 1,
            max_depth,
        );
        color =
            color * (1.0 - hit.material.reflectivity) + reflected_color * hit.material.reflectivity;
    }

    if hit.material.transparency > 0.0 {
        let entering = ray.direction.dot(hit.normal) < 0.0;
        let normal = if entering { hit.normal } else { -hit.normal };
        let eta = if entering {
            1.0 / hit.material.refractive_index
        } else {
            hit.material.refractive_index
        };
        let refracted = ray
            .direction
            .refract(normal, eta)
            .unwrap_or_else(|| ray.direction.reflect(normal));
        let refracted_color = trace(
            scene,
            bvh,
            Ray {
                origin: hit.point - normal * EPSILON * 2.0,
                direction: refracted.normalized(),
            },
            depth + 1,
            max_depth,
        );
        color =
            color * (1.0 - hit.material.transparency) + refracted_color * hit.material.transparency;
    }

    let fog = ((hit.distance - 20.0) / 34.0).clamp(0.0, 1.0) * 0.30;
    color = color * (1.0 - fog) + skybox(ray.direction) * fog;
    color.clamp01()
}

fn soft_shadow(scene: &Scene, bvh: &Bvh, point: Vec3, normal: Vec3, light_dir: Vec3) -> f32 {
    let tangent = light_dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalized();
    let bitangent = tangent.cross(light_dir).normalized();
    let offsets = [(0.0, 0.0), (0.045, -0.025), (-0.035, 0.040)];
    let mut visible = 0.0;

    for (x, y) in offsets {
        let direction = (light_dir + tangent * x + bitangent * y).normalized();
        let shadow_ray = Ray {
            origin: point + normal * EPSILON,
            direction,
        };
        if !bvh.any_hit(&scene.cubes, shadow_ray) {
            visible += 1.0;
        }
    }

    visible / offsets.len() as f32
}

fn tone_map(color: Color) -> Color {
    let exposure = 1.55;
    Color::new(
        (1.0 - (-color.x * exposure).exp()).powf(1.0 / 2.2),
        (1.0 - (-color.y * exposure).exp()).powf(1.0 / 2.2),
        (1.0 - (-color.z * exposure).exp()).powf(1.0 / 2.2),
    )
    .clamp01()
}

fn intersect_scene(scene: &Scene, bvh: &Bvh, ray: Ray) -> Option<Hit> {
    bvh.nearest(&scene.cubes, ray)
        .map(|(index, distance, normal)| Hit {
            point: ray.at(distance),
            normal,
            material: scene.materials[scene.cubes[index].material],
            distance,
        })
}

fn skybox(dir: Vec3) -> Color {
    let t = (dir.y * 0.5 + 0.5).clamp(0.0, 1.0);
    let horizon = Color::new(0.72, 0.30, 0.12);
    let zenith = Color::new(0.10, 0.24, 0.44);
    let mut color = horizon * (1.0 - t) + zenith * t;

    let sun_dir = Vec3::new(-0.38, 0.36, -0.85).normalized();
    let sun = dir.dot(sun_dir).max(0.0).powf(320.0);
    let glow = dir.dot(sun_dir).max(0.0).powf(18.0);
    color += Color::new(1.0, 0.86, 0.54) * sun;
    color += Color::new(0.72, 0.32, 0.14) * glow * 0.28;

    let cloud_band = (1.0 - (dir.y - 0.14).abs() * 6.0).max(0.0);
    let cloud_shape =
        ((dir.x * 18.0 + dir.z * 11.0).sin() + (dir.x * 31.0 - dir.z * 7.0).sin() * 0.45) * 0.5
            + 0.42;
    color += Color::new(0.48, 0.31, 0.25) * cloud_shape.max(0.0) * cloud_band * 0.24;
    color.clamp01()
}

fn build_scene() -> Scene {
    let mut scene = Scene {
        cubes: Vec::new(),
        materials: scene_materials(),
        light_dir: Vec3::new(0.42, -0.82, -0.30).normalized(),
        light_color: Color::new(1.0, 0.95, 0.84),
    };

    build_sage_arrival(&mut scene);
    scene
}

fn build_sage_arrival(scene: &mut Scene) {
    add_block(
        scene,
        Vec3::new(0.0, -0.65, 0.0),
        Vec3::new(24.0, 1.3, 12.0),
        0,
    );
    add_block(
        scene,
        Vec3::new(0.0, 0.08, 0.4),
        Vec3::new(16.0, 0.25, 7.5),
        17,
    );

    add_destroyed_konoha(scene);
    add_gamabunta(scene, Vec3::new(0.0, 0.0, -0.3));
    add_gamaken(scene, Vec3::new(-7.2, 0.0, -1.3));
    add_gamahiro(scene, Vec3::new(7.2, 0.0, -1.3));
    add_naruto_sage(scene, Vec3::new(0.0, 8.0, 0.25));
    add_summoning_clouds(scene);
}

fn add_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    scene.cubes.push(Cube {
        min: center - size * 0.5,
        max: center + size * 0.5,
        material,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.0001,
            "expected {expected}, got {actual}"
        );
    }

    fn test_cube() -> Cube {
        Cube {
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
            material: 0,
        }
    }

    #[test]
    fn reflection_preserves_the_incident_angle() {
        let reflected = Vec3::new(1.0, -1.0, 0.0).reflect(Vec3::new(0.0, 1.0, 0.0));

        assert_close(reflected.x, 1.0);
        assert_close(reflected.y, 1.0);
        assert_close(reflected.z, 0.0);
    }

    #[test]
    fn refraction_keeps_a_perpendicular_ray_straight() {
        let refracted = Vec3::new(0.0, -1.0, 0.0)
            .refract(Vec3::new(0.0, 1.0, 0.0), 1.0 / 1.33)
            .expect("a perpendicular ray should refract");

        assert_close(refracted.x, 0.0);
        assert_close(refracted.y, -1.0);
        assert_close(refracted.z, 0.0);
    }

    #[test]
    fn refraction_detects_total_internal_reflection() {
        let direction = Vec3::new(0.9, -0.435_889_9, 0.0).normalized();

        assert!(direction.refract(Vec3::new(0.0, 1.0, 0.0), 1.5).is_none());
    }

    #[test]
    fn cube_intersection_returns_distance_and_face_normal() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -3.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let (distance, normal) =
            bvh::intersect_cube(ray, test_cube()).expect("ray should hit cube");

        assert_close(distance, 2.0);
        assert_close(normal.z, -1.0);
    }

    #[test]
    fn cube_intersection_uses_exit_face_when_ray_starts_inside() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
        };
        let (distance, normal) =
            bvh::intersect_cube(ray, test_cube()).expect("ray should exit cube");

        assert_close(distance, 1.0);
        assert_close(normal.x, 1.0);
    }

    #[test]
    fn center_camera_ray_points_at_target() {
        let origin = Vec3::new(0.0, 2.0, -5.0);
        let target = Vec3::new(0.0, 1.0, 0.0);
        let camera = Camera::look_at(origin, target, 46.0, 16.0 / 9.0);
        let expected = (target - origin).normalized();
        let ray = camera.ray(0.5, 0.5);

        assert_close(ray.direction.x, expected.x);
        assert_close(ray.direction.y, expected.y);
        assert_close(ray.direction.z, expected.z);
    }

    #[test]
    fn scene_contains_required_raytracing_materials() {
        let scene = build_scene();

        assert!(scene.materials.len() >= 5);
        assert!(scene.materials.iter().any(|m| m.reflectivity > 0.0));
        assert!(scene.materials.iter().any(|m| m.transparency > 0.0));
        assert!(scene.materials.iter().any(|m| m.refractive_index > 1.0));
        assert!(!scene.cubes.is_empty());
    }

    #[test]
    fn bvh_matches_brute_force_for_camera_rays() {
        let scene = build_scene();
        let bvh = Bvh::build(&scene.cubes);
        let angle = 90.0_f32.to_radians();
        let radius = 25.5 / 0.96;
        let camera = Camera::look_at(
            Vec3::new(angle.cos() * radius, 11.8, angle.sin() * radius),
            Vec3::new(0.0, 3.6, -1.0),
            46.0,
            16.0 / 9.0,
        );

        for y in 0..30 {
            for x in 0..50 {
                let ray = camera.ray((x as f32 + 0.5) / 50.0, (y as f32 + 0.5) / 30.0);
                let accelerated = bvh.nearest(&scene.cubes, ray);
                let mut brute_force = None;
                let mut closest = f32::INFINITY;
                for (index, &cube) in scene.cubes.iter().enumerate() {
                    if let Some((distance, normal)) = bvh::intersect_cube(ray, cube) {
                        if distance < closest {
                            closest = distance;
                            brute_force = Some((index, distance, normal));
                        }
                    }
                }

                assert_eq!(
                    accelerated.map(|hit| hit.0),
                    brute_force.map(|hit| hit.0),
                    "different cube for sample ({x}, {y})"
                );
                if let (Some(accelerated), Some(brute_force)) = (accelerated, brute_force) {
                    assert_close(accelerated.1, brute_force.1);
                }
            }
        }
    }
}
