use opencv::core::{Mat};

mod threshold;
mod morphology;

pub fn run(image: Mat) -> Mat {
    println!("前処理開始");

    // ２値化処理
    let img_threshold = threshold::run(image);

    // クロージング処理
    let img_closed = morphology::closing(img_threshold);

    // オープニング処理
    let img_opened = morphology::opening(img_closed);

    // imwrite("output_pretreatment.jpg", &img_opened, &Vector::new());

    println!("前処理終了");
    img_opened
}