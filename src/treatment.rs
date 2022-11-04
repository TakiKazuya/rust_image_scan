use opencv::core::Mat;

mod trapezoid;
mod contours;
mod tilt;

// 台形補正
pub fn correct_trapezoid(image: Mat) -> Mat {
    trapezoid::correct_trapezoid(image)
}

// 傾き補正
pub fn correct_tilt(image: Mat) -> Mat {
    tilt::correct_tilt(image)
}
