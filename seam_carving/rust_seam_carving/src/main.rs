mod image_utils;
mod vector_utils;

use anyhow::Result;
use image::{io::Reader, DynamicImage};

use crate::{
    image_utils::save_vec_as_image,
    vector_utils::{MinInSection, RemoveMultiple},
};

const GX: [[f32; 3]; 3] = [[1.0, 0.0, -1.0], [2.0, 0.0, -2.0], [1.0, 0.0, -1.0]];
const GY: [[f32; 3]; 3] = [[1.0, 2.0, 1.0], [0.0, 0.0, 0.0], [-1.0, -2.0, -1.0]];

fn generate_light_map(img: &DynamicImage) -> Vec<f32> {
    img.to_rgb8()
        .enumerate_pixels()
        .map(|(_, _, p)| {
            let (r, g, b) = (p[0], p[1], p[2]);
            (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) / 100.0
        })
        .collect::<Vec<_>>()
}

fn generate_energy_map(light_map: &Vec<f32>, width: usize, height: usize) -> Vec<f32> {
    let mut energy_map = light_map.clone();
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let p = x + y * width;
            let a = [
                [
                    light_map[(x - 1) + width * (y - 1)],
                    light_map[x + width * (y - 1)],
                    light_map[(x + 1) + width * (y - 1)],
                ],
                [
                    light_map[(x - 1) + width * y],
                    light_map[x + width * y],
                    light_map[(x + 1) + width * y],
                ],
                [
                    light_map[(x - 1) + width * (y + 1)],
                    light_map[x + width * (y + 1)],
                    light_map[(x + 1) + width * (y + 1)],
                ],
            ];

            let mut cx = 0.0;
            let mut cy = 0.0;
            for i in 0..3 {
                for j in 0..3 {
                    cx += GX[2 - i][2 - j] * a[i][j];
                    cy += GY[2 - i][2 - j] * a[i][j];
                }
            }

            let g = (cx * cx + cy * cy).sqrt();
            energy_map[p] = g;
        }
    }

    energy_map
}

fn find_seam_vertical(width: usize, height: usize, energy_map: &Vec<f32>) -> Vec<usize> {
    let mut seam_idxs: Vec<usize> = Vec::new();

    let (mut minx_idx, _) = energy_map.min(0, width).unwrap();
    seam_idxs.push(minx_idx);
    for y in 1..height {
        let i = minx_idx + y * width;
        let m = energy_map.min(i - 1, i + 1).unwrap();
        minx_idx = m.0;
        seam_idxs.push(m.0);
        minx_idx %= width;
    }

    seam_idxs
}

fn remove_seam_vertical_from_map(img_map: &mut Vec<f32>, seam: &Vec<usize>) {
    img_map.remove_multiple(seam.to_vec());
}

fn remove_seam_vertical_from_imgbuf(img: &mut Vec<u8>, seam: &Vec<usize>) {
    img.remove_multiple_pixels(seam.to_vec());
}

// TODO: Rewrite for generating multiple seams in parallel and chosing one with less energy
fn main() -> Result<()> {
    let image = Reader::open("../images/Broadway_tower_edit.jpg")?.decode()?;
    let mut width = image.width() as usize;
    let height = image.height() as usize;
    let lightmap = generate_light_map(&image);
    let mut energy_map = generate_energy_map(&lightmap, width, height);
    let mut image_vec = image.to_rgb8().to_vec();

    println!(
        "Picture mode: {:?}, {} bytes, {} pixels",
        image.color(),
        image_vec.len(),
        image_vec.len() / 3,
    );
    println!(
        "Original image dimensions: {}x{}",
        image.width(),
        image.height()
    );

    for i in 1..200 {
        let seam = find_seam_vertical(width, height, &energy_map);
        save_vec_as_image(
            &energy_map,
            width,
            height,
            &format!("rust_energy_map_{}", i),
        )?;
        remove_seam_vertical_from_imgbuf(&mut image_vec, &seam);
        remove_seam_vertical_from_map(&mut energy_map, &seam);
        let image_buf =
            image::ImageBuffer::from_vec(width as u32 - 1, height as u32, image_vec.to_vec())
                .unwrap();
        let result = image::DynamicImage::ImageRgb8(image_buf);
        result.save(format!("../images/tmp/rust_result_{}.png", i))?;
        println!("Iteration: {} completed", i);
        width -= 1;
    }
    Ok(())
}
