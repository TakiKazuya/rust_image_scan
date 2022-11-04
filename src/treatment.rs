use opencv::core::{Mat, Vector, Point};
use opencv::imgproc::contour_area;
use opencv::types::VectorOfVectorOfPoint;

mod contours;

pub fn get_contours(image: Mat) -> VectorOfVectorOfPoint {
    // 輪郭の抽出
    contours::extract(image)
}

pub fn get_max_contours(contours: VectorOfVectorOfPoint) -> Vector<Point> {
    //面積が最大になる輪郭を取得

    println!("面積が最大になる輪郭を取得する処理開始");

    // 輪郭の面積を保存するベクタを定義する。 要素の型はf64
    // 抽出した輪郭(contours)から面積を取得し、配列に追加していく。
    let contour_areas: Vec<f64> = contours.iter().map(|contour| {
        contour_area(&contour, false).unwrap_or(0.0)
    }).collect();

    println!("contour_areas: {:?}", contour_areas);

    // 最大値を取得する。
    let max_area = contour_areas.iter().fold(0.0/0.0, |m, v| v.max(m));

    // インデックスを取得
    let index = contour_areas.iter().position(|&area| area == max_area).unwrap();

    // 取得したインデックスから輪郭の情報を取得する。
    let max_contour = contours.get(index).unwrap();

    println!("面積が最大になる輪郭 -> {:?}", max_contour);

    println!("面積が最大になる輪郭を取得する処理終了");
    max_contour
}