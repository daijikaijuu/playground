use anyhow::Result;

pub fn _save_vec_as_image(
    buf: &Vec<f32>,
    width: usize,
    height: usize,
    filename: &str,
) -> Result<()> {
    let mut image_buf = image::ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let i = x as usize + y as usize * width;
        let value = (buf[i] * 100.0) as u8;
        *pixel = image::Luma([value]);
    }

    image_buf.save(format!("../images/tmp/{}.png", filename))?;

    Ok(())
}
