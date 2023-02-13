use std::sync::{Arc, Mutex};

use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::{window::Window, event_loop::EventLoop, dpi::PhysicalSize};

use crate::{window, Settings, Cursor, utils, Camera, shader, Chunks};

#[derive(Clone)]
pub struct Context {
    pub window: Arc<Window>,
    pub settings: Arc<Settings>,
    pub surface: Arc<Surface>,
    pub surface_config: Arc<Mutex<SurfaceConfiguration>>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub cursor: Arc<Cursor>,
    pub shader: Arc<wgpu::RenderPipeline>,
    pub camera: Arc<Camera>,
    pub chunks: Arc<Chunks>
}
impl Context {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let settings = Settings::read();

        let window = window::new(&settings, event_loop);

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = utils::create_adapter(&instance, &surface);
        let (device, queue) = utils::create_device_queue(&adapter);
        
        let surface_config = utils::configure_surface(&settings, &window, &device, &adapter, &surface);
        
        let cursor = Cursor::new(&window);

        let shader = shader::new(&device, surface_config.format);
        let camera = Camera::new(&device, &window, &shader);
        let chunks = Chunks::new(&device, &shader);

        Self {
            window: Arc::new(window),
            settings: Arc::new(settings),
            surface: Arc::new(surface),
            surface_config: Arc::new(Mutex::new(surface_config)),
            device: Arc::new(device),
            queue: Arc::new(queue),
            cursor: Arc::new(cursor),
            shader: Arc::new(shader),
            camera: Arc::new(camera),
            chunks: Arc::new(chunks)
        }
    }
    pub fn resize(&self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return }
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = new_size.width;
        surface_config.height = new_size.height;
        self.surface.configure(&self.device, &surface_config);
        self.camera.resize(new_size);
    }
    pub fn draw(&self) {
        self.camera.update(&self.queue);
        self.chunks.update();
        shader::draw(self);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        log::trace!("Dropping Context");
    }
}