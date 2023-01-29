use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "alphafix")]
pub struct Cli {
    // for now only support the alpha bleeding command

    #[arg(required = true, num_args = 1..)]
    pub paths: Vec<String>,

    /// Set the transparent pixels to opaque after bleeding (DEBUG)
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
            let save_path = self.get_save_path(open_path);
            println!("Fixing image, path: {}", save_path.display());
            alpha_fix::fix_alpha(img_rgba8, self.opaque);
            match img_dynamic.save(&save_path) {
                Ok(_) => println!("Saved image, path: {}", save_path.display()),
                Err(_) => println!("ERROR: Can't save image, path: {}", save_path.display()),
            };
        }
        println!("Finished.");
    }

    fn get_open_path(&self, path_str: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(path_str)
    }

    fn get_save_path(&self, path: std::path::PathBuf) -> std::path::PathBuf {
        if self.opaque {
            let mut file_name = path.file_stem().unwrap().to_os_string();
            file_name.push("_opaque");
            let mut save_path = path.to_owned();
            save_path.set_file_name(file_name);
            save_path.set_extension(path.extension().unwrap());
            save_path
        }
        else {
            path
        }
    }
}