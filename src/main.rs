use fastrand;
use macroquad::prelude::*;
use rayon::prelude::*;

mod hsl;
use hsl::hue_to_rgb;

mod grid;
use grid::Grid;

//Constants
const N: usize = 1500;
const COLORS: usize = 5;
const FRICTION: f32 = 0.8;
const FORCE: f32 = 2.0;
const TIME_STEP: f32 = 1.0 / 45.0;

const INFLUENCE_RADIUS: f32 = 80.0;
const RADIUS: f32 = 3.0;

//Rest of the code
fn force(dist: f32, a: f32) -> f32 {
    const A_MIN: f32 = 5.0;
    const RNORMAL: f32 = 3.0 * RADIUS / INFLUENCE_RADIUS;

    if dist <= RNORMAL {
        A_MIN / RNORMAL * dist - A_MIN
    } else if dist < 1.0 {
        let up = (1.0 + RNORMAL - 2.0 * dist).abs();
        let down = RNORMAL - 1.0;

        (up / down + 1.0) * a
    } else {
        0.0
    }
}

#[inline]
fn rnd(start: f32, end: f32) -> f32 {
    fastrand::f32() * (end - start) + start
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle life".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut vel_x = [0.0; N];
    let mut vel_y = [0.0; N];

    let mut pos_x = [0.0; N];
    let mut pos_y = [0.0; N];

    let mut p_cols = [0; N];
    let mut col_matrix = [[0.0; COLORS]; COLORS];
    let mut computed_colors: Vec<Color> = Vec::new();

    let mut is_debug = false;

    {
        let w = screen_width();
        let h = screen_height();
        for i in 0..N {
            pos_x[i] = rnd(0.0, w);
            pos_y[i] = rnd(0.0, h);
            p_cols[i] = fastrand::usize(0..COLORS);
        }

        for x in 0..COLORS {
            for y in 0..COLORS {
                col_matrix[x][y] = rnd(-1.0, 1.0);
            }

            let hue = (x as f32 / COLORS as f32) * 360.0;
            let rgb = hue_to_rgb(hue);
            let color = Color::new(rgb.0, rgb.1, rgb.2, 1.0);
            computed_colors.push(color);
        }
    }

    loop {
        clear_background(BLACK);

        let mut grid = Grid::new(INFLUENCE_RADIUS, screen_width(), screen_height());

        let grid_w = grid.cols as f32 * grid.cell_size;
        let grid_h = grid.rows as f32 * grid.cell_size;

        for (i, (x, y)) in pos_x.iter().zip(pos_y.iter()).enumerate() {
            grid.insert(i, *x, *y);
        }

        let forces: Vec<(f32, f32)> = (0..N)
            .into_par_iter()
            .map(|i| {
                let mut forcex: f32 = 0.0;
                let mut forcey: f32 = 0.0;

                for j in grid.query(pos_x[i], pos_y[i]) {
                    if i == j {
                        continue;
                    }

                    let dx = pos_x[j] - pos_x[i];
                    let dy = pos_y[j] - pos_y[i];

                    let w_dx = dx - grid_w * (dx / grid_w).round();
                    let w_dy = dy - grid_h * (dy / grid_h).round();

                    let r_squared = w_dx * w_dx + w_dy * w_dy;
                    if r_squared > 0.0 && r_squared < INFLUENCE_RADIUS * INFLUENCE_RADIUS {
                        let r = r_squared.sqrt();
                        let a = col_matrix[p_cols[i]][p_cols[j]];
                        let f = force(r / INFLUENCE_RADIUS, a);

                        forcex += w_dx / r * f;
                        forcey += w_dy / r * f;
                    }
                }

                (
                    forcex * INFLUENCE_RADIUS * FORCE,
                    forcey * INFLUENCE_RADIUS * FORCE,
                )
            })
            .collect();

        for i in 0..N {
            vel_x[i] *= FRICTION;
            vel_y[i] *= FRICTION;

            vel_x[i] += forces[i].0 * TIME_STEP;
            vel_y[i] += forces[i].1 * TIME_STEP;

            pos_x[i] += vel_x[i] * TIME_STEP;
            pos_y[i] += vel_y[i] * TIME_STEP;

            pos_x[i] = pos_x[i].rem_euclid(grid_w);
            pos_y[i] = pos_y[i].rem_euclid(grid_h);
        }

        //render
        for i in 0..N {
            let (x, y) = (pos_x[i], pos_y[i]);
            let c = p_cols[i];
            draw_circle(x, y, RADIUS, computed_colors[c]);
        }

        //show how much time it took to render
        let dt = get_frame_time();
        if is_debug {
            draw_text(
                &format!("Frame time: {:.2} ms\n", dt * 1000.0),
                10.0,
                20.0,
                30.0,
                WHITE,
            );

            draw_text(&format!("Fps: {:.0}", 1.0 / dt), 10.0, 50.0, 30.0, WHITE);
        };

        if is_key_released(KeyCode::N) {
            for x in 0..COLORS {
                for y in 0..COLORS {
                    col_matrix[x][y] = rnd(-1.0, 1.0);
                }
            }
        }

        if is_key_released(KeyCode::M) {
            is_debug = !is_debug;
        }

        next_frame().await
    }
}
