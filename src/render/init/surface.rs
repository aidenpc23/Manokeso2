use wgpu::{Backends, Surface, Device, Queue, SurfaceConfiguration};
use winit::window::Window;

pub async fn init_surface(
    window: &Window,
) -> (
    Surface,
    Device,
    Queue,
    SurfaceConfiguration,
) {
    let size = window.inner_size();

    // ==============================================
    // Setup backend, surface and render devce
    // ==============================================
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::GL,
        ..Default::default()
    });

    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Could not create window surface!")
    };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Could not get adapter!");

    println!("Loaded backend: {:?}", adapter.get_info().backend);

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None, // Trace path
        )
        .await
        .expect("Could not get device!");

    // ==============================================
    //       Configure surface
    // ==============================================
    let surface_caps = surface.get_capabilities(&adapter);
    // Set surface format to srbg
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    // create surface config
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    return (surface, device, queue, config);
}
