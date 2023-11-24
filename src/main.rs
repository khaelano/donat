use std::f32::consts::PI;
use std::{thread::sleep, time::Duration};

mod utils;
use utils::{rotate_x, rotate_y, rotate_z, torus};

const S_WIDTH: usize = 50_usize;
const S_HEIGHT: usize = 50_usize;

const THETA_INCR: f32 = 0.07;
const PHI_INCR: f32 = 0.02;
const R1: f32 = 1.00; // radius of circle that revolves an axis to make torus
const R2: f32 = 2.00; // radius of torus

const K2: f32 = 8.00; // distance from viewer
// const K1: f32 = 15.0; // FOV
const K1: f32 = (S_WIDTH as f32 * K2 * 3.0) / (8.0 * (R1 + R2));   // FOV

fn main() -> ! {
    print!("\x1b[2J");
    // represents the current angle of torus
    let (mut a, mut b) = (0.0, 90.0);

    loop {
        a += 0.2;
        b += 0.2;

        // reset either a or b above if its value above 2 * PI
        if a > 2.0 * PI {
            a = 0.0;
        }
        if b > 2.0 * PI {
            b = 0.0;
        }

        // create output and zbuffer
        let mut output = [[' '; S_WIDTH]; S_HEIGHT];
        let mut zbuffer = [[0.0f32; S_WIDTH]; S_HEIGHT];

        // the loop of circle
        let mut theta = 0.0;
        while theta < 2.0 * PI {
            theta += THETA_INCR;

            // the loop of torus
            let mut phi = 0.0;
            while phi < 2.0 * PI {
                phi += PHI_INCR;

                // create torus
                let (mut tx, mut ty, mut tz) = torus(R2, R1, theta, phi);
                // rotate torus on y axis
                (tx, ty, tz) = rotate_x(tx, ty, tz, a);
                //(tx, ty, tz) = rotate_y(tx, ty, tz, a);
                (tx, ty, tz) = rotate_z(tx, ty, tz, b);

                tz += K2; // added with K2 because K2 is the distance of point from screen
                let ooz = 1.0 / tz;

                // create 2d projection of torus
                let p_tx = S_WIDTH as i32 / 2 + (K1 * tx * ooz) as i32;
                let p_ty = S_HEIGHT as i32 / 2 - (K1 * ty * ooz) as i32;

                // get the torus surface normal
                let (mut nx, mut ny, mut nz) = torus(0.0, 1.0, theta, phi);
                // rotate the surface normal on y axis
                (nx, ny, nz) = rotate_x(nx, ny, nz, a);
                //(nx, ny, nz) = rotate_y(nx, ny, nz, a);
                (nx, ny, nz) = rotate_z(nx, ny, nz, b);

                // get luminance using dot product
                let l = ny - nz;
                let l_index = f32::floor(l * 8.0) as usize;

                if l > 0.0 && ooz > zbuffer[p_ty as usize][p_tx as usize] {
                    zbuffer[p_ty as usize][p_tx as usize] = ooz;
                    // output[p_ty as usize][p_tx as usize] = "@$#*!=;:^~,."
                    output[p_ty as usize][p_tx as usize] = ".,-~:;=!*#$@"
                        .chars()
                        .nth(l_index)
                        .expect("Error cannot indexp_ty as usize luminance level");
                }
            }
        }
        print!("\x1b[H");
        // print the output
        for j in output {
            for i in j {
                print!("{}", i);
            }
            println!()
        }

        sleep(Duration::from_millis(500));
    }
}
