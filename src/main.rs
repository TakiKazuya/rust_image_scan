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

    // 台形補正処理
    let img_trapezoid_corrected = treatment::correct_trapezoid(img_pretreatment);

    // 傾き補正処理
    // let img_tilt_corrected = treatment::correct_tilt(img_trapezoid_corrected);

    // 全ての処理が終わったあと、画像を出力する
    println!("画像を出力します。");
    imwrite("output.jpg", &img_trapezoid_corrected, &Vector::new());
}
