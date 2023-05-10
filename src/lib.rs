use std::error::Error;

use image::Rgba;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum BleedStage {
    Unprocessed,
    Staged,
    Processed,
}

const NEIGHBORS: [(i8, i8); 8] = [
    ( 1,  0),
    ( 1, -1),
    ( 0, -1),
    (-1, -1),
    (-1,  0),
    (-1,  1),
    ( 0,  1),
    ( 1,  1),
];

fn neighbors(x: i32, y: i32, w: i32, h: i32) -> impl Iterator<Item = (i32, i32)> {
    return NEIGHBORS.iter()
        .filter(move |(u, v)| {
            let x1 = x + *u as i32;
            let y1 = y + *v as i32;
            x1 > 0 && y1 > 0 && x1 < w && y1 < h
        })
        .map(move |(u, v)| {
            (x + *u as i32, y + *v as i32)
        });
}

// set the alpha of all pixels in the image to alpha
pub fn set_alpha(img: &mut image::RgbaImage, a: u8) {
    for pixel in img.pixels_mut() {
        let Rgba([r, g, b, _]) = *pixel;
        *pixel = Rgba([r, g, b, a]);
    }
}

// propogate the color of opaque pixels to transparent pixels
// meant to mitigate issues with image sampling of scaled images (Roblox image handling)
// inspired by: https://github.com/urraka/alpha-bleeding
pub fn fix_alpha(img: &mut image::RgbaImage, set_opaque: bool) -> Result<(), Box<dyn Error>> {
    let alpha = if set_opaque {255} else {0};
    let (width, height) = (img.width() as i32, img.height() as i32);
    // make a transparent double queue
    let mut queue0: Vec<(i32, i32)> = Vec::new();
    let mut queue1: Vec<(i32, i32)> = Vec::new();
    let mut stages: Vec<BleedStage> = vec![BleedStage::Unprocessed; (width * height) as usize];
    // mark non-transparent pixels as processed
    for y in 0..width {
        for x in 0..height {
            let index = (y * width + x) as usize;
            let Rgba([_, _, _, a]) = img[(x as u32, y as u32)];
            if a > 0 {
                stages[index] = BleedStage::Processed;
            } 
            else {
                stages[index] = BleedStage::Unprocessed;
            }
        }
    }
    // stage transparent pixels that are neighbors of processed pixels
    for y in 0..width {
        for x in 0..height {
            let index = (y * width + x) as usize;            
            let stage0 = stages[index];
            if stage0 != BleedStage::Processed {
                continue;
            }
            // get neighbors:
            for (x1, y1) in neighbors(x, y, width, height) {
                let index1 = (y1 * width + x1) as usize;
                if stages[index1] == BleedStage::Unprocessed {
                    queue0.push((x1, y1));
                    stages[index1] = BleedStage::Staged;
                    break;
                }
            }
        }
    }

    
    // until first queue is empty
    while !queue0.is_empty() {
        // set queue pixel color to sum of neighboring processed pixels
        for (x, y) in queue0.iter() {
            let (x, y) = (*x, *y);
            let mut c: u32 = 0;
            let mut r: u32 = 0; let mut g: u32 = 0; let mut b: u32 = 0;
            // sum colors of proccessed neighbors
            for (x1, y1) in neighbors(x, y, width, height) {
                let index1 = (y1 * width + x1) as usize;
                let stage = stages[index1];
                if stage == BleedStage::Processed {
                    let pixel = img.get_pixel(x1 as u32, y1 as u32);
                    let Rgba([r1, g1, b1, _]) = *pixel;
                    c += 1;
                    r += r1 as u32;
                    g += g1 as u32;
                    b += b1 as u32;
                } 
                else if stage == BleedStage::Unprocessed {
                    stages[index1] = BleedStage::Staged;
                    queue1.push((x1, y1));
                }
            }
            if c > 0 {
                r /= c; g /= c; b /= c;
                let pixel = img.get_pixel_mut(x as u32, y as u32);
                *pixel = Rgba([r as u8, g as u8, b as u8, alpha]);
            }
            // TODO: Handle unforseen case where we don't mod a pixel in this queue (might propogate grey and be ugly)
        }
        
        // set pixels to processed
        for &(x, y) in queue0.iter() {
            let index = (y * width + x) as usize;
            stages[index] = BleedStage::Processed;
        }

        // clear and switch queue
        queue0.clear();
        std::mem::swap(&mut queue0, &mut queue1);
    }

    Ok(())
}

pub fn open_image_file(path: impl AsRef<std::path::Path>) -> Result<image::DynamicImage, Box<dyn Error>> {
    Ok(image::io::Reader::open(path)?.decode()?)
}

pub fn save_image_file(path: impl AsRef<std::path::Path>) -> Result<image::DynamicImage, Box<dyn Error>> {
    // TODO:
    unimplemented!();
}