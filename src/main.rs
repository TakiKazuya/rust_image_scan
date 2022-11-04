use opencv::core::{Vector};
use opencv::imgcodecs::{IMREAD_GRAYSCALE, IMREAD_COLOR, imwrite};

mod pretreatment;
mod treatment;
mod colors;

const SOURCE_IMAGE_PATH: &str = "image.jpg";

fn main(){
    // 元画像を読み込み
    println!("画像の読み込みを開始します。");

    // 処理元の画像を定義
    let src_img;
    let result_read_img = opencv::imgcodecs::imread(SOURCE_IMAGE_PATH, IMREAD_GRAYSCALE);
    match result_read_img {
        Ok(img) => src_img = img,
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    };

    let output_img;
    let result_read_img = opencv::imgcodecs::imread(SOURCE_IMAGE_PATH, IMREAD_COLOR);
    match result_read_img {
        Ok(img) => output_img = img,
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    };

    // 前処理
    let img_pretreatment = pretreatment::run(src_img);

    // 輪郭の抽出
    let contours = treatment::get_contours(img_pretreatment);

    // 面積が最大になる輪郭を取得
    let max_contour = treatment::get_max_contours(contours);

    // 図形の周囲の長さ取得
    let arc_len = treatment::get_arc_len(&max_contour);

    // 図形の頂点抽出
    let vertex_points = treatment::get_vertex_points(&max_contour, arc_len);

    // 座標を左上、右上、右下、左下に分類する
    let (left_up, left_down, right_up, right_down) = treatment::split_vertex_points(&vertex_points);

    // 台形補正処理
    let img_trapezoid_corrected = treatment::correct_trapezoid(left_up, left_down, right_up, right_down, &vertex_points);

    // 全ての処理が終わったあと、画像を出力する
    println!("画像を出力します。");
    imwrite("output.jpg", &img_trapezoid_corrected, &Vector::new());
}
