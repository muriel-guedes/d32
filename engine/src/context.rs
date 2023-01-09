use std::sync::{Arc, Mutex};

use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::{window::Window, event_loop::EventLoop, dpi::PhysicalSize};

use crate::{window, Settings, Cursor, utils, RasterShader, CopyShader, Object, Camera};

#[derive(Clone)]
pub struct Context {
    pub window: Arc<Window>,
    pub settings: Arc<Settings>,
    pub surface: Arc<Surface>,
    pub surface_config: Arc<Mutex<SurfaceConfiguration>>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub cursor: Arc<Cursor>,
    pub raster_shader: Arc<RasterShader>,
    pub copy_shader: Arc<CopyShader>,
    pub objects: Arc<Mutex<Vec<Object>>>,
    pub camera: Arc<Camera>,
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

        let copy_shader = CopyShader::new(&device, surface_config.format);
        let raster_shader = RasterShader::new(&device, &window, &copy_shader.pipeline);
        let camera = Camera::new(&settings, &device, &window, &cursor, &raster_shader.pipeline);

        Self {
            window: Arc::new(window),
            settings: Arc::new(settings),
            surface: Arc::new(surface),
            surface_config: Arc::new(Mutex::new(surface_config)),
            device: Arc::new(device),
            queue: Arc::new(queue),
            cursor: Arc::new(cursor),
            raster_shader: Arc::new(raster_shader),
            copy_shader: Arc::new(copy_shader),
            objects: Arc::new(Mutex::new(Vec::new())),
            camera: Arc::new(camera)
        }
    }
    pub fn resize(&self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return }
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = new_size.width;
        surface_config.height = new_size.height;
        self.surface.configure(&self.device, &surface_config);
        self.raster_shader.resize(&self.device, new_size, &self.copy_shader.pipeline);
        self.camera.resize(&self.settings, new_size);
    }
    pub fn add_object(&self, chunks: usize) -> Object {
        let object = Object::new(&self.device, &self.raster_shader.pipeline, chunks);
        self.objects.lock().unwrap().push(object.clone());
        object
    }
    pub fn update(&self) {
        for object in self.objects.lock().unwrap().iter() {
            object.update(&self.queue)
        }
        self.camera.update(&self.queue, &self.cursor);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        log::trace!("Dropping Context");
    }
}