use std::mem::transmute;
use std::thread::current;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub struct Area {
    path: String,
    img: DynamicImage,
    width: usize,
    height: usize,
    tolerance: i32,
    maybe_font_height: u16,
    maybe_font_width: u16,
    color_to_ignore: [u8; 4],
    pub y_pre_sum_matrix: Vec<Vec<u16>>,
    pub dir: [(i32, i32); 8],
}

impl Area {
    pub fn new(path: &str, tolerance: i32) -> Area {
        let img = image::open(path).expect("Cant open the image");
        let width = img.width() as usize;
        let height = img.height() as usize;
        Area {
            path: path.to_string(),
            img,
            width,
            height,
            tolerance,
            maybe_font_height: 12,
            maybe_font_width: 3,
            color_to_ignore: [250, 250, 250, 250],
            y_pre_sum_matrix: vec![vec![0; height as usize]; width as usize],
            dir: [(1, 0), (1, -1), (0, 1), (-1, 1), (-1, 0), (0, -1), (-1, -1), (1, 1)],
        }
    }

    pub fn pixel_is_similar_with_tolerance(&self, current_pixel: [u8; 4], pixel_to_compare: [u8; 4]) -> bool {
        let mut changed_found_on_colors: i8 = 0;
        for i in 0..3 {
            if (current_pixel[i] as i32 - pixel_to_compare[i] as i32).abs() > self.tolerance {
                changed_found_on_colors += 1;
            }
        }
        changed_found_on_colors == 0
    }

    pub fn y_matrix_pre_sum_from_bottom_up(&mut self) {
        for x in 0..self.width {
            let mut pre = self.img.get_pixel(x as u32, self.height as u32 - 1).0;
            let mut y = self.height - 2;
            while y > 0 {
                let current_pixel = self.img.get_pixel(x as u32, y as u32).0;
                if self.pixel_is_similar_with_tolerance(current_pixel, pre) {
                    self.y_pre_sum_matrix[x][y] = self.y_pre_sum_matrix[x][y + 1] + 1;
                }
                pre = current_pixel;
                y -= 1;
            }
        }
    }

    pub fn create_color_sets(&mut self) {
        //TODO: create color sets
    }

    pub fn separate_by_color(&mut self, color: [u8; 4]) {
        let path = self.path.clone().replace(".png", "_separated_by_color.png");
        let mut img: DynamicImage = DynamicImage::new_rgb8(self.width as u32, self.height as u32);
        for x in 0..self.width {
            for y in 0..self.height {
                let current_pixel = self.img.get_pixel(x as u32, y as u32);
                if self.pixel_is_similar_with_tolerance(current_pixel.0, color) {
                    img.put_pixel(x as u32, y as u32, current_pixel);
                }
            }
        }
        img.save(path).expect("Cant save the image");
    }


    pub fn y_heat_map(&mut self) {
        let path = self.path.clone().replace(".png", "_y_heatMap.png");
        let mut img = DynamicImage::new_rgb8(self.width as u32, self.height as u32);
        let height = self.height as u16;
        for x in 0..self.width {
            for y in 0..self.height {
                let mut color = ((self.y_pre_sum_matrix[x][y] as usize * 255) / self.height) as u8;
                img.put_pixel(x as u32, y as u32, Rgba([255, 255 - color, 255, 255]));
            }
        }
        img.save(path).expect("Cant save the image");
    }

    pub fn mean_pixel_is_smaller_than_tolerance_if_last_was_mismatch(&self, first: [u8; 4], mid: [u8; 4], last: [u8; 4]) -> bool {
        let mut changed_found_on_colors: i8 = 0;
        for i in 0..3 {
            if (((first[i] as i32 + last[i] as i32) / 2) - (mid[i] as i32)).abs() > self.tolerance {
                changed_found_on_colors += 1;
            }
        }
        changed_found_on_colors == 0
    }

    pub fn dominant_outer_color(&self) -> [u8; 4] {
        self.img.get_pixel(0, 0).0
        // TODO : implement the better way of finding the dominate pixel
    }
}

