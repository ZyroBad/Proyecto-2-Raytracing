mod background;
mod bvh;
mod camera;
mod characters;
mod config;
mod image;
mod material;
mod math;
mod pain;
mod scene;
mod window;

use background::add_destroyed_konoha;
use bvh::Bvh;
#[cfg(test)]
use bvh::{intersect_capsule, intersect_ellipsoid};
use camera::{Camera, Ray};
use characters::{
    add_gamabunta, add_gamahiro, add_gamaken, add_gamakichi, add_naruto_sage, add_summoning_clouds,
};
use config::Config;
use image::{save_bmp, save_ppm};
use material::{scene_materials, Material};
use math::{Color, Vec3};
use pain::add_six_paths;
#[cfg(test)]
use scene::{Capsule, Ellipsoid};
use scene::{Cube, Scene};
use std::f32::consts::PI;
use std::fs::create_dir_all;
use std::io::{self, Write};
use std::path::Path;
use std::thread;

const EPSILON: f32 = 0.001;
const SECONDARY_RAY_THRESHOLD: f32 = 0.075;

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
    } else if cfg.window {
        window::run_window(&scene, cfg, render_pixels_with_bvh)?;
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
    println!("Elipsoides: {}", scene.ellipsoids.len());
    println!("Capsulas: {}", scene.capsules.len());
    println!("Triangulos: {}", scene.triangles.len());
    println!("Materiales: {}", scene.materials.len());
    println!("Efectos: sombras, specular, reflexion, refraccion y skybox procedural");
}

fn run_interactive(scene: &Scene, mut cfg: Config) -> std::io::Result<()> {
    create_dir_all("renders")?;
    cfg.output = "renders/interactive.bmp".to_string();
    cfg.angle_deg = cfg.angle_deg.or(Some(90.0));
    cfg.zoom = cfg.zoom.max(0.35);

    println!("Modo interactivo de camara");
    println!("a/d: rotar | w/s: elevar/bajar camara | +/-: zoom | r: render | q: salir");
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
            "w" => cfg.elevation = (cfg.elevation + 0.8).min(8.0),
            "s" => cfg.elevation = (cfg.elevation - 0.8).max(-6.0),
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

    let pixels = render_pixels(scene, cfg, frame);

    if path.to_ascii_lowercase().ends_with(".bmp") {
        save_bmp(path, cfg.width, cfg.height, &pixels)?;
    } else {
        save_ppm(path, cfg.width, cfg.height, &pixels)?;
    }

    Ok(())
}

fn render_pixels(scene: &Scene, cfg: &Config, frame: usize) -> Vec<Color> {
    let bvh = Bvh::build_scene(
        &scene.cubes,
        &scene.triangles,
        &scene.ellipsoids,
        &scene.capsules,
    );
    render_pixels_with_bvh(scene, &bvh, cfg, frame)
}

fn render_pixels_with_bvh(scene: &Scene, bvh: &Bvh, cfg: &Config, frame: usize) -> Vec<Color> {
    let aspect = cfg.width as f32 / cfg.height as f32;
    let t = frame as f32 / cfg.frames.max(1) as f32;
    let angle = cfg
        .angle_deg
        .map(|a| a.to_radians())
        .unwrap_or(t * 2.0 * PI + PI * 0.5);
    let zoom_wave = (t * 2.0 * PI).sin() * 0.18;
    let base_radius = if cfg.cinematic { 25.5 } else { 33.0 };
    let base_height = if cfg.cinematic { 5.2 } else { 20.0 };
    let radius = (base_radius - zoom_wave * 5.5) / cfg.zoom.max(0.35);
    let camera_pos = Vec3::new(
        angle.cos() * radius,
        base_height + cfg.elevation + zoom_wave * 2.8,
        angle.sin() * radius,
    );
    let camera_right = Vec3::new(angle.sin(), 0.0, -angle.cos());
    let target_height = if cfg.cinematic { 2.0 } else { 1.2 };
    let camera_target =
        Vec3::new(0.0, target_height + cfg.look_y, -1.0) + camera_right * cfg.look_x;
    let field_of_view = if cfg.cinematic { 50.0 } else { 44.0 };
    let camera = Camera::look_at(camera_pos, camera_target, field_of_view, aspect);
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
            scope.spawn(move || {
                render_rows(scene, bvh, camera, cfg, start_y, pixel_chunk);
            });
        }
    });

    pixels
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
                    color += trace(
                        scene,
                        &bvh,
                        camera.ray(u, v),
                        0,
                        cfg.max_depth,
                        cfg.hd,
                        cfg.realtime_preview,
                    );
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

fn trace(
    scene: &Scene,
    bvh: &Bvh,
    ray: Ray,
    depth: u32,
    max_depth: u32,
    hd: bool,
    realtime_preview: bool,
) -> Color {
    if depth >= max_depth {
        return skybox(ray.direction);
    }

    if let Some(hit) = intersect_scene(scene, bvh, ray) {
        shade(scene, bvh, ray, hit, depth, max_depth, hd, realtime_preview)
    } else {
        skybox(ray.direction)
    }
}

fn shade(
    scene: &Scene,
    bvh: &Bvh,
    ray: Ray,
    hit: Hit,
    depth: u32,
    max_depth: u32,
    hd: bool,
    realtime_preview: bool,
) -> Color {
    let view_dir = -ray.direction;
    let light_dir = -scene.light_dir.normalized();
    let geometric_normal = hit.normal;
    let normal = if hd {
        hit.material.detailed_normal(hit.point, geometric_normal)
    } else {
        geometric_normal
    };
    let base = hit.material.texture(hit.point, normal);

    let visibility = if realtime_preview {
        1.0
    } else {
        let shadow_samples = if max_depth <= 1 { 1 } else { 3 };
        soft_shadow(
            scene,
            bvh,
            hit.point,
            geometric_normal,
            light_dir,
            shadow_samples,
        )
    };
    let contact = if hd && depth == 0 {
        ambient_visibility(scene, bvh, hit.point, geometric_normal)
    } else {
        1.0
    };
    let ndotl = normal.dot(light_dir).max(0.0);
    let diffuse_strength = ndotl * (0.10 + visibility * 0.90);
    let diffuse = base.hadamard(scene.light_color) * diffuse_strength * (0.72 + contact * 0.28);
    let fill_dir = Vec3::new(-0.62, 0.34, 0.48).normalized();
    let fill = normal.dot(fill_dir).max(0.0) * 0.12;
    let fill_light = base.hadamard(Color::new(0.28, 0.39, 0.55)) * fill;

    let half_vec = (light_dir + view_dir).normalized();
    let spec = normal.dot(half_vec).max(0.0).powf(hit.material.shininess())
        * hit.material.specular
        * visibility;
    let specular = scene.light_color * spec;
    let sky_ambient = Color::new(0.12, 0.20, 0.32);
    let ground_bounce = Color::new(0.28, 0.12, 0.04);
    let upward = normal.y.max(0.0);
    let downward = (-normal.y).max(0.0);
    let ambient = base * 0.072
        + base.hadamard(sky_ambient) * (0.10 + upward * 0.08)
        + base.hadamard(ground_bounce) * downward * 0.07;
    let rim = (1.0 - normal.dot(view_dir).max(0.0)).powf(3.5) * 0.12;
    let rim_light = Color::new(0.28, 0.44, 0.78) * rim;

    let mut color =
        ambient * contact + diffuse + fill_light + specular + rim_light * (0.72 + contact * 0.28);

    if !realtime_preview && hit.material.reflectivity >= SECONDARY_RAY_THRESHOLD {
        let reflected = ray.direction.reflect(normal).normalized();
        let reflected_color = trace(
            scene,
            bvh,
            Ray {
                origin: hit.point + geometric_normal * EPSILON,
                direction: reflected,
            },
            depth + 1,
            max_depth,
            hd,
            realtime_preview,
        );
        let facing = (-ray.direction.dot(hit.normal)).abs().clamp(0.0, 1.0);
        let fresnel = hit.material.reflectivity * (0.52 + 0.48 * (1.0 - facing).powf(5.0));
        color = color * (1.0 - fresnel) + reflected_color * fresnel;
    }

    if !realtime_preview && hit.material.transparency >= SECONDARY_RAY_THRESHOLD {
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
            hd,
            realtime_preview,
        );
        color =
            color * (1.0 - hit.material.transparency) + refracted_color * hit.material.transparency;
    }

    let distance_fog = ((hit.distance - 16.0) / 38.0).clamp(0.0, 1.0);
    let low_dust = (1.0 - (hit.point.y / 11.0).clamp(0.0, 1.0)) * distance_fog;
    let fog = (distance_fog * 0.18 + low_dust * 0.09).min(0.29);
    let fog_color = skybox(ray.direction) * 0.80 + Color::new(0.46, 0.34, 0.20) * 0.20;
    color = color * (1.0 - fog) + fog_color * fog;
    Color::new(color.x.max(0.0), color.y.max(0.0), color.z.max(0.0))
}

fn ambient_visibility(scene: &Scene, bvh: &Bvh, point: Vec3, normal: Vec3) -> f32 {
    let reference = if normal.y.abs() < 0.92 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let tangent = reference.cross(normal).normalized();
    let bitangent = normal.cross(tangent).normalized();
    let phase = (point.x * 1.73 + point.y * 2.31 + point.z * 2.93).sin() * PI;
    let max_distance = 2.2;
    let mut occlusion = 0.0;

    for sample in 0..3 {
        let angle = phase + 2.0 * PI * sample as f32 / 3.0;
        let direction =
            (normal * 0.72 + tangent * angle.cos() * 0.52 + bitangent * angle.sin() * 0.52)
                .normalized();
        let ray = Ray {
            origin: point + normal * EPSILON * 3.0,
            direction,
        };
        let mut nearest = bvh
            .nearest(&scene.cubes, ray)
            .map(|hit| hit.1)
            .unwrap_or(f32::INFINITY);
        if let Some((_, distance, _)) = bvh.nearest_triangle(&scene.triangles, ray) {
            nearest = nearest.min(distance);
        }
        if let Some((_, distance, _)) = bvh.nearest_ellipsoid(&scene.ellipsoids, ray) {
            nearest = nearest.min(distance);
        }
        if let Some((_, distance, _)) = bvh.nearest_capsule(&scene.capsules, ray) {
            nearest = nearest.min(distance);
        }
        if nearest < max_distance {
            occlusion += 1.0 - nearest / max_distance;
        }
    }

    (1.0 - occlusion / 3.0 * 0.78).clamp(0.28, 1.0)
}

fn soft_shadow(
    scene: &Scene,
    bvh: &Bvh,
    point: Vec3,
    normal: Vec3,
    light_dir: Vec3,
    sample_count: usize,
) -> f32 {
    let tangent = light_dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalized();
    let bitangent = tangent.cross(light_dir).normalized();
    let offsets = [(0.0, 0.0), (0.045, -0.025), (-0.035, 0.040)];
    let mut visible = 0.0;

    for &(x, y) in offsets.iter().take(sample_count) {
        let direction = (light_dir + tangent * x + bitangent * y).normalized();
        let shadow_ray = Ray {
            origin: point + normal * EPSILON,
            direction,
        };
        if !bvh.any_hit(&scene.cubes, shadow_ray)
            && !bvh.any_triangle_hit(&scene.triangles, shadow_ray)
            && !bvh.any_ellipsoid_hit(&scene.ellipsoids, shadow_ray)
            && !bvh.any_capsule_hit(&scene.capsules, shadow_ray)
        {
            visible += 1.0;
        }
    }

    visible / sample_count as f32
}

fn tone_map(color: Color) -> Color {
    fn aces(value: f32) -> f32 {
        let value = value * 1.28;
        ((value * (2.51 * value + 0.03)) / (value * (2.43 * value + 0.59) + 0.14))
            .clamp(0.0, 1.0)
            .powf(1.0 / 2.2)
    }

    let mapped = Color::new(aces(color.x), aces(color.y), aces(color.z));
    let luminance = mapped.x * 0.2126 + mapped.y * 0.7152 + mapped.z * 0.0722;
    let gray = Color::new(luminance, luminance, luminance);
    (gray + (mapped - gray) * 1.08).clamp01()
}

fn intersect_scene(scene: &Scene, bvh: &Bvh, ray: Ray) -> Option<Hit> {
    let cube_hit = bvh
        .nearest(&scene.cubes, ray)
        .map(|(index, distance, geometric_normal)| {
            let mut normal = scene.cubes[index].smooth_normal.unwrap_or(geometric_normal);
            if normal.dot(ray.direction) > 0.0 {
                normal = -normal;
            }
            Hit {
                point: ray.at(distance),
                normal,
                material: scene.materials[scene.cubes[index].material],
                distance,
            }
        });
    let mut closest = cube_hit;
    if let Some((index, distance, mut normal)) = bvh.nearest_ellipsoid(&scene.ellipsoids, ray) {
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }
        if closest
            .as_ref()
            .map(|hit| distance < hit.distance)
            .unwrap_or(true)
        {
            closest = Some(Hit {
                point: ray.at(distance),
                normal,
                material: scene.materials[scene.ellipsoids[index].material],
                distance,
            });
        }
    }
    if let Some((index, distance, mut normal)) = bvh.nearest_capsule(&scene.capsules, ray) {
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }
        if closest
            .as_ref()
            .map(|hit| distance < hit.distance)
            .unwrap_or(true)
        {
            closest = Some(Hit {
                point: ray.at(distance),
                normal,
                material: scene.materials[scene.capsules[index].material],
                distance,
            });
        }
    }
    if let Some((index, distance, mut normal)) = bvh.nearest_triangle(&scene.triangles, ray) {
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }
        if closest
            .as_ref()
            .map(|hit| distance < hit.distance)
            .unwrap_or(true)
        {
            closest = Some(Hit {
                point: ray.at(distance),
                normal,
                material: scene.materials[scene.triangles[index].material],
                distance,
            });
        }
    }
    closest
}

fn skybox(dir: Vec3) -> Color {
    let t = ((dir.y + 0.12) * 2.05).clamp(0.0, 1.0);
    let horizon = Color::new(0.20, 0.43, 0.68);
    let zenith = Color::new(0.025, 0.18, 0.58);
    let mut color = horizon * (1.0 - t) + zenith * t;

    let sun_dir = Vec3::new(-0.42, 0.29, -0.86).normalized();
    let sun = dir.dot(sun_dir).max(0.0).powf(420.0);
    let inner_glow = dir.dot(sun_dir).max(0.0).powf(42.0);
    let outer_glow = dir.dot(sun_dir).max(0.0).powf(8.0);
    color += Color::new(1.0, 0.90, 0.62) * sun * 1.35;
    color += Color::new(0.96, 0.62, 0.26) * inner_glow * 0.30;
    color += Color::new(0.72, 0.36, 0.16) * outer_glow * 0.08;

    let cloud_band = (1.0 - (dir.y - 0.14).abs() * 6.0).max(0.0);
    let cloud_shape =
        ((dir.x * 18.0 + dir.z * 11.0).sin() + (dir.x * 31.0 - dir.z * 7.0).sin() * 0.45) * 0.5
            + 0.42;
    color += Color::new(0.66, 0.72, 0.80) * cloud_shape.max(0.0) * cloud_band * 0.36;

    let high_band = (1.0 - (dir.y - 0.42).abs() * 8.0).max(0.0);
    let high_shape = ((dir.x * 27.0 - dir.z * 19.0).sin() * 0.55
        + (dir.x * 43.0 + dir.z * 13.0).sin() * 0.25
        + 0.38)
        .max(0.0);
    color += Color::new(0.50, 0.61, 0.76) * high_shape * high_band * 0.22;

    let dust_haze = (1.0 - (dir.y + 0.02).abs() * 4.2).max(0.0);
    color += Color::new(0.34, 0.24, 0.13) * dust_haze * 0.11;
    color.clamp01()
}

fn build_scene() -> Scene {
    let mut scene = Scene {
        cubes: Vec::new(),
        ellipsoids: Vec::new(),
        capsules: Vec::new(),
        triangles: Vec::new(),
        materials: scene_materials(),
        light_dir: Vec3::new(0.48, -0.78, -0.34).normalized(),
        light_color: Color::new(1.0, 0.88, 0.68),
    };

    build_sage_arrival(&mut scene);
    scene
}

fn build_sage_arrival(scene: &mut Scene) {
    const CRATER_DEPTH: f32 = 8.0;
    add_block(
        scene,
        Vec3::new(0.0, -0.65 - CRATER_DEPTH, 0.0),
        Vec3::new(24.0, 1.3, 12.0),
        0,
    );
    add_block(
        scene,
        Vec3::new(0.0, 0.08 - CRATER_DEPTH, 0.4),
        Vec3::new(16.0, 0.25, 7.5),
        17,
    );

    add_destroyed_konoha(scene);
    add_six_paths(scene);
    let first_character_cube = scene.cubes.len();
    let first_character_ellipsoid = scene.ellipsoids.len();
    let first_character_capsule = scene.capsules.len();
    let first_character_triangle = scene.triangles.len();
    add_gamabunta(scene, Vec3::new(0.0, 0.0, -0.3));
    add_gamaken(scene, Vec3::new(-12.4, 0.0, -1.3));
    add_gamahiro(scene, Vec3::new(12.4, 0.0, -1.3));
    add_gamakichi(scene, Vec3::new(0.0, 14.00, 0.12));
    add_naruto_sage(scene, Vec3::new(0.0, 18.22, 0.30));
    add_summoning_clouds(scene);
    for cube in &mut scene.cubes[first_character_cube..] {
        cube.min.y -= CRATER_DEPTH;
        cube.max.y -= CRATER_DEPTH;
    }
    for ellipsoid in &mut scene.ellipsoids[first_character_ellipsoid..] {
        ellipsoid.center.y -= CRATER_DEPTH;
    }
    for capsule in &mut scene.capsules[first_character_capsule..] {
        capsule.start.y -= CRATER_DEPTH;
        capsule.end.y -= CRATER_DEPTH;
    }
    for triangle in &mut scene.triangles[first_character_triangle..] {
        for vertex in &mut triangle.vertices {
            vertex.y -= CRATER_DEPTH;
        }
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
            smooth_normal: None,
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
    fn ellipsoid_intersection_returns_smooth_normal() {
        let ellipsoid = Ellipsoid {
            center: Vec3::default(),
            radii: Vec3::new(1.0, 1.5, 1.0),
            material: 0,
        };
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -3.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let (distance, normal) =
            intersect_ellipsoid(ray, ellipsoid).expect("ray should hit ellipsoid");

        assert_close(distance, 2.0);
        assert_close(normal.z, -1.0);
    }

    #[test]
    fn capsule_intersection_returns_rounded_surface_normal() {
        let capsule = Capsule {
            start: Vec3::new(0.0, -1.0, 0.0),
            end: Vec3::new(0.0, 1.0, 0.0),
            radius: 0.5,
            material: 0,
        };
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -3.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let (distance, normal) = intersect_capsule(ray, capsule).expect("ray should hit capsule");

        assert_close(distance, 2.5);
        assert_close(normal.z, -1.0);
    }

    #[test]
    fn triangle_intersection_interpolates_a_smooth_normal() {
        let triangle = scene::Triangle {
            vertices: [
                Vec3::new(-1.0, -1.0, 0.0),
                Vec3::new(1.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ],
            normals: [Vec3::new(0.0, 0.0, -1.0); 3],
            material: 0,
        };
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -2.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let (distance, normal) =
            bvh::intersect_triangle(ray, triangle).expect("ray should hit triangle");

        assert_close(distance, 2.0);
        assert_close(normal.z, -1.0);
    }

    #[test]
    fn procedural_relief_keeps_a_unit_surface_normal() {
        let material = scene_materials()[17];
        let normal = material.detailed_normal(Vec3::new(1.7, -0.4, 2.2), Vec3::new(0.0, 1.0, 0.0));

        assert_close(normal.length(), 1.0);
        assert!(normal.y > 0.9);
    }

    #[test]
    fn cinematic_tone_mapping_stays_in_display_range() {
        let mapped = tone_map(Color::new(4.0, 1.5, 0.2));

        assert!((0.0..=1.0).contains(&mapped.x));
        assert!((0.0..=1.0).contains(&mapped.y));
        assert!((0.0..=1.0).contains(&mapped.z));
        assert!(mapped.x > mapped.y && mapped.y > mapped.z);
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
        assert!(!scene.ellipsoids.is_empty());
        assert!(!scene.capsules.is_empty());
        assert!(!scene.triangles.is_empty());
    }

    #[test]
    fn bvh_matches_brute_force_for_camera_rays() {
        let scene = build_scene();
        let bvh = Bvh::build(&scene.cubes, &scene.triangles);
        let angle = 90.0_f32.to_radians();
        let radius = 25.5 / 0.96;
        let camera = Camera::look_at(
            Vec3::new(angle.cos() * radius, 11.8, angle.sin() * radius),
            Vec3::new(0.0, 3.6, -1.0),
            46.0,
            16.0 / 9.0,
        );

        for y in 0..18 {
            for x in 0..30 {
                let ray = camera.ray((x as f32 + 0.5) / 30.0, (y as f32 + 0.5) / 18.0);
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

    #[test]
    fn smooth_shape_bvh_matches_brute_force_for_camera_rays() {
        let scene = build_scene();
        let bvh = Bvh::build_scene(
            &scene.cubes,
            &scene.triangles,
            &scene.ellipsoids,
            &scene.capsules,
        );
        let camera = Camera::look_at(
            Vec3::new(0.0, 10.0, 27.0),
            Vec3::new(0.0, 4.0, -1.0),
            46.0,
            16.0 / 9.0,
        );

        for y in 0..12 {
            for x in 0..20 {
                let ray = camera.ray((x as f32 + 0.5) / 20.0, (y as f32 + 0.5) / 12.0);
                let accelerated_ellipsoid = bvh.nearest_ellipsoid(&scene.ellipsoids, ray);
                let brute_ellipsoid = scene
                    .ellipsoids
                    .iter()
                    .enumerate()
                    .filter_map(|(index, &shape)| {
                        intersect_ellipsoid(ray, shape)
                            .map(|(distance, normal)| (index, distance, normal))
                    })
                    .min_by(|left, right| left.1.total_cmp(&right.1));
                assert_eq!(
                    accelerated_ellipsoid.map(|hit| hit.0),
                    brute_ellipsoid.map(|hit| hit.0),
                    "different ellipsoid for sample ({x}, {y})"
                );

                let accelerated_capsule = bvh.nearest_capsule(&scene.capsules, ray);
                let brute_capsule = scene
                    .capsules
                    .iter()
                    .enumerate()
                    .filter_map(|(index, &shape)| {
                        intersect_capsule(ray, shape)
                            .map(|(distance, normal)| (index, distance, normal))
                    })
                    .min_by(|left, right| left.1.total_cmp(&right.1));
                assert_eq!(
                    accelerated_capsule.map(|hit| hit.0),
                    brute_capsule.map(|hit| hit.0),
                    "different capsule for sample ({x}, {y})"
                );
            }
        }
    }
}
