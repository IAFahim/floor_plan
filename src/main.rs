use std::io::{stdin, Write};

mod floor_plan;

fn main() {
    let  pic_path="img/simple_floor_plan_1.png";
    let mut floor_plan = floor_plan::Area::new(pic_path, 20);
    floor_plan.y_axis_pre_sum();
    let txt_path= pic_path.replace(".png", ".txt");
    let mut buf=std::io::BufWriter::new(std::fs::File::create(txt_path).unwrap());
    buf.write_fmt(format_args!("{:?}", floor_plan.y_pre_sum_bottom_up)).unwrap();
}

