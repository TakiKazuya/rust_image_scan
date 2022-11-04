use opencv::core::{Mat, Vector, Size, Point, Scalar, BORDER_WRAP, BORDER_TRANSPARENT, BORDER_REPLICATE, CV_8UC3, no_array, VectorExtern, norm, Point2f, DECOMP_LU};
use opencv::imgcodecs::{IMREAD_GRAYSCALE, IMREAD_COLOR, imwrite};
use opencv::imgproc::{get_structuring_element, find_contours, threshold, morphology_ex, contour_area, draw_contours, arc_length, approx_poly_dp, circle, get_perspective_transform, warp_perspective};
use opencv::imgproc::{THRESH_OTSU, MORPH_OPEN, MORPH_CLOSE, MORPH_RECT, RETR_CCOMP, RETR_EXTERNAL, CHAIN_APPROX_SIMPLE, INTER_MAX, LINE_8, INTER_NEAREST, RETR_LIST};
use opencv::types::{VectorOfVectorOfPoint, VectorOfPoint, VectorOfPoint2f};
use opencv::core::NormTypes::NORM_L1;
use std::error::Error;

mod pretreatment;
mod treatment;
mod colors;

const SOURCE_IMAGE_PATH: &str = "image.jpg";

fn main(){
    // 元画像を読み込み
    println!("画像の読み込みを開始します。");

    // 処理元の画像を定義
    let mut src_img;
    let result_read_img = opencv::imgcodecs::imread(SOURCE_IMAGE_PATH, IMREAD_GRAYSCALE);
    match result_read_img {
        Ok(img) => src_img = img,
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    };

    let mut output_img;
    let result_read_img = opencv::imgcodecs::imread(SOURCE_IMAGE_PATH, IMREAD_COLOR);
    match result_read_img {
        Ok(img) => output_img = img,
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    };

    ////// 前処理ここから //////

    let img_pretreatment = pretreatment::run(src_img);

    ////// 前処理ここまで //////

    ////// 輪郭の抽出ここから//////

    let contours = treatment::get_contours(img_pretreatment);

    ////// 輪郭の抽出ここまで //////

    ////// 面積が最大になる輪郭を取得 //////

    let max_contour = treatment::get_max_contours(contours);

    ////// 面積が最大になる輪郭を取得ここまで //////

    ////// 図形の周囲の長さ取得ここから //////

    let arc_len = treatment::get_arc_len(&max_contour);

    ////// 図形の周囲の長さ取得ここまで //////

    ////// 図形の頂点抽出ここから //////

    let vertex_points = treatment::get_vertex_points(&max_contour, arc_len);

    ////// 図形の頂点抽出ここまで //////

    ////// 座標を左上、右上、右下、左下に分類する ここから //////

    let (left_up, left_down, right_up, right_down) = treatment::split_vertex_points(&vertex_points);

    ////// 座標を左上、右上、右下、左下に分類する ここまで //////

    ////// 台形補正処理 ここから //////

    let img_trapezoid_corrected = treatment::correct_trapezoid(left_up, left_down, right_up, right_down, &vertex_points);

    ////// 台形補正処理 ここまで //////

    // 全ての処理が終わったあと、画像を出力する
    println!("画像を出力します。");
    imwrite("output.jpg", &img_trapezoid_corrected, &Vector::new());
}
