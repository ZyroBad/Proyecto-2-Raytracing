use crate::math::Color;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn save_ppm(path: &str, width: usize, height: usize, pixels: &[Color]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", width, height)?;
    writeln!(writer, "255")?;

    for y in 0..height {
        for x in 0..width {
            let color = pixels[y * width + x];
            write!(
                writer,
                "{} {} {} ",
                (color.x * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.z * 255.0) as u8
            )?;
        }
        writeln!(writer)?;
    }
    Ok(())
}

pub fn save_bmp(path: &str, width: usize, height: usize, pixels: &[Color]) -> std::io::Result<()> {
    let row_stride = (width * 3 + 3) & !3;
    let pixel_data_size = row_stride * height;
    let file_size = 54 + pixel_data_size;
    let mut file = BufWriter::new(File::create(path)?);

    file.write_all(b"BM")?;
    file.write_all(&(file_size as u32).to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&(54u32).to_le_bytes())?;
    file.write_all(&(40u32).to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(height as i32).to_le_bytes())?;
    file.write_all(&(1u16).to_le_bytes())?;
    file.write_all(&(24u16).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;
    file.write_all(&(pixel_data_size as u32).to_le_bytes())?;
    file.write_all(&(2835i32).to_le_bytes())?;
    file.write_all(&(2835i32).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;

    let padding = vec![0u8; row_stride - width * 3];
    for y in (0..height).rev() {
        for x in 0..width {
            let color = pixels[y * width + x];
            file.write_all(&[
                (color.z * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.x * 255.0) as u8,
            ])?;
        }
        file.write_all(&padding)?;
    }
    Ok(())
}
