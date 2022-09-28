use std::mem::transmute;
use std::thread::current;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub struct Area {
    path: String,
    img: DynamicImage,
    width: u16,
    height: u16,
    tolerance: i32,
    maybe_font_height: u16,
    maybe_font_width: u16,
    color_to_ignore: [u8; 4],
    pub y_boundary_bottom_up: Vec<(u16, Vec<(u16, u16)>)>,
    pub bounds: Vec<(u16, Vec<(u16, u16)>)>,
}

impl Area {
    pub fn new(path: &str, tolerance: i32) -> Area {
        let img = image::open(path).expect("Cant open the image");
        let width = img.width() as u16;
        let height = img.height() as u16;
        Area {
            path: path.to_string(),
            img,
            width,
            height,
            tolerance,
            maybe_font_height: 12,
            maybe_font_width: 3,
            color_to_ignore: [250, 250, 250, 250],
            y_boundary_bottom_up: Vec::with_capacity(width as usize),
            bounds: Vec::new(),
        }
    }

    pub fn get_walls_looking_from_x(&self) {
        let mut bounds: Vec<(u32, Vec<(u32, u32)>)> = Vec::new();
        for y in 0..self.height {
            let mut sum = 0;
            for x in 1..self.width {
                let current_pixel = self.img.get_pixel(x as u32, y as u32).0;
                if self.pixel_is_similar_with_tolerance(current_pixel, self.color_to_ignore) {
                    continue;
                }
                let previous_pixel = self.img.get_pixel((x - 1) as u32, y as u32).0;
                if self.pixel_is_similar_with_tolerance(previous_pixel, self.color_to_ignore) {
                    sum += 1;
                }
            }
        }
    }

    pub fn x_bound_heat(){

    }

    pub fn y_axis_heat_map(&self) {
        let new_string_path = self.path.replace(".png", "_heatMap.png");
        let mut imgbuf = DynamicImage::new_rgb8(self.width as u32, self.height as u32);
        for x in 0..self.width {
            let v: Vec<(u16, u16)> = self.y_boundary_bottom_up.get(x as usize).unwrap().1.clone();
            for bound in v {
                let mut count: u32 = 0;
                let mut y = bound.1;
                while y <= bound.0 {
                    let g=((count * 255) / self.height as u32 ) as u8;
                    let mut buf: [u8; 4] = [255, 255-g, 255, 255];
                    imgbuf.put_pixel(x as u32, y as u32, Rgba(buf));
                    y += 1;
                    count += 1;
                }
            }
        }
        imgbuf.save(new_string_path).unwrap();
    }

    fn y_axis_contains_wall(&self, x: u16, y: u16) -> bool {
        let point: Vec<(u16, u16)> = self.y_boundary_bottom_up.get(y as usize).unwrap().1.clone();
        for i in point {
            if i.0 <= x && x <= i.1 {
                return true;
            }
        }
        false
    }


    pub fn y_axis_boundary_down_to_up(&mut self) {
        for x in 0..self.width {
            let mut sum = 0;
            let mut y = self.height - 2;
            let mut height_up: Vec<(u16, u16)> = Vec::new();

            while y > 1 {
                let current_pixel = self.img.get_pixel(x as u32, y as u32).0;
                let pixel_below = self.img.get_pixel(x as u32, (y + 1) as u32).0;
                if self.pixel_is_similar_with_tolerance(current_pixel, pixel_below) {
                    sum += 1;
                } else if sum > self.maybe_font_height {
                    height_up.push((y + sum, y));
                    sum = 0;
                }
                y -= 1;
            }
            self.y_boundary_bottom_up.push((x, height_up));
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

    pub fn mean_pixel_is_smaller_than_tolerance_if_last_was_mismatch(&self, first: [u8; 4], mid: [u8; 4], last: [u8; 4]) -> bool {
        let mut changed_found_on_colors: i8 = 0;
        for i in 0..3 {
            if (((first[i] as i32 + last[i] as i32) / 2) - (mid[i] as i32)).abs() > self.tolerance {
                changed_found_on_colors += 1;
            }
        }
        changed_found_on_colors == 0
    }

    pub fn dominent_outer_color(&self) -> [u8; 4] {
        self.img.get_pixel(0, 0).0
        // TODO : implement the better way of finding the dominate pixel
    }
}

// #[allow(unused_variables)]
// fn nc() {
//     let img = image::open("img/simple_floor_plan_1.png").unwrap();
//     let upset: i32 = 15;
//     let (width, height) = img.dimensions();
//     let p_to_remove = dominate_pixel(&img);
//     println!("Pixel to remove: {:?}", p_to_remove);
//     let x_wall: Vec<(u32, (u32, u32))> = look_for_pixel_from_side_x(&img, p_to_remove, width, height, upset);
//     println!("x_wall: {:?}", x_wall);
// }
//
//
// /// Look for pixel from side x if it's kinda same the pixel to remove
// fn look_for_pixel_from_side_x(img: &DynamicImage, pixel_to_remove: Rgba<u8>, width: u32, height: u32, upset: i32) -> Vec<(u32, (u32, u32))> {
//     let mut x_wall: Vec<(u32, (u32, u32))> = Vec::with_capacity((width / 2) as usize);
//     let mut points: (u32, (u32, u32)) = (0, (0, 0));
//     for y in 0..height {
//         let mut found_at: (u32, u32) = (u32::MAX, u32::MAX);
//         let mut count = 0;
//         for x in 0..width {
//             if has_n_bellow_with_tolerance(img, pixel_to_remove, width, height, x, y, 5, upset) {
//                 count += 1;
//                 if count > 5 {
//                     points = (y, (x - count, 0));
//                     found_at.0 = x - count;
//                     break;
//                 }
//             }
//         }
//         if found_at.0 < u32::MAX {
//             let mut count = 0;
//             let mut x = width - 1;
//             while x > found_at.0 as u32 {
//                 if has_n_bellow_with_tolerance(img, pixel_to_remove, width, height, x, y, 5, upset) {
//                     count += 1;
//                     if count > 5 {
//                         points.1.1 = x + count;
//                         x_wall.push(points);
//                         break;
//                     }
//                 }
//                 x -= 1;
//             }
//         }
//     }
//     return x_wall;
// }
//
// fn has_n_bellow_with_tolerance(img: &DynamicImage, pixel_to_remove: Rgba<u8>, width: u32, height: u32, x: u32, y: u32, n: u32, upset: i32) -> bool {
//     let pixel = img.get_pixel(x, y);
//     let different = changed_found_on_colors_with_upset(pixel, pixel_to_remove, upset);
//     if different == 0 {
//         return false;
//     }
//     let mut i = y;
//     let mut count = 0;
//     while i < height && count < n {
//         let pixel = img.get_pixel(x, i);
//         if changed_found_on_colors_with_upset(pixel, pixel_to_remove, upset) > 0 {
//             count += 1;
//         }
//         i += 1;
//     }
//     return n == count;
// }
//
// fn dominate_pixel(img: &DynamicImage) -> Rgba<u8> {
//     return img.get_pixel(0, 0);
// }
//
// fn changed_found_on_colors_with_upset(current_pixel: Rgba<u8>, pixel_to_remove: Rgba<u8>, upset: i32) -> i32 {
//     let mut changed_found_on_colors: i32 = 0;
//     for i in 0..3 {
//         if (current_pixel.0[i] as i32 - pixel_to_remove.0[i] as i32).abs() > upset {
//             changed_found_on_colors += 1;
//         }
//     }
//     return changed_found_on_colors;
// }
//
