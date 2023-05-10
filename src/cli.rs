use std::io::{self, Write, Read};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "alphafix")]
pub struct Cli {
    /// paths of images to modify (supports: png)
    #[arg(required = true, num_args = 1..)]
    pub paths: Vec<String>,

    /// append to the output filenames for all images (by default _fixed is appeneded)
    #[arg(long = "append")]
    pub append: Option<String>,

    /// set source directory
    #[arg(long = "dir")]
    pub dir: Option<String>,
  
    /// skip prompts
    #[arg(long = "auto")]
    pub auto: bool,

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
                Err(err) => {
                    eprintln!("Could not open image, path: {}, error: {}", open_path.display(), err.to_string());
                    continue;
                },
            };
            println!("Opened image, path: {}", open_path.display());
            // convert to rgba8
            let img_rgba8 = match img_dynamic.as_mut_rgba8() {
                Some(img) => img,
                None => {
                    eprintln!("Could not process image, path: {}", open_path.display());
                    continue;
                },
            };
            // proccess and save
            let save_path = self.get_save_path(&open_path);
            println!("Fixing image, path: {}", save_path.display());
            match alpha_fix::fix_alpha(img_rgba8, self.opaque) {
                Err(err) => eprintln!("Error fixing image: {}", err.to_string()),
                _ => (),
            }
            match img_dynamic.save(&save_path) {
                Ok(_) => println!("Saved image, path: {}", save_path.display()),
                Err(_) => eprintln!("Error saving image: {}", save_path.display()),
            };
        }
        println!("Finished.");
        if !self.auto {
            Self::term_pause();
        }
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
        new_path.push(if let Some(dir_str) = &self.dir {
            dir_str.as_ref()
        }
        else {
            file_parent
        });
        file_stem.push(if let Some(app_str) = &self.append {
            app_str
        } else {
            "_fixed"
        });

        if self.opaque {
            file_stem.push("_opaque");
        }

        new_path.push(file_stem);
        new_path.set_extension(file_ext);
        new_path
    }

    fn term_pause() {
        print!("Press enter to continue...");
        io::stdout().flush().unwrap();
        io::stdin().read(&mut [0u8, 0]).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_many_0() {
        let input = ["exe", "test/pngs/gummy_worm_idk.png", "test/pngs/navball_velocity_prograde.png", "--dir=test/dest"];
        let args: Cli = Cli::parse_from(input);
        args.run();
    }

    #[test]
    fn cli_many_1() {
        let input = ["exe", "test/pngs/gummy_worm_idk.png", "test/pngs/navball_velocity_prograde.png", "--dir=test/dest", "--opaque"];
        let args: Cli = Cli::parse_from(input);
        args.run();
    }

    #[test]
    fn cli_many_2() {
        let input = ["exe", "test/pngs/gummy_worm_idk.png", "test/pngs/navball_velocity_prograde.png", "--dir=test/dest", "--append=_fixed"];
        let args: Cli = Cli::parse_from(input);
        args.run();
    }

    // test alpha fix with opaque flag 
    #[test]
    fn cli_opaque_0() {
        let input = ["exe", "test/pngs/gummy_worm_idk.png", "--opaque"];
        let args: Cli = Cli::parse_from(input);
        args.run();
    }

    // test alpha fix with opaque flag and appendeded arg
    #[test]
    fn cli_opaque_1() {
        let input = ["exe", "test/pngs/gummy_worm_idk.png", "--opaque", "--dir=test/dest"];
        let args: Cli = Cli::parse_from(input);
        args.run();
    }

    #[test]
    fn cli_get_save_path_0() {
        let cli: Cli = Cli {
            paths: Vec::new(),
            append: Some("_fix".to_string()),
            dir: Some("D:/idk/".to_string()),
            opaque: false,
            verbose: false,
            auto: false,
        };

        let in_path = PathBuf::from("homework/hentai/blurry.png");
        let out_path = cli.get_save_path(&in_path);
        let expect_path = PathBuf::from("D:/idk/blurry_fix.png");
        assert_eq!(out_path, expect_path);
    }

    #[test]
    fn cli_get_save_path_1() {
        let cli: Cli = Cli {
            paths: Vec::new(),
            append: Some("_fix".to_string()),
            dir: Some("D:/idk/".to_string()),
            opaque: true,
            verbose: false,
            auto: false,
        };

        let in_path = PathBuf::from("homework/hentai/blurry.png");
        let out_path = cli.get_save_path(&in_path);
        let expect_path = PathBuf::from("D:/idk/blurry_fix_opaque.png");
        assert_eq!(out_path, expect_path);
    }
}