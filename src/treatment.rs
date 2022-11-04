use opencv::core::Mat;
use opencv::types::VectorOfVectorOfPoint;

mod contours;

pub fn run(image: Mat) -> VectorOfVectorOfPoint {
    // 輪郭の抽出
    let contours = contours::extract(image);
    contours
}