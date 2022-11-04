use std::collections::HashMap;
use opencv::core::{CV_PI, Point, Scalar, Vector, Size, MatTrait, Point2f, BORDER_CONSTANT, MatTraitManual, Mat};
use opencv::imgcodecs::{imread, imwrite, IMREAD_GRAYSCALE, IMREAD_COLOR};
use opencv::imgproc::{canny, hough_lines_p, line, warp_affine, get_rotation_matrix_2d, WARP_INVERSE_MAP, threshold, THRESH_OTSU, cvt_color, COLOR_BGR2GRAY};
use opencv::types::{VectorOfVec4i};
use ang::atan2;


pub fn correct_tilt(image: Mat) -> Mat {
    let width = image.cols();
    let height = image.rows();

    let mut gray_img = Mat::default();
    cvt_color(&image, &mut gray_img, COLOR_BGR2GRAY, 0);

    let max_thresh_val = threshold(&gray_img, &mut Mat::default(), 0.0, 255.0, THRESH_OTSU).unwrap();
    let min_thresh_val = max_thresh_val * 0.5;
    let max_line_gap = (((width * width) + (height * height)) as f64).sqrt();

    // ハフ変換による直線検出
    let mut line_img = image.clone();
    let mut lines= VectorOfVec4i::default();
    let threshold_val_for_hough = (max_thresh_val * 2.0) as i32;
    hough_lines_p(&gray_img, &mut lines, 1.0, CV_PI / 180.0, threshold_val_for_hough, 0.0, max_line_gap);

    // 線分の角度の配列を作成する
    let mut angles = vec![];
    for line_vec in lines.to_vec() {
        let x1 = line_vec[0] as f64;
        let y1 = line_vec[1] as f64;
        let x2 = line_vec[2] as f64;
        let y2 = line_vec[3] as f64;
        let angle = atan2(y2 - y1, x1 - x2).in_degrees().round() as i32;
        angles.push(angle);
    }

    println!("ここ{:?}", &lines);

    // 角度の配列から最頻値を取得(複数ある場合は最初の要素を選択)
    let angle = get_mode(&angles).first().unwrap().clone();

    // 角度が0or90の場合は何もしない。
    // それ以外はアフィン変換
    let result_img = if angle.abs() == 0 || angle.abs() == 90 {
        image
    } else {
        let mut dst_img = image.clone();
        let center = Point2f::new((width/2) as f32, (height/2) as f32); // 回転中心
        let rotation_angle = (angle - 180) as f64; // 回転する角度

        let m =
            get_rotation_matrix_2d(center, rotation_angle, 1.0)
                .unwrap_or_else(|code| {
                    panic!("code: {}", code)
                });

        let size = Size::new(width, height); // 出力画像のサイズ
        let result_affine = warp_affine(&image, &mut dst_img, &m, size, WARP_INVERSE_MAP, BORDER_CONSTANT, Scalar::default());

        match result_affine {
            Ok(_) => {
                dst_img
            },
            Err(code) => {
                panic!("code: {}", code);
            }
        }
    };

    result_img
}

pub fn get_mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for integer in numbers {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}
