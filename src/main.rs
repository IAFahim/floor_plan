use image::{DynamicImage, GenericImageView, Rgba};

#[allow(unused_variables)]
fn main() {
    let img = image::open("img/simple_floor_plan_1.png").unwrap();
    let upset: i32 = 20;
    let (width, height) = img.dimensions();
    let p_to_remove = pixel_to_remove(&img);
    println!("Pixel to remove: {:?}", p_to_remove);
    let x_wall: Vec<((u32, u32), (u32, u32))> = look_for_pixel_from_side_x(&img, p_to_remove, width, height, upset);
    println!("x_wall: {:?}", x_wall);
}

/// Look for pixel from side x if it's kinda same the pixel to remove
fn look_for_pixel_from_side_x(img: &DynamicImage, p_to_remove: Rgba<u8>, width: u32, height: u32, upset: i32) -> Vec<((u32, u32), (u32, u32))> {
    let mut x_wall: Vec<((u32, u32), (u32, u32))> = Vec::new();
    let mut points: ((u32, u32), (u32, u32)) = ((0, 0), (0, 0));
    for y in 0..height {
        let mut found: (i32, i32) = (-1, -1);
        for x in 0..width {
            let p = img.get_pixel(x, y);
            let different = changed_found_on_colors_with_upset(p, p_to_remove, upset);
            if different > 0 {
                points.0 = (x, y);
                println!("{},{} {:?} {}", x, y, p, different);
                found.0 = x as i32;
                break;
            }
        }
        if found.0 > -1 {
            let mut x = width - 1;
            while x > found.0 as u32 {
                let p = img.get_pixel(x as u32, y);
                let different = changed_found_on_colors_with_upset(p, p_to_remove, upset);
                if different > 0 {
                    points.1 = (x , y);
                    x_wall.push(points);
                    println!("{},{} {:?} {}", x, y, p, different);
                    break;
                }
                x -= 1;
            }
        }
    }
    return x_wall;
}

fn pixel_to_remove(img: &DynamicImage) -> Rgba<u8> {
    return img.get_pixel(0, 0);
}

fn changed_found_on_colors_with_upset(current_pixel: Rgba<u8>, pixel_to_remove: Rgba<u8>, upset: i32) -> i32 {
    let mut changed_found_on_colors: i32 = 0;
    for i in 0..3 {
        if (current_pixel.0[i] as i32 - pixel_to_remove.0[i] as i32).abs() > upset {
            changed_found_on_colors += 1;
        }
    }
    return changed_found_on_colors;
}