use std::io::{stdin, Write};

mod floor_plan;

fn main() {
    let  pic_path="img/Untitled.png";
    let mut floor_plan = floor_plan::Area::new(pic_path, 20);
    floor_plan.get_rid_of_all_light_color();

    floor_plan.create_y_matrix_pre_sum_from_bottom_up();
    floor_plan.gather_prominent_colors();
    floor_plan.create_wall_matrix();
    

    floor_plan.get_walls();
    println!("{:?}", floor_plan.walls);
    floor_plan.img.save("img/Untitled2.png").unwrap();
}

