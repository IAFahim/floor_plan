use std::usize;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub struct Area {
    path: String,
    pub img: DynamicImage,
    width: usize,
    height: usize,
    tolerance: u8,
    maybe_font_height: u16,
    maybe_font_width: u16,
    dark_color_sum: u16,
    color_to_ignore: [u8; 4],
    pub y_pre_sum_matrix: Vec<Vec<u16>>,
    pub x_pre_sum_matrix: Vec<Vec<u16>>,
    pub wall_visited: Vec<Vec<bool>>,
    pub dir: [(i32, i32); 8],
    pub prominent_dark_color: Vec<[u8; 4]>,
    pub walls: Vec<(u16, Vec<u16>)>,
    pub wall_count: u16,
    NOT_FOUND: (u16, u16),
}

#[allow(dead_code)]
impl Area {
    pub fn new(path: &str, tolerance: u8) -> Area {
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
            dark_color_sum: 300,
            y_pre_sum_matrix: vec![vec![0; height as usize]; width as usize],
            x_pre_sum_matrix: vec![vec![0; height as usize]; width as usize],
            wall_visited: vec![vec![false; height as usize]; width as usize],
            dir: [(1, 0), (1, -1), (0, 1), (-1, 1), (-1, 0), (0, -1), (-1, -1), (1, 1)],
            prominent_dark_color: Vec::new(),
            walls: Vec::new(),
            wall_count: 0,
            NOT_FOUND: (u16::MAX, u16::MAX),
        }
    }

    pub fn pixel_is_similar_with_tolerance(&self, current_pixel: [u8; 4], pixel_to_compare: [u8; 4]) -> bool {
        let mut changed_found_on_colors: i8 = 0;
        for i in 0..3 {
            let mut dif: u8 = 0;
            if current_pixel[i] > pixel_to_compare[i] {
                dif = current_pixel[i] - pixel_to_compare[i];
            } else {
                dif = pixel_to_compare[i] - current_pixel[i];
            }
            if dif > self.tolerance {
                changed_found_on_colors += 1;
            }
        }
        changed_found_on_colors == 0
    }

    pub fn create_y_matrix_pre_sum_from_bottom_up(&mut self) {
        for x in 0..self.width {
            let mut pre = self.img.get_pixel(x as u32, self.height as u32 - 1).0;
            let mut y = self.height - 2;
            while y != 0 {
                let current_pixel = self.img.get_pixel(x as u32, y as u32).0;
                if self.pixel_is_similar_with_tolerance(current_pixel, pre) {
                    self.y_pre_sum_matrix[x][y] = self.y_pre_sum_matrix[x][y + 1] + 1;
                }
                pre = current_pixel;
                y -= 1;
            }
        }
    }

    pub fn create_x_matrix_pre_sum_from_bottom_up(&mut self) {
        for y in 0..self.height {
            let mut pre = self.img.get_pixel(0, y as u32).0;
            let mut x = self.width - 2;
            while x != 0 {
                let current_pixel = self.img.get_pixel(x as u32, y as u32).0;
                if self.pixel_is_similar_with_tolerance(current_pixel, pre) {
                    self.x_pre_sum_matrix[x][y] = self.x_pre_sum_matrix[x + 1][y] + 1;
                }
                pre = current_pixel;
                x -= 1;
            }
        }
    }


    pub fn create_color_sets(&mut self) {
        //TODO: create color sets
    }

    pub fn get_wall_map(&mut self) {
        let mut one_wall: (u16, Vec<(u16, u16)>) = (0, Vec::new());
        for y in 0..self.height - 1 {
            for mut x in 0..self.width - 1 {
                let got = self.dir_x_1(x, y, &mut one_wall);
                if got != self.NOT_FOUND {
                    x = got.0 as usize;
                    // self.walls.push((self.wall_count, one_wall));
                }
            }
        }
    }

    pub fn dir_x_1(&mut self, mut x: usize, y: usize, _current_wall: &mut (u16, Vec<(u16, u16)>)) -> (u16, u16) {
        if !self.wall_visited[x][y] { return self.NOT_FOUND; }
        _current_wall.0 = 0;
        _current_wall.1.push((x as u16, y as u16));
        while x < self.width - 1 && self.wall_visited[x + 1][y] {
            x += 1;
            self.wall_visited[x][y] = false;
        }
        _current_wall.0 += 1;
        _current_wall.1.push(((x - 1) as u16, y as u16));
        self.dir_y__1((x - 1), y, _current_wall);
        (x as u16, y as u16)
    }

    pub fn dir_y__1(&mut self, x: usize, mut y: usize, _current_wall: &mut (u16, Vec<(u16, u16)>)) {
        _current_wall.0 = 0;
        _current_wall.1.push((x as u16, y as u16));
        while y < self.height - 1 && self.wall_visited[x][y + 1] {
            y += 1;
            self.wall_visited[x][y] = false;
        }
        _current_wall.0 += 1;
        _current_wall.1.push((x as u16, (y - 1) as u16));
        self.dir_x__1(x, (y - 1), _current_wall);
        self.dir_x_1(x, (y - 1), _current_wall);
    }

    pub fn dir_x__1(&mut self, mut x: usize, y: usize, _current_wall: &mut (u16, Vec<(u16, u16)>)) {
        _current_wall.0 = 0;
        _current_wall.1.push((x as u16, y as u16));
        while x != 0 && self.wall_visited[x - 1][y] {
            x -= 1;
            self.wall_visited[x][y] = false;
        }
        _current_wall.0 += 1;
        _current_wall.1.push(((x + 1) as u16, y as u16));
        self.dir_y_1((x + 1), y, _current_wall);
    }

    pub fn dir_y_1(&mut self, x: usize, mut y: usize, _current_wall: &mut (u16, Vec<(u16, u16)>)) {
        _current_wall.0 = 0;
        _current_wall.1.push((x as u16, y as u16));
        while y != 0 && self.wall_visited[x][y - 1] {
            y -= 1;
            self.wall_visited[x][y] = false;
        }
        _current_wall.0 += 1;
        _current_wall.1.push((x as u16, (y + 1) as u16));
    }


    pub fn gather_prominent_colors(&mut self) {
        let rather_black = self.tolerance as u8;
        self.prominent_dark_color.push([rather_black, rather_black, rather_black, 255]);
    }

    pub fn create_wall_matrix(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let current_pixel = self.img.get_pixel(x as u32, y as u32).0;
                if self.pixel_is_similar_with_tolerance(current_pixel, self.prominent_dark_color[0]) {
                    self.wall_visited[x][y] = true;
                }
            }
        }
    }

    pub fn get_rid_of_all_light_color(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let current_pixel_rgba = self.img.get_pixel(x as u32, y as u32);
                let current_pixel = current_pixel_rgba.0;
                let sum = current_pixel[0] as u16 + current_pixel[1] as u16 + current_pixel[2] as u16;
                if sum < self.dark_color_sum {
                    self.img.put_pixel(x as u32, y as u32, current_pixel_rgba);
                } else {
                    self.img.put_pixel(x as u32, y as u32, Rgba([255, 255, 255, self.tolerance]));
                }
            }
        }
    }


    pub fn separate_by_color(&mut self, color: [u8; 4]) {
        if self.path.contains(".jpg") {
            self.path = self.path.replace(".jpg", ".png");
        }
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
        for x in 0..self.width {
            for y in 0..self.height {
                let mut color = ((self.y_pre_sum_matrix[x][y] as usize * 255) / self.height) as u8;
                img.put_pixel(x as u32, y as u32, Rgba([255, 255 - color, 255, 255]));
            }
        }
        img.save(path).expect("Cant save the image");
    }

    pub fn dominant_outer_color(&self) -> [u8; 4] {
        self.img.get_pixel(0, 0).0
        // TODO : implement the better way of finding the dominate pixel
    }
}

