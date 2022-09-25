mod floor_plan;

fn main() {
    let floor_plan = floor_plan::Area::new("img/simple_floor_plan_1.png", 5);
    println!("Floor plan: {:?}", floor_plan.pixel_to_remove());
}

