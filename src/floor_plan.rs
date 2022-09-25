use std::ptr::null;
use image::{DynamicImage, GenericImageView, Rgba};

pub struct Area {
    path: String,
    img: DynamicImage,
    width: u32,
    height: u32,
    tolerance: i32,
    y_pre_sum: Vec<Vec<u16>>,
}

impl Area {
    pub fn new(path: &str, tolerance: i32) -> Area {
        let img = image::open(path).expect("Cant open the image");
        let (width, height) = img.dimensions();
        Area {
            path: path.to_string(),
            img,
            width,
            height,
            tolerance,
            y_pre_sum: vec![vec![0; height as usize]; width as usize],
        }
    }

    // pub fn look_pixel_from_x_axis(&self, x: u32, y: u32) -> Vec<(u32, (u32, u32))> {
    //     let mut x_wall: Vec<(u32, (u32, u32))> = Vec::with_capacity((self.width / 2) as usize);
    //     let mut points: (u32, (u32, u32)) = (0, (0, 0));
    //     for y in 0..height {
    //         let mut found_at: (u32, u32) = (u32::MAX, u32::MAX);
    //         let mut count = 0;
    //         for x in 0..width {
    //             if has_n_bellow_with_tolerance(img, self.dominate_pixel, width, height, x, y, 5, upset) {
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
    //                 if has_n_bellow_with_tolerance(img, self.dominate_pixel, width, height, x, y, 5, upset) {
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
    //     x_wall
    // }

    fn y_axis_pre_sum(&self) {}

    fn pixel_is_similar_with_tolerance_and_get_pre(&self, x: u32, y: u32) -> (bool, Rgba<u8>) {
        let current_pixel = self.img.get_pixel(x, y);
        let previous_pixel = self.img.get_pixel(x - 1, y - 1);
        let mut changed_found_on_colors: i32 = 0;
        for i in 0..3 {
            if (current_pixel.0[i] as i32 - previous_pixel.0[i] as i32).abs() > self.tolerance {
                changed_found_on_colors += 1;
            }
        }
        (changed_found_on_colors != 0, current_pixel)
    }

    pub fn pixel_to_remove(&self) -> Rgba<u8> {
        self.img.get_pixel(0, 0)
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