use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "alphafix")]
pub struct Cli {
    /// paths of images to modify (supports: png)
    #[arg(required = true, num_args = 1..)]
    pub paths: Vec<String>,

    /// append to the output filenames for all images (avoid overriding)
    #[arg(long = "append")]
    pub append: Option<String>,

    /// set source directory
    #[arg(long = "dir")]
    pub dir: Option<String>,
  
    /// (DEBUG)
    #[arg(long = "verbose")]
    pub verbose: bool,

    /// (DEBUG) set transparent pixels to opaque after bleeding 
    #[arg(long)]
    pub opaque: bool,
}

impl Cli {
    pub fn run(&self) {
        for path in self.paths.iter() {
            // open from file
            let open_path = self.get_open_path(path);
            let mut img_dynamic = match alpha_fix::open_image_file(&open_path) {
                Ok(img) => img,
                Err(_) => {
                    log::warn!("Can't open image, path: {}", open_path.display());
                    continue;
                },
            };
            println!("Opened image, path: {}", open_path.display());
            // convert to rgba8
            let img_rgba8 = match img_dynamic.as_mut_rgba8() {
                Some(img) => img,
                None => {
                    log::warn!("Can't process image, path: {}", open_path.display());
                    continue;
                },
            };
            // proccess and save
            let save_path = self.get_save_path(&open_path);
            println!("Fixing image, path: {}", save_path.display());
            match alpha_fix::fix_alpha(img_rgba8, self.opaque) {
                Err(err) => println!("Error processing image: {}", err.to_string()),
                _ => (),
            }
            match img_dynamic.save(&save_path) {
                Ok(_) => println!("Saved image, path: {}", save_path.display()),
                Err(_) => println!("ERROR: Can't save image, path: {}", save_path.display()),
            };
        }
        println!("Finished.");
    }

    fn get_open_path(&self, path_str: &str) -> PathBuf {
        PathBuf::from(path_str)
    }

    fn get_save_path(&self, path: &PathBuf) -> PathBuf {
        if !(self.dir.is_some() || self.append.is_some() || self.opaque) {
            return path.clone();
        }
        let file_parent: &Path = path.parent().unwrap();
        let mut file_stem: OsString = path.file_stem().unwrap().to_os_string();
        let file_ext: &OsStr = path.extension().unwrap();

        let mut new_path = PathBuf::new();
        if let Some(dir_str) = &self.dir {
            new_path.push(dir_str);
        }
        else {
            new_path.push(file_parent);
        }

        if let Some(app_str) = &self.append {
            file_stem.push(app_str);
        }
        if self.opaque {
            file_stem.push("_opaque");
        }

        new_path.push(file_stem);
        new_path.set_extension(file_ext);
        new_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_save_path_0() {
        let cli: Cli = Cli {
            paths: Vec::new(),
            append: Some("_fix".to_string()),
            dir: Some("D:/idk/".to_string()),
            opaque: false,
            verbose: false,
        };

        let in_path = PathBuf::from("homework/hentai/blurry.png");
        let out_path = cli.get_save_path(&in_path);
        let expect_path = PathBuf::from("D:/idk/blurry_fix.png");
        assert_eq!(out_path, expect_path);
    }

    #[test]
    fn test_get_save_path_1() {
        let cli: Cli = Cli {
            paths: Vec::new(),
            append: Some("_fix".to_string()),
            dir: Some("D:/idk/".to_string()),
            opaque: true,
            verbose: false,
        };

        let in_path = PathBuf::from("homework/hentai/blurry.png");
        let out_path = cli.get_save_path(&in_path);
        let expect_path = PathBuf::from("D:/idk/blurry_fix_opaque.png");
        assert_eq!(out_path, expect_path);
    }
}