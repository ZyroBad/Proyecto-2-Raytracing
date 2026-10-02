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
    use std::path::{Path, PathBuf};
    use std::ptr::{null, null_mut};
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, Instant};

    struct PreviewFrame {
        generation: u64,
        pixels: Vec<u8>,
        width: usize,
        height: usize,
        quality: PreviewQuality,
        camera: CameraState,
        elapsed: f32,
    }

    struct RenderRequest {
        generation: u64,
        config: Config,
        quality: PreviewQuality,
        moving_width: usize,
    }

    #[derive(Clone, Copy)]
    struct CameraState {
        angle: f32,
        zoom: f32,
        elevation: f32,
        look_x: f32,
        look_y: f32,
    }

    struct CameraTransition {
        from: CameraState,
        to: CameraState,
        started: Instant,
        duration: Duration,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum PreviewQuality {
        Moving,
        Settled,
        Detail,
    }

    struct BackgroundMusic {
        opened: bool,
        playing: bool,
    }

    impl BackgroundMusic {
        fn start() -> Self {
            let mut music = Self {
                opened: false,
                playing: false,
            };
            let Some(path) = music_asset_path() else {
                eprintln!("No se encontro assets/musica_epica_naruto.mp3");
                return music;
            };

            let open = format!(
                "open \"{}\" type mpegvideo alias diorama_bgm",
                path.to_string_lossy()
            );
            if let Err(error) = mci_command(&open) {
                eprintln!("No se pudo abrir la musica: {error}");
                return music;
            }
            music.opened = true;

            if let Err(error) = mci_command("setaudio diorama_bgm volume to 120") {
                eprintln!("No se pudo ajustar el volumen de la musica: {error}");
            }
            match mci_command("play diorama_bgm repeat") {
                Ok(()) => music.playing = true,
                Err(error) => eprintln!("No se pudo reproducir la musica: {error}"),
            }
            music
        }

        fn toggle(&mut self) {
            if !self.opened {
                return;
            }
            let (command, next_state) = if self.playing {
                ("pause diorama_bgm", false)
            } else {
                ("resume diorama_bgm", true)
            };
            match mci_command(command) {
                Ok(()) => self.playing = next_state,
                Err(error) => eprintln!("No se pudo cambiar la musica: {error}"),
            }
        }

        fn status(&self) -> &'static str {
            if self.playing {
                "ON"
            } else {
                "OFF"
            }
        }
    }

    impl Drop for BackgroundMusic {
        fn drop(&mut self) {
            if self.opened {
                let _ = mci_command("close diorama_bgm");
            }
        }
    }

    struct MenuBackground {
        bitmap: Hbitmap,
        width: i32,
        height: i32,
    }

    impl MenuBackground {
        unsafe fn load() -> Option<Self> {
            let path = asset_path("fondo_menu_konoha.bmp")?;
            let bitmap = LoadImageW(
                null_mut(),
                wide(&path.to_string_lossy()).as_ptr(),
                IMAGE_BITMAP,
                0,
                0,
                LR_LOADFROMFILE,
            );
            if bitmap.is_null() {
                eprintln!("No se pudo cargar el fondo del menu");
                return None;
            }

            let mut info: Bitmap = zeroed();
            if GetObjectW(
                bitmap,
                std::mem::size_of::<Bitmap>() as i32,
                &mut info as *mut Bitmap as *mut c_void,
            ) == 0
            {
                DeleteObject(bitmap);
                return None;
            }
            Some(Self {
                bitmap,
                width: info.width,
                height: info.height,
            })
        }
    }

    impl Drop for MenuBackground {
        fn drop(&mut self) {
            unsafe {
                DeleteObject(self.bitmap);
            }
        }
    }

    type Handle = *mut c_void;
    type Hwnd = Handle;
    type Hdc = Handle;
    type Hinstance = Handle;
    type Hicon = Handle;
    type Hcursor = Handle;
    type Hbrush = Handle;
    type Hfont = Handle;
    type Hbitmap = Handle;
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
    const IMAGE_BITMAP: u32 = 0;
    const LR_LOADFROMFILE: u32 = 0x0010;
    const HALFTONE: i32 = 4;
    const TRANSPARENT: i32 = 1;
    const DT_CENTER: u32 = 0x0001;
    const DT_VCENTER: u32 = 0x0004;
    const DT_SINGLELINE: u32 = 0x0020;
    const IDC_ARROW: usize = 32_512;

    const VK_ESCAPE: i32 = 0x1B;
    const VK_RETURN: i32 = 0x0D;
    const VK_LBUTTON: i32 = 0x01;
    const VK_LEFT: i32 = 0x25;
    const VK_UP: i32 = 0x26;
    const VK_RIGHT: i32 = 0x27;
    const VK_DOWN: i32 = 0x28;
    const VK_A: i32 = 0x41;
    const VK_D: i32 = 0x44;
    const VK_M: i32 = 0x4D;
    const VK_R: i32 = 0x52;
    const VK_S: i32 = 0x53;
    const VK_W: i32 = 0x57;
    const VK_1: i32 = 0x31;
    const VK_2: i32 = 0x32;
    const VK_3: i32 = 0x33;
    const VK_4: i32 = 0x34;
    const VK_5: i32 = 0x35;
    const VK_OEM_PLUS: i32 = 0xBB;
    const VK_OEM_MINUS: i32 = 0xBD;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
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

    #[repr(C)]
    struct Bitmap {
        bitmap_type: i32,
        width: i32,
        height: i32,
        width_bytes: i32,
        planes: u16,
        bits_per_pixel: u16,
        bits: *mut c_void,
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
        fn LoadImageW(
            instance: Hinstance,
            name: *const u16,
            image_type: u32,
            width: i32,
            height: i32,
            flags: u32,
        ) -> Handle;
        fn FillRect(dc: Hdc, rect: *const Rect, brush: Hbrush) -> i32;
        fn DrawTextW(dc: Hdc, text: *const u16, count: i32, rect: *mut Rect, format: u32) -> i32;
    }

    #[link(name = "gdi32")]
    extern "system" {
        fn CreateCompatibleDC(dc: Hdc) -> Hdc;
        fn CreateCompatibleBitmap(dc: Hdc, width: i32, height: i32) -> Hbitmap;
        fn CreateSolidBrush(color: u32) -> Hbrush;
        fn CreateFontW(
            height: i32,
            width: i32,
            escapement: i32,
            orientation: i32,
            weight: i32,
            italic: u32,
            underline: u32,
            strike_out: u32,
            char_set: u32,
            out_precision: u32,
            clip_precision: u32,
            quality: u32,
            pitch_and_family: u32,
            face_name: *const u16,
        ) -> Hfont;
        fn DeleteObject(object: Handle) -> i32;
        fn DeleteDC(dc: Hdc) -> i32;
        fn GetObjectW(object: Handle, size: i32, output: *mut c_void) -> i32;
        fn SelectObject(dc: Hdc, object: Handle) -> Handle;
        fn SetBkMode(dc: Hdc, mode: i32) -> i32;
        fn SetTextColor(dc: Hdc, color: u32) -> u32;
        fn RoundRect(
            dc: Hdc,
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
            width: i32,
            height: i32,
        ) -> i32;
        fn BitBlt(
            destination: Hdc,
            x_destination: i32,
            y_destination: i32,
            width: i32,
            height: i32,
            source: Hdc,
            x_source: i32,
            y_source: i32,
            operation: u32,
        ) -> i32;
        fn StretchBlt(
            destination: Hdc,
            x_destination: i32,
            y_destination: i32,
            destination_width: i32,
            destination_height: i32,
            source: Hdc,
            x_source: i32,
            y_source: i32,
            source_width: i32,
            source_height: i32,
            operation: u32,
        ) -> i32;
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

    #[link(name = "winmm")]
    extern "system" {
        fn mciSendStringW(
            command: *const u16,
            return_value: *mut u16,
            return_length: u32,
            callback: Hwnd,
        ) -> u32;
        fn mciGetErrorStringW(error: u32, text: *mut u16, length: u32) -> i32;
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

        thread::scope(|scope| {
            let (request_tx, request_rx) = mpsc::channel::<RenderRequest>();
            let (frame_tx, frame_rx) = mpsc::channel::<PreviewFrame>();
            scope.spawn(move || {
                while let Ok(request) = request_rx.recv() {
                    let frame = render_preview(scene, &bvh, &request, render);
                    if frame_tx.send(frame).is_err() {
                        break;
                    }
                }
            });

            let result = unsafe {
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
                let mut music = BackgroundMusic::start();
                if !run_main_menu(hwnd, &mut music) {
                    DestroyWindow(hwnd);
                    return Ok(());
                }
                println!("Ventana interactiva abierta");
                println!(
                "Mouse: dirigir mirada | A/D: orbitar | W/S: elevar | +/-: zoom | 1-5: camaras | R: detalle | M: musica | Esc: salir"
            );

                let mut frame = PreviewFrame {
                    generation: 0,
                    pixels: vec![28, 31, 28, 0],
                    width: 1,
                    height: 1,
                    quality: PreviewQuality::Settled,
                    camera: camera_state(cfg),
                    elapsed: 0.0,
                };
                let mut next_generation = 1_u64;
                let mut render_busy = false;
                let mut pending_request = Some(render_request(
                    next_generation,
                    cfg,
                    PreviewQuality::Settled,
                    288,
                ));
                let mut previous_keys = [false; 17];
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
                    VK_1,
                    VK_2,
                    VK_3,
                    VK_4,
                    VK_5,
                    VK_M,
                ];
                let mut message: Message = zeroed();
                let mut running = true;
                let mut previous_mouse = client_cursor(hwnd);
                let mut last_tick = Instant::now();
                let mut last_draw = Instant::now();
                let mut frame_changed = true;
                let mut camera_was_moving = false;
                let mut transition: Option<CameraTransition> = None;
                let mut moving_width = 288_usize;

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

                    while let Ok(completed) = frame_rx.try_recv() {
                        render_busy = false;
                        if completed.quality == PreviewQuality::Moving {
                            moving_width = adaptive_preview_width(moving_width, completed.elapsed);
                        }
                        if completed.generation > frame.generation {
                            frame = completed;
                            update_window_title(hwnd, &frame, music.status());
                            frame_changed = true;
                        }
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
                        if GetClientRect(hwnd, &mut rect) != 0 && rect.right > 1 && rect.bottom > 1
                        {
                            if let Some(cursor) = current_mouse {
                                let nx = (cursor.x as f32 / rect.right as f32 * 2.0 - 1.0)
                                    .clamp(-1.0, 1.0);
                                let ny = (cursor.y as f32 / rect.bottom as f32 * 2.0 - 1.0)
                                    .clamp(-1.0, 1.0);
                                cfg.look_x = nx * 8.0;
                                cfg.look_y = -3.0 - ny * 3.4;
                            }
                        }
                    }
                    let manual_moving = current_keys[..10].iter().any(|&down| down) || mouse_moving;
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
                        cfg.elevation += elevation_step;
                    }

                    if current_keys[3] || current_keys[7] {
                        cfg.elevation -= elevation_step;
                    }
                    if current_keys[8] {
                        cfg.zoom = (cfg.zoom + zoom_step).min(2.5);
                    }
                    if current_keys[9] {
                        cfg.zoom = (cfg.zoom - zoom_step).max(0.45);
                    }
                    if pressed(16) {
                        music.toggle();
                        update_window_title(hwnd, &frame, music.status());
                    }

                    if manual_moving {
                        transition = None;
                    } else if let Some(preset_index) = (0..5).find(|index| pressed(11 + index)) {
                        transition = Some(CameraTransition {
                            from: camera_state(cfg),
                            to: camera_preset(preset_index),
                            started: now,
                            duration: Duration::from_millis(1_350),
                        });
                    }

                    let mut transition_moving = false;
                    if let Some(active) = &transition {
                        let progress = now.duration_since(active.started).as_secs_f32()
                            / active.duration.as_secs_f32();
                        apply_camera_state(
                            cfg,
                            interpolate_camera(active.from, active.to, progress),
                        );
                        transition_moving = progress < 1.0;
                        if !transition_moving {
                            transition = None;
                        }
                    }
                    let moving = manual_moving || transition_moving;
                    if moving {
                        next_generation += 1;
                        pending_request = Some(render_request(
                            next_generation,
                            cfg,
                            PreviewQuality::Moving,
                            moving_width,
                        ));
                    } else if pressed(10) {
                        next_generation += 1;
                        pending_request = Some(render_request(
                            next_generation,
                            cfg,
                            PreviewQuality::Detail,
                            moving_width,
                        ));
                    } else if camera_was_moving {
                        next_generation += 1;
                        pending_request = Some(render_request(
                            next_generation,
                            cfg,
                            PreviewQuality::Settled,
                            moving_width,
                        ));
                    }
                    if !render_busy {
                        if let Some(request) = pending_request.take() {
                            if request_tx.send(request).is_ok() {
                                render_busy = true;
                            }
                        }
                    }
                    previous_keys = current_keys;
                    previous_mouse = current_mouse;
                    camera_was_moving = moving;
                    if frame_changed || last_draw.elapsed() >= Duration::from_millis(250) {
                        draw_frame(hwnd, frame.width, frame.height, &frame.pixels);
                        last_draw = Instant::now();
                        frame_changed = false;
                    }
                    thread::sleep(Duration::from_millis(16));
                }
                Ok(())
            };
            drop(request_tx);
            result
        })
    }

    unsafe fn run_main_menu(hwnd: Hwnd, music: &mut BackgroundMusic) -> bool {
        let background = MenuBackground::load();
        let mut selected = 0_usize;
        let mut showing_controls = false;
        let mut previous_keys = [false; 6];
        let mut message: Message = zeroed();

        SetWindowTextW(hwnd, wide("Naruto Shippuden | Konoha destruida").as_ptr());
        loop {
            while PeekMessageW(&mut message, null_mut(), 0, 0, PM_REMOVE) != 0 {
                if message.message == WM_QUIT {
                    return false;
                }
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }

            let current_keys = [
                key_down(VK_UP),
                key_down(VK_DOWN),
                key_down(VK_RETURN),
                key_down(VK_M),
                key_down(VK_LBUTTON),
                key_down(VK_ESCAPE),
            ];
            let pressed = |index: usize| current_keys[index] && !previous_keys[index];

            if showing_controls {
                if pressed(2) || pressed(5) || pressed(4) {
                    showing_controls = false;
                }
            } else {
                if pressed(5) {
                    return false;
                }
                if pressed(0) {
                    selected = (selected + 3) % 4;
                }
                if pressed(1) {
                    selected = (selected + 1) % 4;
                }
                if pressed(3) {
                    music.toggle();
                }

                let hovered = client_cursor(hwnd).and_then(|point| menu_hit_test(hwnd, point));
                if let Some(index) = hovered {
                    selected = index;
                }
                let activated = if pressed(2) {
                    Some(selected)
                } else if pressed(4) {
                    hovered
                } else {
                    None
                };
                match activated {
                    Some(0) => return true,
                    Some(1) => showing_controls = true,
                    Some(2) => music.toggle(),
                    Some(3) => return false,
                    _ => {}
                }
            }

            draw_main_menu(
                hwnd,
                selected,
                music.status(),
                showing_controls,
                background.as_ref(),
            );
            previous_keys = current_keys;
            thread::sleep(Duration::from_millis(33));
        }
    }

    unsafe fn draw_main_menu(
        hwnd: Hwnd,
        selected: usize,
        music_status: &str,
        showing_controls: bool,
        background: Option<&MenuBackground>,
    ) {
        let mut client: Rect = zeroed();
        if GetClientRect(hwnd, &mut client) == 0 || client.right <= 0 || client.bottom <= 0 {
            return;
        }
        let window_dc = GetDC(hwnd);
        let dc = CreateCompatibleDC(window_dc);
        let bitmap = CreateCompatibleBitmap(window_dc, client.right, client.bottom);
        let previous_bitmap = SelectObject(dc, bitmap);
        SetBkMode(dc, TRANSPARENT);

        if let Some(background) = background {
            let source_dc = CreateCompatibleDC(window_dc);
            let previous_source = SelectObject(source_dc, background.bitmap);
            SetStretchBltMode(dc, HALFTONE);
            StretchBlt(
                dc,
                0,
                0,
                client.right,
                client.bottom,
                source_dc,
                0,
                0,
                background.width,
                background.height,
                SRCCOPY,
            );
            SelectObject(source_dc, previous_source);
            DeleteDC(source_dc);
        } else {
            let band_count = 14;
            for band in 0..band_count {
                let progress = band as f32 / (band_count - 1) as f32;
                let color = rgb(
                    (48.0 + 74.0 * progress) as u8,
                    (15.0 + 18.0 * progress) as u8,
                    (19.0 + 5.0 * progress) as u8,
                );
                let band_rect = Rect {
                    left: 0,
                    top: client.bottom * band / band_count,
                    right: client.right,
                    bottom: client.bottom * (band + 1) / band_count + 1,
                };
                fill_rect(dc, band_rect, color);
            }
        }

        fill_rect(
            dc,
            Rect {
                left: 0,
                top: 0,
                right: client.right,
                bottom: 10,
            },
            rgb(12, 10, 12),
        );
        fill_rect(
            dc,
            Rect {
                left: 0,
                top: client.bottom - 12,
                right: client.right,
                bottom: client.bottom,
            },
            rgb(12, 10, 12),
        );

        let title_font = create_menu_font(-80, 900, true, "Arial Black");
        draw_stroked_text(
            dc,
            "NARUTO",
            Rect {
                left: 0,
                top: 14,
                right: client.right,
                bottom: 124,
            },
            title_font,
            rgb(248, 84, 22),
            6,
        );
        DeleteObject(title_font);

        let shippuden_font = create_menu_font(-40, 900, true, "Arial Black");
        draw_stroked_text(
            dc,
            "SHIPPUDEN",
            Rect {
                left: 0,
                top: 105,
                right: client.right,
                bottom: 166,
            },
            shippuden_font,
            rgb(35, 148, 218),
            4,
        );
        DeleteObject(shippuden_font);

        let subtitle_font = create_menu_font(-17, 700, false, "Arial");
        draw_stroked_text(
            dc,
            "KONOHA DESTRUIDA  |  LA INVASION DE PAIN",
            Rect {
                left: 0,
                top: 164,
                right: client.right,
                bottom: 198,
            },
            subtitle_font,
            rgb(244, 205, 126),
            2,
        );
        DeleteObject(subtitle_font);

        if showing_controls {
            draw_controls_panel(dc, client);
        } else {
            let options = [
                "INICIAR",
                "CONTROLES",
                if music_status == "ON" {
                    "MUSICA: ON"
                } else {
                    "MUSICA: OFF"
                },
                "SALIR",
            ];
            let button_font = create_menu_font(-21, 800, false, "Arial");
            for (index, button) in menu_button_rects(client).into_iter().enumerate() {
                let active = index == selected;
                fill_round_rect(
                    dc,
                    button,
                    if active {
                        rgb(220, 71, 24)
                    } else {
                        rgb(25, 24, 29)
                    },
                );
                let accent = Rect {
                    left: button.left + 9,
                    top: button.top + 9,
                    right: button.left + 14,
                    bottom: button.bottom - 9,
                };
                fill_rect(
                    dc,
                    accent,
                    if active {
                        rgb(255, 194, 45)
                    } else {
                        rgb(124, 48, 35)
                    },
                );
                draw_text(
                    dc,
                    options[index],
                    button,
                    button_font,
                    if active {
                        rgb(255, 245, 220)
                    } else {
                        rgb(212, 205, 194)
                    },
                );
            }
            DeleteObject(button_font);
        }

        BitBlt(
            window_dc,
            0,
            0,
            client.right,
            client.bottom,
            dc,
            0,
            0,
            SRCCOPY,
        );
        SelectObject(dc, previous_bitmap);
        DeleteObject(bitmap);
        DeleteDC(dc);
        ReleaseDC(hwnd, window_dc);
    }

    unsafe fn draw_controls_panel(dc: Hdc, client: Rect) {
        let panel = Rect {
            left: client.right / 2 - 270,
            top: 207,
            right: client.right / 2 + 270,
            bottom: (client.bottom - 25).min(430),
        };
        fill_round_rect(dc, panel, rgb(22, 21, 26));
        let heading = create_menu_font(-27, 800, false, "Arial");
        draw_text(
            dc,
            "CONTROLES",
            Rect {
                bottom: panel.top + 43,
                ..panel
            },
            heading,
            rgb(245, 145, 38),
        );
        DeleteObject(heading);

        let body = create_menu_font(-16, 600, false, "Arial");
        let lines = [
            "MOUSE  Dirigir la mirada",
            "A / D  Orbitar     W / S  Elevar",
            "+ / -  Zoom       1 - 5  Camaras",
            "R  Maximo detalle     M  Musica",
            "ENTER O CLIC PARA VOLVER",
        ];
        let line_count = lines.len();
        for (index, line) in lines.into_iter().enumerate() {
            draw_text(
                dc,
                line,
                Rect {
                    left: panel.left + 20,
                    top: panel.top + 42 + index as i32 * 34,
                    right: panel.right - 20,
                    bottom: panel.top + 74 + index as i32 * 34,
                },
                body,
                if index == line_count - 1 {
                    rgb(245, 145, 38)
                } else {
                    rgb(236, 230, 218)
                },
            );
        }
        DeleteObject(body);
    }

    unsafe fn menu_hit_test(hwnd: Hwnd, point: Point) -> Option<usize> {
        let mut client: Rect = zeroed();
        if GetClientRect(hwnd, &mut client) == 0 {
            return None;
        }
        menu_button_rects(client).iter().position(|rect| {
            point.x >= rect.left
                && point.x < rect.right
                && point.y >= rect.top
                && point.y < rect.bottom
        })
    }

    fn menu_button_rects(client: Rect) -> [Rect; 4] {
        let width = 360.min(client.right.saturating_sub(40));
        let left = (client.right - width) / 2;
        let top = 207;
        std::array::from_fn(|index| Rect {
            left,
            top: top + index as i32 * 55,
            right: left + width,
            bottom: top + index as i32 * 55 + 44,
        })
    }

    unsafe fn create_menu_font(height: i32, weight: i32, italic: bool, face: &str) -> Hfont {
        CreateFontW(
            height,
            0,
            0,
            0,
            weight,
            italic as u32,
            0,
            0,
            1,
            0,
            0,
            5,
            0,
            wide(face).as_ptr(),
        )
    }

    unsafe fn draw_stroked_text(
        dc: Hdc,
        text: &str,
        rect: Rect,
        font: Hfont,
        fill: u32,
        stroke: i32,
    ) {
        for (offset, color) in [(stroke, rgb(8, 8, 10)), (stroke / 2, rgb(248, 242, 226))] {
            for (x, y) in [
                (-offset, 0),
                (offset, 0),
                (0, -offset),
                (0, offset),
                (-offset, -offset),
                (offset, -offset),
                (-offset, offset),
                (offset, offset),
            ] {
                draw_text(dc, text, offset_rect(rect, x, y), font, color);
            }
        }
        draw_text(dc, text, rect, font, fill);
    }

    unsafe fn draw_text(dc: Hdc, text: &str, mut rect: Rect, font: Hfont, color: u32) {
        let previous = SelectObject(dc, font);
        SetTextColor(dc, color);
        let text = wide(text);
        DrawTextW(
            dc,
            text.as_ptr(),
            text.len() as i32 - 1,
            &mut rect,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );
        SelectObject(dc, previous);
    }

    unsafe fn fill_rect(dc: Hdc, rect: Rect, color: u32) {
        let brush = CreateSolidBrush(color);
        FillRect(dc, &rect, brush);
        DeleteObject(brush);
    }

    unsafe fn fill_round_rect(dc: Hdc, rect: Rect, color: u32) {
        let brush = CreateSolidBrush(color);
        let previous = SelectObject(dc, brush);
        RoundRect(dc, rect.left, rect.top, rect.right, rect.bottom, 12, 12);
        SelectObject(dc, previous);
        DeleteObject(brush);
    }

    fn offset_rect(rect: Rect, x: i32, y: i32) -> Rect {
        Rect {
            left: rect.left + x,
            top: rect.top + y,
            right: rect.right + x,
            bottom: rect.bottom + y,
        }
    }

    fn rgb(red: u8, green: u8, blue: u8) -> u32 {
        red as u32 | ((green as u32) << 8) | ((blue as u32) << 16)
    }

    fn configure_preview(cfg: &mut Config) {
        cfg.hd = false;
        let aspect = cfg.width as f32 / cfg.height.max(1) as f32;
        if cfg.width > 480 {
            cfg.width = 480;
            cfg.height = (cfg.width as f32 / aspect).round().max(1.0) as usize;
        }
        cfg.samples_per_axis = 1;
        cfg.max_depth = cfg.max_depth.min(2);
    }

    fn render_request(
        generation: u64,
        cfg: &Config,
        quality: PreviewQuality,
        moving_width: usize,
    ) -> RenderRequest {
        RenderRequest {
            generation,
            config: cfg.clone(),
            quality,
            moving_width,
        }
    }

    fn camera_preset(index: usize) -> CameraState {
        match index {
            0 => CameraState {
                angle: 90.0,
                zoom: 0.78,
                elevation: -1.2,
                look_x: 0.0,
                look_y: -3.0,
            },
            1 => CameraState {
                angle: 90.0,
                zoom: 1.12,
                elevation: -2.4,
                look_x: 0.0,
                look_y: -2.5,
            },
            2 => CameraState {
                angle: 32.0,
                zoom: 0.90,
                elevation: 1.8,
                look_x: 4.5,
                look_y: -4.2,
            },
            3 => CameraState {
                angle: 90.0,
                zoom: 0.66,
                elevation: 4.8,
                look_x: 0.0,
                look_y: 5.8,
            },
            _ => CameraState {
                angle: 102.0,
                zoom: 0.58,
                elevation: 13.0,
                look_x: 0.0,
                look_y: -2.2,
            },
        }
    }

    fn interpolate_camera(from: CameraState, to: CameraState, progress: f32) -> CameraState {
        let t = progress.clamp(0.0, 1.0);
        let smooth = t * t * (3.0 - 2.0 * t);
        let angle_delta = (to.angle - from.angle + 180.0).rem_euclid(360.0) - 180.0;
        CameraState {
            angle: from.angle + angle_delta * smooth,
            zoom: lerp(from.zoom, to.zoom, smooth),
            elevation: lerp(from.elevation, to.elevation, smooth),
            look_x: lerp(from.look_x, to.look_x, smooth),
            look_y: lerp(from.look_y, to.look_y, smooth),
        }
    }

    fn apply_camera_state(cfg: &mut Config, state: CameraState) {
        cfg.angle_deg = Some(state.angle.rem_euclid(360.0));
        cfg.zoom = state.zoom;
        cfg.elevation = state.elevation;
        cfg.look_x = state.look_x;
        cfg.look_y = state.look_y;
    }

    fn lerp(from: f32, to: f32, amount: f32) -> f32 {
        from + (to - from) * amount
    }

    fn adaptive_preview_width(current: usize, elapsed: f32) -> usize {
        if elapsed > 0.34 {
            current.saturating_sub(16).max(256)
        } else if elapsed < 0.16 {
            (current + 16).min(320)
        } else {
            current
        }
    }

    fn camera_state(cfg: &Config) -> CameraState {
        CameraState {
            angle: cfg.angle_deg.unwrap_or(90.0),
            zoom: cfg.zoom,
            elevation: cfg.elevation,
            look_x: cfg.look_x,
            look_y: cfg.look_y,
        }
    }

    fn render_preview(
        scene: &Scene,
        bvh: &Bvh,
        request: &RenderRequest,
        render: RenderFunction,
    ) -> PreviewFrame {
        let mut render_cfg = request.config.clone();
        if request.quality == PreviewQuality::Moving && render_cfg.width > request.moving_width {
            let aspect = render_cfg.width as f32 / render_cfg.height.max(1) as f32;
            render_cfg.width = request.moving_width;
            render_cfg.height = (render_cfg.width as f32 / aspect).round().max(1.0) as usize;
        }
        render_cfg.realtime_preview = request.quality == PreviewQuality::Moving;
        render_cfg.hd = true;
        render_cfg.samples_per_axis = if request.quality == PreviewQuality::Detail {
            2
        } else {
            1
        };
        render_cfg.max_depth = if request.quality == PreviewQuality::Detail {
            request.config.max_depth.min(2)
        } else if request.quality == PreviewQuality::Settled {
            2
        } else {
            1
        };

        let started = Instant::now();
        let colors = render(scene, bvh, &render_cfg, 0);
        let elapsed = started.elapsed().as_secs_f32();
        PreviewFrame {
            generation: request.generation,
            pixels: colors_to_bgra(&colors),
            width: render_cfg.width,
            height: render_cfg.height,
            quality: request.quality,
            camera: camera_state(&request.config),
            elapsed,
        }
    }

    unsafe fn update_window_title(hwnd: Hwnd, frame: &PreviewFrame, music_status: &str) {
        let title = wide(&format!(
            "Diorama | {}x{} | angulo {:.0} | zoom {:.2} | altura {:+.1} | mirada {:+.1},{:+.1} | {} {:.2}s | musica {}",
            frame.width,
            frame.height,
            frame.camera.angle,
            frame.camera.zoom,
            frame.camera.elevation,
            frame.camera.look_x,
            frame.camera.look_y,
            match frame.quality {
                PreviewQuality::Moving => "movimiento",
                PreviewQuality::Settled => "enfoque",
                PreviewQuality::Detail => "detalle",
            },
            frame.elapsed,
            music_status
        ));
        SetWindowTextW(hwnd, title.as_ptr());
    }

    fn music_asset_path() -> Option<PathBuf> {
        asset_path("musica_epica_naruto.mp3")
    }

    fn asset_path(file_name: &str) -> Option<PathBuf> {
        let relative = Path::new("assets").join(file_name);
        let mut candidates = vec![relative.clone()];
        if let Ok(executable) = std::env::current_exe() {
            if let Some(directory) = executable.parent() {
                candidates.push(directory.join(&relative));
                if let Some(project) = directory.parent().and_then(Path::parent) {
                    candidates.push(project.join(&relative));
                }
            }
        }
        candidates
            .into_iter()
            .find(|path| path.is_file())
            .and_then(|path| std::fs::canonicalize(path).ok())
    }

    fn mci_command(command: &str) -> Result<(), String> {
        let error = unsafe { mciSendStringW(wide(command).as_ptr(), null_mut(), 0, null_mut()) };
        if error == 0 {
            return Ok(());
        }

        let mut message = [0_u16; 256];
        let found =
            unsafe { mciGetErrorStringW(error, message.as_mut_ptr(), message.len() as u32) != 0 };
        if found {
            let length = message
                .iter()
                .position(|&character| character == 0)
                .unwrap_or(message.len());
            Err(String::from_utf16_lossy(&message[..length]))
        } else {
            Err(format!("codigo MCI {error}"))
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn cinematic_camera_uses_the_shortest_orbit() {
            let from = CameraState {
                angle: 350.0,
                zoom: 1.0,
                elevation: 0.0,
                look_x: 0.0,
                look_y: 0.0,
            };
            let to = CameraState {
                angle: 10.0,
                ..from
            };
            let halfway = interpolate_camera(from, to, 0.5);
            assert!((halfway.angle - 360.0).abs() < 0.001);
        }

        #[test]
        fn adaptive_resolution_preserves_the_quality_floor() {
            assert_eq!(adaptive_preview_width(256, 0.5), 256);
            assert_eq!(adaptive_preview_width(320, 0.05), 320);
            assert_eq!(adaptive_preview_width(288, 0.5), 272);
            assert_eq!(adaptive_preview_width(288, 0.05), 304);
        }
    }
}
