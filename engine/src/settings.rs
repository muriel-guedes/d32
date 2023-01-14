use std::io::Read;

use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub window_size: Option<[u32;2]>,
    pub window_fullscreen: bool,
    pub window_decorations: bool,
    pub window_maximized: bool,
    pub window_position: Option<[u32;2]>,
    pub vsync: bool,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}
impl Settings {
    pub fn read() -> Self {
        let user_dir = UserDirs::new().expect("Failed to get user directory");
        let doc_dir = user_dir.document_dir().expect("Failed to get user document directory");
        let set_dir = doc_dir.join(env!("DOC_PATH"));
        let set_path = set_dir.join("settings.json");
        log::info!("Settings path: {set_path:?}");

        std::fs::create_dir_all(&set_dir).expect(&format!("Error creating directory: {set_dir:?}", ));
        
        let mut file = std::fs::OpenOptions::new().create(true).read(true).write(true).truncate(false).open(&set_path)
            .expect(&format!("Failed to open path: {set_path:?}"));
        
        let mut content = String::new();
        file.read_to_string(&mut content).expect(&format!("Error reading file: {set_path:?}"));

        if content.len() == 0 {
            let default = Self::default();
            serde_json::to_writer_pretty(&mut file, &default).unwrap();
            return default
        }else {
            serde_json::from_str(&content).unwrap()
        }
    }
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            window_size: None,
            window_position: None,
            window_fullscreen: false,
            window_decorations: true,
            window_maximized: true,
            vsync: true,
            fov: 90.,
            near: 10.,
            far: 1000.
        }
    }
}