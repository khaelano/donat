use std::f32::consts::PI;
use std::{thread::sleep, time::Duration};

const THETA_INCR: f32 = 0.07;
const PHI_INCR: f32 = 0.02;
const R1: f32 = 1.00;   // radius of circle
const R2: f32 = 2.00;   // radius of toroid
const K2: f32 = 5.00;   // distance from viewer
const K1: f32 = 1.00;   // FOV

fn main () {
    const  S_WIDTH: usize = 80;
    const S_HEIGHT: usize = 80;

    let a = 0.00;
    let b = 0.00;
    let mut zbuffer = [[0.00f32 ; S_WIDTH] ; S_HEIGHT];
    let mut output = [[' ' ; S_WIDTH] ; S_HEIGHT];

    loop {
        // Precalculate sin and cos of a and b
        let sin_a = f32::sin(a);
        let sin_b = f32::sin(b);
    
        let cos_a = f32::cos(a);
        let cos_b = f32::cos(b);

        // Logics
        let mut theta = 0.00;
        while theta < 2.00 * PI {
            let sin_theta = f32::sin(theta);
            let cos_theta = f32::cos(theta);

            let mut phi = 0.00;
            while phi < 2.00 * PI {
                let sin_phi = f32::sin(phi);
                let cos_phi = f32::cos(phi);

                // Circle
                let circle_x = R2 + R1 * cos_theta;
                let circle_y = R1 * sin_theta;

                // DONATT!!
                let tx = circle_x * cos_phi;
                let ty = circle_y;
                let tz = circle_x * sin_phi;

                // Rotating DONAT!! on x and z axis using matrix of rotation
                let ty_cosa = ty * cos_a;
                let tz_sina = tx * sin_a;

                let tx_rotated = tx * cos_b + ty_cosa * sin_b + tz_sina * sin_b;
                let ty_rotated = tx * -sin_b + ty_cosa * cos_b + tz_sina * cos_b;
                let tz_rotated = K2 + (ty * -sin_a + tz * cos_a);
                let ooz = 1.00 / tx_rotated;

                // Projection of the DONAT!!
                let px = (S_WIDTH as f32 / 2.00 + K1 * tx_rotated) as usize;
                let py = (S_HEIGHT as f32 / 2.00 - K1 * ty_rotated) as usize;

                // Finding two vector that tangent with two curves on point theta and phi
                // Vector that are tangent with curve 1 on circle
                let d_circle_x = R2 + R1 * cos_theta;
                let c1_dx = d_circle_x * cos_phi;
                let c1_dy = R1 * cos_theta;
                let c1_dz = d_circle_x * sin_phi;

                // Vector that are tangent with cuve 2 on torus
                let c2_dx = circle_x * -sin_phi;
                let c2_dy = R1 * sin_theta;
                let c2_dz = circle_x * cos_phi;

                // Compute the surface normal using cross product
                // c1_dx    c2_dx
                // c1_dy    c2_dy
                // c1_dz    c2_dz
                let mut nx = c1_dy * c2_dz - c1_dz * c2_dy;
                let mut ny = c1_dz * c2_dx - c1_dx * c2_dz;
                let mut nz = c1_dx * c2_dy - c1_dy * c2_dx;

                // Normalize the vector
                let surface_len = f32::sqrt(nx*nx + ny * ny + nz * nz);
                nx /= surface_len;
                ny /= surface_len;
                nz /= surface_len;

                // Rotate the surface normal
                let ny_cosa = ny * cos_a;
                let nz_sina = nx * sin_a;

                let tx_rotated = nx * cos_b + ny_cosa * sin_b + nz_sina * sin_b;
                let ty_rotated = nx * -sin_b + ny_cosa * cos_b + nz_sina * cos_b;
                let tz_rotated = K2 + (ny * -sin_a + nz * cos_a);

                // Dot product of normal and light direction
                let l = tx_rotated * 0.00 + ty_rotated * 1.00 + tz_rotated * -1.00;
                println!("{}", l);

                if l > 0.00 && ooz > zbuffer[py][px] {
                    zbuffer[py][px] = ooz;
                    let l_index = f32::floor(l * 5.00) as usize;
                    output[py][px] = ".,-~:;=!*#$@".chars().nth(l_index).unwrap();
                    output[py][px] = 'x';

                }
                phi += PHI_INCR;
            }
            theta += THETA_INCR;
        }
        print!("\x1B[2J\x1B[1;1H");
        for i in output.iter() {
            for j in i.iter() {
                print!("{}", j);
                // print!("*");

            }
            println!();
        }
        sleep(Duration::from_millis(20));
    }
    
}
