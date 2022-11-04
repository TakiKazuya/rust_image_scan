use opencv::core::Mat;

mod trapezoid;
mod contours;

// 台形補正
pub fn correct_trapezoid(image: Mat) -> Mat {
    trapezoid::correct_trapezoid(image)
}
