use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub struct FileManager;

impl FileManager {

    pub fn create_file(path: &str) -> Result<(), io::Error> {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        Ok(())
    }

    pub fn create_file_with_content(path: &str, content: &str) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn read_file(path: &str) -> Result<String, io::Error> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn update_file(path: &str, content: &str) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn delete_file(path: &str) -> Result<(), io::Error> {
        if Path::new(path).exists() {
            fs::remove_file(path)?;
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_FILE: &str = "test.txt";
    const TEST_CONTENT: &str = "Hello, Rust!";
    const UPDATED_CONTENT: &str = "Hello, updated Rust!";

    #[test]
    fn test_create_file() {
        FileManager::create_file(TEST_FILE).expect("Failed to create file");
        assert!(Path::new(TEST_FILE).exists());
        fs::remove_file(TEST_FILE).unwrap(); // Удаляем файл после теста
    }

    #[test]
    fn test_create_file_with_content() {
        FileManager::create_file_with_content(TEST_FILE, TEST_CONTENT).expect("Failed to create file");
        assert!(Path::new(TEST_FILE).exists());
        fs::remove_file(TEST_FILE).unwrap(); // Удаляем файл после теста
    }

    #[test]
    fn test_read_file() {
        FileManager::create_file_with_content(TEST_FILE, TEST_CONTENT).expect("Failed to create file");
        let content = FileManager::read_file(TEST_FILE).expect("Failed to read file");
        assert_eq!(content, TEST_CONTENT);
        fs::remove_file(TEST_FILE).unwrap(); // Удаляем файл после теста
    }

    #[test]
    fn test_update_file() {
        FileManager::create_file_with_content(TEST_FILE, TEST_CONTENT).expect("Failed to create file");
        FileManager::update_file(TEST_FILE, UPDATED_CONTENT).expect("Failed to update file");
        let content = FileManager::read_file(TEST_FILE).expect("Failed to read file");
        assert_eq!(content, UPDATED_CONTENT);
        fs::remove_file(TEST_FILE).unwrap(); // Удаляем файл после теста
    }

    #[test]
    fn test_delete_file() {
        FileManager::create_file_with_content(TEST_FILE, TEST_CONTENT).expect("Failed to create file");
        FileManager::delete_file(TEST_FILE).expect("Failed to delete file");
        assert!(!Path::new(TEST_FILE).exists());
    }

    #[test]
    fn test_delete_nonexistent_file() {
        let result = FileManager::delete_file("nonexistent.txt");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            io::ErrorKind::NotFound
        );
    }
}
