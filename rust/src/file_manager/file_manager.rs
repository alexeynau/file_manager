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

    pub fn read_file(path: &str) -> Result<File, io::Error> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let metadata = file.metadata()?;
        let file = File {
            name: path.to_string(),
            content,
            size: metadata.len(),
            is_dir: false,
        };
        Ok(file)
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

    pub fn create_dir(path: &str) -> Result<(), io::Error> {
        fs::create_dir(path)?;
        Ok(())
    }

    pub fn delete_dir(path: &str) -> Result<(), io::Error> {
        if Path::new(path).exists() {
            fs::remove_dir_all(path)?;
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found"));
        }
        Ok(())
    }


    pub fn list_all(path: &str) -> Result<Vec<File>, io::Error> {
        let mut all = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let file = File {
                name: path.display().to_string()[2..].to_string(),
                content: String::new(),
                size: metadata.len(),
                is_dir: path.is_dir(),
            };
            all.push(file);
        }
        Ok(all)
    }
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: String,
    pub size: u64,
    pub is_dir: bool,
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_FILE: &str = "test.txt";
    const TEST_DIR: &str = "test_dir";
    const TEST_CONTENT: &str = "Hello, Rust!";
    const UPDATED_CONTENT: &str = "Hello, updated Rust!";


    #[test]
    fn test_create_dir() {
        FileManager::create_dir(TEST_DIR).expect("Failed to create directory");
        assert!(Path::new(TEST_DIR).exists());
        fs::remove_dir(TEST_DIR).unwrap(); // Удаляем директорию после теста
    }

    #[test]
    fn test_list_all() {
        FileManager::create_file(TEST_FILE).expect("Failed to create file");
        let files = FileManager::list_all(".").expect("Failed to list files");
        print!("{:?}", files);
        assert!(files.len() > 0);
        assert!(files.iter().any(|file| file.name == TEST_FILE));
        fs::remove_file(TEST_FILE).unwrap(); // Удаляем файл после теста
    }

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
        assert_eq!(content.content, TEST_CONTENT);
        fs::remove_file(TEST_FILE).unwrap(); // Удаляем файл после теста
    }

    #[test]
    fn test_update_file() {
        FileManager::create_file_with_content(TEST_FILE, TEST_CONTENT).expect("Failed to create file");
        FileManager::update_file(TEST_FILE, UPDATED_CONTENT).expect("Failed to update file");
        let content = FileManager::read_file(TEST_FILE).expect("Failed to read file");
        assert_eq!(content.content, UPDATED_CONTENT);
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
