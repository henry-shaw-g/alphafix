// tbh not liking rust rn (C SUPERIORITY)
mod cli;

use cli::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    args.run();
    println!("paths: {:?}", args.paths);
    println!("--opaque: {}", args.opaque);
    // println!("opening image.");
    // let dynamic_img = ImageReader::open("pngs/test_picture_in.png").unwrap().decode().unwrap();
    // let mut img = dynamic_img.to_rgba8();
    // println!("processing image.");
    // alpha_fix::fix_alpha(&mut img);
    // alpha_fix::set_alpha(& mut img, 255);
    // img.save("pngs/test_picture_in.png").unwrap();
    // println!("finished.");
    
}
