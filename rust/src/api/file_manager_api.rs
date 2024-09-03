use crate::file_manager;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_file(path: &str) -> Result<(), std::io::Error> {
    file_manager::file_manager::FileManager::create_file(path)
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_file_with_content(path: &str, content: &str) -> Result<(), std::io::Error> {
    file_manager::file_manager::FileManager::create_file_with_content(path, content)
}

#[flutter_rust_bridge::frb(sync)]
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    file_manager::file_manager::FileManager::read_file(path)
}

#[flutter_rust_bridge::frb(sync)]
pub fn update_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    file_manager::file_manager::FileManager::update_file(path, content)
}

#[flutter_rust_bridge::frb(sync)]
pub fn delete_file(path: &str) -> Result<(), std::io::Error> {
    file_manager::file_manager::FileManager::delete_file(path)
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
