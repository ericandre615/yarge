use image::{ImageResult, DynamicImage};

use rusttype::{Font, FontCollection};

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read};
use std::ffi;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error")]
    Io(#[cause] io::Error),
    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,
    #[fail(display = "Failed to get executable path")]
    FailedToGetExePath,
    #[fail(display = "Failed to load image")]
    FailedToLoadImage,
    #[fail(display = "Failed to load json")]
    FailedToLoadJson,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

#[derive(Debug)]
pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_path(rel_path: &Path) -> Result<Resources, Error> {
        let exe_file_name = ::std::env::current_exe()
            .map_err(|_| Error::FailedToGetExePath)?;
        let exe_path = exe_file_name.parent()
            .ok_or(Error::FailedToGetExePath)?;

        Ok(Resources {
            root_path: exe_path.join(rel_path)
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path, resource_name)
        )?;
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );

        file.read_to_end(&mut buffer)?;

        if buffer.iter().any(|i| *i == 0) {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe {
            ffi::CString::from_vec_unchecked(buffer)
        })
    }

    pub fn load_image_from_path(&self, path: &str) -> ImageResult<DynamicImage> {
        let file_path = resource_name_to_path(&self.root_path, path);

        image::open(file_path) // .ok().expect(Error::FailedToLoadImage);
    }

    pub fn load_from_json(&self, path: &str) -> Result<serde_json::Value, failure::Error> {
        let file_path = resource_name_to_path(&self.root_path, path);
        let mut file_contents = String::new();
        let mut file = fs::File::open(file_path)?;

        file.read_to_string(&mut file_contents)?;

        let json: serde_json::Value = serde_json::from_str(&file_contents)?;

        Ok(json)
    }

    pub fn load_font(&self, path: &str) -> Result<Font, failure::Error> {
        let font_path = resource_name_to_path(&self.root_path, path);
        let font_data = std::fs::read(font_path)?;
        let font = FontCollection::from_bytes(font_data).unwrap()
            .into_font().expect("Error loading Font");

        Ok(font)
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split('/') {
        path = path.join(part);
    }

    path
}
