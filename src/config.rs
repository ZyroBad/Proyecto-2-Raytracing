use std::env;

pub const DEFAULT_MAX_DEPTH: u32 = 3;

#[derive(Clone)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub frame: usize,
    pub frames: usize,
    pub animate: bool,
    pub interactive: bool,
    pub window: bool,
    pub summary: bool,
    pub angle_deg: Option<f32>,
    pub zoom: f32,
    pub elevation: f32,
    pub look_x: f32,
    pub look_y: f32,
    pub samples_per_axis: usize,
    pub max_depth: u32,
    pub output: String,
}

impl Config {
    pub fn from_args() -> Self {
        let mut cfg = Self {
            width: 480,
            height: 270,
            frame: 0,
            frames: 120,
            animate: false,
            interactive: false,
            window: false,
            summary: false,
            angle_deg: None,
            zoom: 1.0,
            elevation: 0.0,
            look_x: 0.0,
            look_y: 0.0,
            samples_per_axis: 2,
            max_depth: DEFAULT_MAX_DEPTH,
            output: "renders/llegada_modo_sabio.ppm".to_string(),
        };

        let args: Vec<String> = env::args().collect();
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--width" | "-w" => {
                    i += 1;
                    cfg.width = parse_or(&args, i, cfg.width);
                }
                "--height" | "-h" => {
                    i += 1;
                    cfg.height = parse_or(&args, i, cfg.height);
                }
                "--frame" => {
                    i += 1;
                    cfg.frame = parse_or(&args, i, cfg.frame);
                }
                "--frames" => {
                    i += 1;
                    cfg.frames = parse_or(&args, i, cfg.frames);
                }
                "--animate" => cfg.animate = true,
                "--interactive" => cfg.interactive = true,
                "--window" => cfg.window = true,
                "--summary" => cfg.summary = true,
                "--angle" => {
                    i += 1;
                    cfg.angle_deg = Some(parse_or(&args, i, 90.0));
                }
                "--zoom" => {
                    i += 1;
                    cfg.zoom = parse_or(&args, i, cfg.zoom);
                }
                "--elevation" => {
                    i += 1;
                    cfg.elevation = parse_or(&args, i, cfg.elevation);
                }
                "--samples" => {
                    i += 1;
                    cfg.samples_per_axis = parse_or(&args, i, cfg.samples_per_axis).clamp(1, 4);
                }
                "--depth" => {
                    i += 1;
                    cfg.max_depth = parse_or(&args, i, cfg.max_depth).clamp(1, 6);
                }
                "--output" | "-o" => {
                    i += 1;
                    if let Some(value) = args.get(i) {
                        cfg.output = value.clone();
                    }
                }
                "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => {}
            }
            i += 1;
        }
        cfg
    }
}

fn parse_or<T: std::str::FromStr>(args: &[String], index: usize, fallback: T) -> T {
    args.get(index)
        .and_then(|value| value.parse::<T>().ok())
        .unwrap_or(fallback)
}

fn print_help() {
    println!("Llegada de Naruto en Modo Sabio - Raytracing");
    println!("cargo run --release -- [opciones]");
    println!("  --width N       ancho del render, default 480");
    println!("  --height N      alto del render, default 270");
    println!("  --frame N       frame individual para camara orbital");
    println!("  --frames N      cantidad de frames para animacion");
    println!("  --animate       renderiza todos los frames en frames/");
    println!("  --interactive   ajusta camara y genera previews");
    println!("  --window        abre la vista interactiva en una ventana nativa");
    println!("  --summary       imprime resumen de escena sin renderizar");
    println!("  --angle N       angulo manual de camara en grados");
    println!("  --zoom N        zoom manual, mayor acerca la camara");
    println!("  --elevation N   desplazamiento vertical de la camara");
    println!("  --samples N     muestras por eje, 1 rapido, 2 default, 4 fino");
    println!("  --depth N       rebotes maximos de raytracing, default 3");
    println!("  --output PATH   salida BMP o PPM");
}
