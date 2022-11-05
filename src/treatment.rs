use opencv::core::Mat;

mod trapezoid;
mod contours;
mod tilt;

pub fn run(image: Mat) -> Mat {
    let img_trapezoid_corrected = trapezoid::correct_trapezoid(image);
    let img_tilt_corrected = tilt::correct_tilt(img_trapezoid_corrected);
    img_tilt_corrected
}
