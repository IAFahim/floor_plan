use std::io::{stdin, Write};

mod floor_plan;

fn main() {
    let  pic_path="img/simple_floor_plan_1.png";
    let mut floor_plan = floor_plan::Area::new(pic_path, 20);
    floor_plan.y_matrix_pre_sum_from_bottom_up();
    floor_plan.y_heat_map();
    floor_plan.separate_by_color([86, 93, 96, 255]);


}

