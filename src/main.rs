use miniquad::*;
mod render;

use render::Stage;

fn main() {
    const N: usize = 10;
    let _pos_x = [0.0; N];
    let _pos_y = [0.0; N];

    fn update(_stage: &mut Stage) {
        println!("Updating");
    }

    fn render(_stage: &mut Stage) {
        println!("Rendering");
    }

    miniquad::start(conf::Conf::default(), move || {
        Box::new(Stage::new(10, update, render))
    });
}
