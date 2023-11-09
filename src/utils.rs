pub fn torus (torus_r: f32, r: f32, theta: f32, phi: f32) -> (f32, f32, f32) {
    // function that creates torus
    let (sin_theta, cos_theta) = theta.sin_cos();
    let (sin_phi, cos_phi) = phi.sin_cos();

    let circle_x = torus_r + r * cos_theta;
    let x = circle_x * cos_phi;
    let y = sin_theta;
    let z = circle_x * sin_phi;

    (x, y, z)
}

pub fn rotate_x (x: f32, y:f32, z: f32, angle: f32) -> (f32, f32, f32) {
    let (sin_angle, cos_angle) = angle.sin_cos();

    // function that rotates point on axis on 3d plane
    let x_rotated = x;
    let y_rotated = y * cos_angle + z * sin_angle;
    let z_rotated = y * -sin_angle + z * cos_angle;

    (x_rotated, y_rotated, z_rotated)
}

pub fn rotate_y (x: f32, y:f32, z: f32, angle: f32) -> (f32, f32, f32) {
    let (sin_angle, cos_angle) = angle.sin_cos();

    // function that rotates point on axis on 3d plane
    let x_rotated = x * cos_angle + z * -sin_angle;
    let y_rotated = y;
    let z_rotated = x * sin_angle + z * cos_angle;

    (x_rotated, y_rotated, z_rotated)
}

pub fn rotate_z (x: f32, y:f32, z: f32, angle: f32) -> (f32, f32, f32) {
    let (sin_angle, cos_angle) = angle.sin_cos();

    // function that rotates point on axis on 3d plane
    let x_rotated = x * cos_angle + y * sin_angle;
    let y_rotated = x * -sin_angle + y * cos_angle;
    let z_rotated = z;

    (x_rotated, y_rotated, z_rotated)
}

pub fn project (x: f32, y: f32, z: f32, k1: f32) -> (u32, u32){
    let ooz = 1.0/z;
    let px = (k1 * x * ooz) as u32;
    let py = (k1 * y * ooz) as u32;

    (px, py)
}