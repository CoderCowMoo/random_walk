use flo_draw::*;
use flo_canvas::*;
use rand;
use rand::seq::SliceRandom;
use std::time;

pub fn main() {
    with_2d_graphics(|| {
        let canvas = create_canvas_window("Random walk");

        let offset: f32 = 10.0; // pixels
        let steps = 1000;
        let mut prev_x = 500.0;
        let mut prev_y = 500.0;
        
        let up = 1.0; let down = -1.0; let left = -1.0; let right = 1.0;
        let vdirection = vec![left, right, up, down];
        let xychoice = vec![0, 1];
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);
            gc.stroke_color(Color::Rgba(1.0, 0.0, 1.0, 1.0));
        });

        for _ in 0..steps {
            canvas.draw(|gc| {
                // start a random walk
                gc.new_path();
                gc.move_to(prev_x, prev_y);
                // choose a random direction to move and move a offset distance
                let direction = vdirection.choose(&mut rand::thread_rng()).unwrap();
                // choose randomly whether to apply the direction to x or y
                if xychoice.choose(&mut rand::thread_rng()).unwrap() == &0 {
                    let x = prev_x + offset * direction;
                    let y = prev_y;
                    gc.line_to(x, y);
                    prev_x = x;
                } else {
                    let x = prev_x;
                    let y = prev_y + offset * direction;
                    gc.line_to(x, y);
                    prev_y = y;
                }
                gc.stroke();
            });
            // now that we've drawn, sleep for like 0.2 seconds
            std::thread::sleep(time::Duration::from_millis(20));
        };
    });
}
