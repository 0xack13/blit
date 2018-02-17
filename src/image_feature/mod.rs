use super::*;
use image::*;

const RGBA_ALPHA_TRESHOLD: u8 = 127;

impl BlitExt for RgbImage {
    fn to_blit_buffer(&self, mask_color: Color) -> BlitBuffer {
        let (width, height) = self.dimensions();

        let pixels = (width * height) as usize;
        let mut color: Vec<Color> = vec![Color::from_u32(0); pixels];
        let mut mask: Vec<Color> = vec![Color::from_u32(0); pixels];

        let mut index = 0;
        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_pixel(x, y).data;

                // Convert pixel to Color
                let raw = Color::from_u8(pixel[0], pixel[1], pixel[2]);

                if raw == mask_color {
                    mask[index] = Color::from_u32(0xFFFFFF);
                } else {
                    color[index] = raw;
                }

                index += 1;
            }
        }

        BlitBuffer { 
            width: width as i32,
            height: height as i32,
            color,
            mask
        }
    }

    fn blit(&self, dst: &mut [u32], dst_width: usize, offset: (i32, i32), mask_color: Color) {
        let dst_size = (dst_width as i32, (dst.len() / dst_width) as i32);

        let (width, height) = self.dimensions();

        // Make sure only the pixels get rendered that are inside the dst
        let min_x = cmp::max(-offset.0, 0);
        let min_y = cmp::max(-offset.1, 0);

        let max_x = cmp::min(dst_size.0 - offset.0, width as i32);
        let max_y = cmp::min(dst_size.1 - offset.1, height as i32);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let pixel = self.get_pixel(x as u32, y as u32).data;

                // Convert pixel to Color
                let raw = Color::from_u8(pixel[0], pixel[1], pixel[2]);

                // Check if the pixel isn't the mask
                if raw != mask_color {
                    // Apply the offsets
                    let dst_x = (x + offset.0) as usize;
                    let dst_y = (y + offset.1) as usize;

                    // Calculate the index
                    let index = dst_x + dst_y * dst_size.0 as usize;
                    dst[index] = raw.u32();
                }
            }
        }
    }
}

impl BlitExt for RgbaImage {
    fn to_blit_buffer(&self, mask_color: Color) -> BlitBuffer {
        let (width, height) = self.dimensions();

        let pixels = (width * height) as usize;
        let mut color: Vec<Color> = vec![Color::from_u32(0); pixels];
        let mut mask: Vec<Color> = vec![Color::from_u32(0); pixels];

        let mut index = 0;
        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_pixel(x, y).data;

                // Convert pixel to Color
                let raw = Color::from_u8(pixel[0], pixel[1], pixel[2]);

                if raw == mask_color || pixel[3] < RGBA_ALPHA_TRESHOLD {
                    mask[index] = Color::from_u32(0xFFFFFF);
                } else {
                    color[index] = raw;
                }

                index += 1;
            }
        }

        BlitBuffer { 
            width: width as i32,
            height: height as i32,
            color,
            mask
        }
    }

    fn blit(&self, dst: &mut [u32], dst_width: usize, offset: (i32, i32), mask_color: Color) {
        let dst_size = (dst_width as i32, (dst.len() / dst_width) as i32);

        let (width, height) = self.dimensions();

        // Make sure only the pixels get rendered that are inside the dst
        let min_x = cmp::max(-offset.0, 0);
        let min_y = cmp::max(-offset.1, 0);

        let max_x = cmp::min(dst_size.0 - offset.0, width as i32);
        let max_y = cmp::min(dst_size.1 - offset.1, height as i32);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let pixel = self.get_pixel(x as u32, y as u32).data;

                // Convert pixel to Color
                let raw = Color::from_u8(pixel[0], pixel[1], pixel[2]);

                // Check if the pixel isn't the mask
                if raw != mask_color && pixel[3] >= RGBA_ALPHA_TRESHOLD {
                    // Apply the offsets
                    let dst_x = (x + offset.0) as usize;
                    let dst_y = (y + offset.1) as usize;

                    // Calculate the index
                    let index = dst_x + dst_y * dst_size.0 as usize;
                    dst[index] = raw.u32();
                }
            }
        }
    }
}