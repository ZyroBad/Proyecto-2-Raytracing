use crate::bvh::Bvh;
use crate::config::Config;
use crate::math::Color;
use crate::scene::Scene;
use std::io;

pub type RenderFunction = fn(&Scene, &Bvh, &Config, usize) -> Vec<Color>;

#[cfg(windows)]
pub fn run_window(scene: &Scene, mut cfg: Config, render: RenderFunction) -> io::Result<()> {
    windows::run(scene, &mut cfg, render)
}

#[cfg(not(windows))]
pub fn run_window(_scene: &Scene, _cfg: Config, _render: RenderFunction) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "El modo --window requiere Windows",
    ))
}

#[cfg(windows)]
mod windows {
    use super::*;
    use std::ffi::c_void;
    use std::mem::zeroed;
    use std::ptr::{null, null_mut};
    use std::thread;
    use std::time::{Duration, Instant};

    struct PreviewFrame {
        pixels: Vec<u8>,
        width: usize,
        height: usize,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum PreviewQuality {
        Moving,
        Settled,
        Detail,
    }

    type Handle = *mut c_void;
    type Hwnd = Handle;
    type Hdc = Handle;
    type Hinstance = Handle;
    type Hicon = Handle;
    type Hcursor = Handle;
    type Hbrush = Handle;
    type Wparam = usize;
    type Lparam = isize;
    type Lresult = isize;

    const CS_HREDRAW: u32 = 0x0002;
    const CS_VREDRAW: u32 = 0x0001;
    const WS_OVERLAPPEDWINDOW: u32 = 0x00CF_0000;
    const CW_USEDEFAULT: i32 = i32::MIN;
    const SW_SHOW: i32 = 5;
    const PM_REMOVE: u32 = 0x0001;
    const WM_DESTROY: u32 = 0x0002;
    const WM_QUIT: u32 = 0x0012;
    const DIB_RGB_COLORS: u32 = 0;
    const SRCCOPY: u32 = 0x00CC_0020;
    const BI_RGB: u32 = 0;
    const HALFTONE: i32 = 4;
    const IDC_ARROW: usize = 32_512;

    const VK_ESCAPE: i32 = 0x1B;
    const VK_LEFT: i32 = 0x25;
    const VK_UP: i32 = 0x26;
    const VK_RIGHT: i32 = 0x27;
    const VK_DOWN: i32 = 0x28;
    const VK_A: i32 = 0x41;
    const VK_D: i32 = 0x44;
    const VK_R: i32 = 0x52;
    const VK_S: i32 = 0x53;
    const VK_W: i32 = 0x57;
    const VK_OEM_PLUS: i32 = 0xBB;
    const VK_OEM_MINUS: i32 = 0xBD;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    struct Rect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    #[repr(C)]
    struct Message {
        hwnd: Hwnd,
        message: u32,
        wparam: Wparam,
        lparam: Lparam,
        time: u32,
        point: Point,
        private: u32,
    }

    type WindowProcedure = unsafe extern "system" fn(Hwnd, u32, Wparam, Lparam) -> Lresult;

    #[repr(C)]
    struct WindowClass {
        style: u32,
        window_procedure: Option<WindowProcedure>,
        class_extra: i32,
        window_extra: i32,
        instance: Hinstance,
        icon: Hicon,
        cursor: Hcursor,
        background: Hbrush,
        menu_name: *const u16,
        class_name: *const u16,
    }

    #[repr(C)]
    struct BitmapInfoHeader {
        size: u32,
        width: i32,
        height: i32,
        planes: u16,
        bit_count: u16,
        compression: u32,
        image_size: u32,
        x_pixels_per_meter: i32,
        y_pixels_per_meter: i32,
        colors_used: u32,
        colors_important: u32,
    }

    #[repr(C)]
    struct RgbQuad {
        blue: u8,
        green: u8,
        red: u8,
        reserved: u8,
    }

    #[repr(C)]
    struct BitmapInfo {
        header: BitmapInfoHeader,
        colors: [RgbQuad; 1],
    }

    #[link(name = "user32")]
    extern "system" {
        fn RegisterClassW(window_class: *const WindowClass) -> u16;
        fn CreateWindowExW(
            extended_style: u32,
            class_name: *const u16,
            window_name: *const u16,
            style: u32,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            parent: Hwnd,
            menu: Handle,
            instance: Hinstance,
            parameter: *mut c_void,
        ) -> Hwnd;
        fn DefWindowProcW(hwnd: Hwnd, message: u32, wparam: Wparam, lparam: Lparam) -> Lresult;
        fn ShowWindow(hwnd: Hwnd, command: i32) -> i32;
        fn UpdateWindow(hwnd: Hwnd) -> i32;
        fn PeekMessageW(
            message: *mut Message,
            hwnd: Hwnd,
            minimum: u32,
            maximum: u32,
            remove: u32,
        ) -> i32;
        fn TranslateMessage(message: *const Message) -> i32;
        fn DispatchMessageW(message: *const Message) -> Lresult;
        fn PostQuitMessage(exit_code: i32);
        fn DestroyWindow(hwnd: Hwnd) -> i32;
        fn GetClientRect(hwnd: Hwnd, rect: *mut Rect) -> i32;
        fn GetCursorPos(point: *mut Point) -> i32;
        fn ScreenToClient(hwnd: Hwnd, point: *mut Point) -> i32;
        fn GetDC(hwnd: Hwnd) -> Hdc;
        fn ReleaseDC(hwnd: Hwnd, dc: Hdc) -> i32;
        fn SetStretchBltMode(dc: Hdc, mode: i32) -> i32;
        fn GetAsyncKeyState(key: i32) -> i16;
        fn SetWindowTextW(hwnd: Hwnd, text: *const u16) -> i32;
        fn LoadCursorW(instance: Hinstance, cursor_name: *const u16) -> Hcursor;
    }

    #[link(name = "gdi32")]
    extern "system" {
        fn StretchDIBits(
            dc: Hdc,
            x_destination: i32,
            y_destination: i32,
            destination_width: i32,
            destination_height: i32,
            x_source: i32,
            y_source: i32,
            source_width: i32,
            source_height: i32,
            bits: *const c_void,
            info: *const BitmapInfo,
            usage: u32,
            operation: u32,
        ) -> i32;
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetModuleHandleW(module_name: *const u16) -> Hinstance;
    }

    unsafe extern "system" fn window_procedure(
        hwnd: Hwnd,
        message: u32,
        wparam: Wparam,
        lparam: Lparam,
    ) -> Lresult {
        if message == WM_DESTROY {
            PostQuitMessage(0);
            return 0;
        }
        DefWindowProcW(hwnd, message, wparam, lparam)
    }

    pub fn run(scene: &Scene, cfg: &mut Config, render: RenderFunction) -> io::Result<()> {
        configure_preview(cfg);
        cfg.angle_deg = cfg.angle_deg.or(Some(90.0));
        if (cfg.zoom - 1.0).abs() < f32::EPSILON {
            cfg.zoom = 0.78;
        }
        if cfg.look_y.abs() < f32::EPSILON {
            cfg.look_y = -3.0;
        }
        let bvh = Bvh::build_scene(
            &scene.cubes,
            &scene.triangles,
            &scene.ellipsoids,
            &scene.capsules,
        );

        unsafe {
            let instance = GetModuleHandleW(null());
            let class_name = wide("RaytracingDioramaWindow");
            let initial_title = wide("Diorama Raytracing - preparando render...");
            let window_class = WindowClass {
                style: CS_HREDRAW | CS_VREDRAW,
                window_procedure: Some(window_procedure),
                class_extra: 0,
                window_extra: 0,
                instance,
                icon: null_mut(),
                cursor: LoadCursorW(null_mut(), IDC_ARROW as *const u16),
                background: null_mut(),
                menu_name: null(),
                class_name: class_name.as_ptr(),
            };
            if RegisterClassW(&window_class) == 0 {
                return Err(io::Error::last_os_error());
            }

            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                initial_title.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                1_000,
                620,
                null_mut(),
                null_mut(),
                instance,
                null_mut(),
            );
            if hwnd.is_null() {
                return Err(io::Error::last_os_error());
            }

            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);
            println!("Ventana interactiva abierta");
            println!(
                "Mouse: dirigir mirada | A/D: orbitar | W/S: elevar | +/-: zoom | R: render | Esc: salir"
            );

            let mut frame = render_preview(scene, &bvh, cfg, render, hwnd, PreviewQuality::Settled);
            let mut previous_keys = [false; 11];
            let keys = [
                VK_A,
                VK_D,
                VK_W,
                VK_S,
                VK_LEFT,
                VK_RIGHT,
                VK_UP,
                VK_DOWN,
                VK_OEM_PLUS,
                VK_OEM_MINUS,
                VK_R,
            ];
            let mut message: Message = zeroed();
            let mut running = true;
            let mut previous_mouse = client_cursor(hwnd);
            let mut mouse_was_moving = false;
            let mut last_tick = Instant::now();
            let mut last_draw = Instant::now();
            let mut frame_changed = true;

            while running {
                while PeekMessageW(&mut message, null_mut(), 0, 0, PM_REMOVE) != 0 {
                    if message.message == WM_QUIT {
                        running = false;
                        break;
                    }
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
                if !running {
                    break;
                }
                if key_down(VK_ESCAPE) {
                    DestroyWindow(hwnd);
                    continue;
                }

                let current_keys = keys.map(|key| key_down(key));
                let pressed = |index: usize| current_keys[index] && !previous_keys[index];
                let now = Instant::now();
                let delta_seconds = now.duration_since(last_tick).as_secs_f32().min(0.10);
                last_tick = now;
                let current_mouse = client_cursor(hwnd);
                let mouse_moving = match (current_mouse, previous_mouse) {
                    (Some(current), Some(previous)) => {
                        (current.x - previous.x).abs() > 1 || (current.y - previous.y).abs() > 1
                    }
                    _ => false,
                };
                if mouse_moving {
                    let mut rect: Rect = zeroed();
                    if GetClientRect(hwnd, &mut rect) != 0 && rect.right > 1 && rect.bottom > 1 {
                        if let Some(cursor) = current_mouse {
                            let nx =
                                (cursor.x as f32 / rect.right as f32 * 2.0 - 1.0).clamp(-1.0, 1.0);
                            let ny =
                                (cursor.y as f32 / rect.bottom as f32 * 2.0 - 1.0).clamp(-1.0, 1.0);
                            cfg.look_x = nx * 8.0;
                            cfg.look_y = -3.0 - ny * 3.4;
                        }
                    }
                }
                let moving = current_keys[..10].iter().any(|&down| down) || mouse_moving;
                let angle_step = 28.0 * delta_seconds;
                let elevation_step = 3.2 * delta_seconds;
                let zoom_step = 0.48 * delta_seconds;
                if current_keys[0] || current_keys[4] {
                    cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) - angle_step);
                }
                if current_keys[1] || current_keys[5] {
                    cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) + angle_step);
                }
                if current_keys[2] || current_keys[6] {
                    cfg.elevation = (cfg.elevation + elevation_step).min(16.0);
                }
                if current_keys[3] || current_keys[7] {
                    cfg.elevation = (cfg.elevation - elevation_step).max(-10.0);
                }
                if current_keys[8] {
                    cfg.zoom = (cfg.zoom + zoom_step).min(2.5);
                }
                if current_keys[9] {
                    cfg.zoom = (cfg.zoom - zoom_step).max(0.45);
                }

                let was_moving = previous_keys[..10].iter().any(|&down| down) || mouse_was_moving;
                if moving {
                    frame = render_preview(scene, &bvh, cfg, render, hwnd, PreviewQuality::Moving);
                    frame_changed = true;
                } else if pressed(10) {
                    frame = render_preview(scene, &bvh, cfg, render, hwnd, PreviewQuality::Detail);
                    frame_changed = true;
                } else if was_moving {
                    frame = render_preview(scene, &bvh, cfg, render, hwnd, PreviewQuality::Settled);
                    frame_changed = true;
                }
                previous_keys = current_keys;
                previous_mouse = current_mouse;
                mouse_was_moving = mouse_moving;
                if frame_changed || last_draw.elapsed() >= Duration::from_millis(250) {
                    draw_frame(hwnd, frame.width, frame.height, &frame.pixels);
                    last_draw = Instant::now();
                    frame_changed = false;
                }
                thread::sleep(Duration::from_millis(16));
            }
        }
        Ok(())
    }

    fn configure_preview(cfg: &mut Config) {
        cfg.hd = false;
        let aspect = cfg.width as f32 / cfg.height.max(1) as f32;
        if cfg.width > 360 {
            cfg.width = 360;
            cfg.height = (cfg.width as f32 / aspect).round().max(1.0) as usize;
        }
        cfg.samples_per_axis = 1;
        cfg.max_depth = cfg.max_depth.min(2);
    }

    unsafe fn render_preview(
        scene: &Scene,
        bvh: &Bvh,
        cfg: &Config,
        render: RenderFunction,
        hwnd: Hwnd,
        quality: PreviewQuality,
    ) -> PreviewFrame {
        let mut render_cfg = cfg.clone();
        if quality == PreviewQuality::Moving && render_cfg.width > 224 {
            let aspect = render_cfg.width as f32 / render_cfg.height.max(1) as f32;
            render_cfg.width = 224;
            render_cfg.height = (render_cfg.width as f32 / aspect).round().max(1.0) as usize;
        }
        render_cfg.realtime_preview = quality != PreviewQuality::Detail;
        render_cfg.samples_per_axis = 1;
        render_cfg.max_depth = if quality == PreviewQuality::Detail {
            cfg.max_depth.min(2)
        } else {
            1
        };

        let started = Instant::now();
        let colors = render(scene, bvh, &render_cfg, 0);
        let elapsed = started.elapsed().as_secs_f32();
        let title = wide(&format!(
            "Diorama | angulo {:.0} | zoom {:.2} | altura {:+.1} | mirada {:+.1},{:+.1} | {} {:.2}s",
            cfg.angle_deg.unwrap_or(90.0),
            cfg.zoom,
            cfg.elevation,
            cfg.look_x,
            cfg.look_y,
            match quality {
                PreviewQuality::Moving => "movimiento",
                PreviewQuality::Settled => "enfoque",
                PreviewQuality::Detail => "detalle",
            },
            elapsed
        ));
        SetWindowTextW(hwnd, title.as_ptr());
        PreviewFrame {
            pixels: colors_to_bgra(&colors),
            width: render_cfg.width,
            height: render_cfg.height,
        }
    }

    fn colors_to_bgra(colors: &[Color]) -> Vec<u8> {
        let mut output = Vec::with_capacity(colors.len() * 4);
        for color in colors {
            output.extend_from_slice(&[
                (color.z * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.x * 255.0) as u8,
                0,
            ]);
        }
        output
    }

    unsafe fn draw_frame(hwnd: Hwnd, width: usize, height: usize, pixels: &[u8]) {
        let mut rect = Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        if GetClientRect(hwnd, &mut rect) == 0 || rect.right <= 0 || rect.bottom <= 0 {
            return;
        }
        let info = BitmapInfo {
            header: BitmapInfoHeader {
                size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                width: width as i32,
                height: -(height as i32),
                planes: 1,
                bit_count: 32,
                compression: BI_RGB,
                image_size: (width * height * 4) as u32,
                x_pixels_per_meter: 0,
                y_pixels_per_meter: 0,
                colors_used: 0,
                colors_important: 0,
            },
            colors: [RgbQuad {
                blue: 0,
                green: 0,
                red: 0,
                reserved: 0,
            }],
        };
        let dc = GetDC(hwnd);
        SetStretchBltMode(dc, HALFTONE);
        StretchDIBits(
            dc,
            0,
            0,
            rect.right - rect.left,
            rect.bottom - rect.top,
            0,
            0,
            width as i32,
            height as i32,
            pixels.as_ptr() as *const c_void,
            &info,
            DIB_RGB_COLORS,
            SRCCOPY,
        );
        ReleaseDC(hwnd, dc);
    }

    unsafe fn key_down(key: i32) -> bool {
        GetAsyncKeyState(key) < 0
    }

    unsafe fn client_cursor(hwnd: Hwnd) -> Option<Point> {
        let mut point = Point { x: 0, y: 0 };
        if GetCursorPos(&mut point) == 0 || ScreenToClient(hwnd, &mut point) == 0 {
            return None;
        }
        let mut rect = Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        if GetClientRect(hwnd, &mut rect) == 0
            || point.x < rect.left
            || point.x >= rect.right
            || point.y < rect.top
            || point.y >= rect.bottom
        {
            None
        } else {
            Some(point)
        }
    }

    fn wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }
}
