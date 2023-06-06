use wgpu;
use winit;
use winit::window::{Window, WindowBuilder};

pub(crate) struct State {
    window: Refcell<winit::window::Window>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl State {
    pub fn window(&self) -> &window {
        &self.window
    }

    pub(crate) fn input(&self) -> bool {
        false
    }
    // Creating some of the wgpu types requires async code
    pub(crate) async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0], //TODO: Screen settings // let modes = &surface_caps.present_modes;
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
        }
    }
}

fn create_window()  {
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

/*
    let w_id = match app
        .new_window()
        .size(1024, 576)
        .key_pressed(key_pressed)
        .view(view)
        .build()
    {
        Ok(val) => val,
        Err(_err) => {
            return Err(Box::new(rend_ox::error::RendError::new(
                "Window Builder failed",
            )))
        }
    };

    let window = match app.window(w_id) {
        None => {
            return Err(Box::new(rend_ox::error::RendError::new(
                "Invalid window id found",
            )))
        }
        Some(val) => val,
    };

    match window.set_cursor_grab(true) {
        Err(_err) => {
            return Err(Box::new(rend_ox::error::RendError::new(
                "Cursor can't be grabbed",
            )))
        }
        _ => {}
    }
    window.set_cursor_visible(false);*/
}