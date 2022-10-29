use std::io::{stdin, Write};

mod floor_plan;

fn main() {
    let  pic_path="img/outline.png";
    let mut floor_plan = floor_plan::Area::new(pic_path, 20);
    floor_plan.get_rid_of_all_light_color();

    floor_plan.gather_prominent_colors();
    floor_plan.create_wall_matrix();

    let mut file = std::fs::File::create("input.txt").unwrap();

    for y in 0..floor_plan.isWall.len() {
        let line= floor_plan.isWall[y].iter().map(|x| if *x==1 { "1" } else { "0" }).collect::<Vec<&str>>().join("");
        file.write_all(line.as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }



    // floor_plan.get_walls();
    println!("{:?}", floor_plan.walls);
    floor_plan.img.save("img/Untitled2.png").unwrap();
}

