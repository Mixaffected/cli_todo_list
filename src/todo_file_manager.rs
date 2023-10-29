use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub struct TodoFileManger {
    pub path: String,
}

impl TodoFileManger {
    fn is_file_existent(&self) -> bool {
        return Path::exists(Path::new(&self.path));
    }

    pub fn create_file(&self) -> io::Result<u8> {
        let result = File::create(&self.path);
        match result {
            Ok(_) => return io::Result::Ok(0),
            Err(e) => return io::Result::Err(e),
        }
    }

    pub fn read_file(&self) -> io::Result<String> {
        if !self.is_file_existent() {
            let result = self.create_file();
            match result {
                Ok(_) => (),
                Err(e) => return io::Result::Err(e),
            }
        }

        let file = File::open(&self.path);
        let mut file = match file {
            Ok(file) => file,
            Err(e) => return io::Result::Err(e),
        };

        let mut content_buf = String::new();
        let result = file.read_to_string(&mut content_buf);
        match result {
            Ok(_) => return io::Result::Ok(content_buf),
            Err(e) => return io::Result::Err(e),
        }
    }

    pub fn write_file(&self, write_buf: String) -> io::Result<u8> {
        if !self.is_file_existent() {
            let result = self.create_file();
            match result {
                Ok(_) => (),
                Err(e) => return io::Result::Err(e),
            }
        }

        let file = File::create(&self.path);
        let mut file = match file {
            Ok(file) => file,
            Err(e) => return io::Result::Err(e),
        };

        let result = file.write_all(write_buf.as_bytes());
        match result {
            Ok(_) => return io::Result::Ok(0),
            Err(e) => return io::Result::Err(e),
        }
    }
}
