use opencv::core::{Mat, Vector, Point};
use opencv::imgproc::{approx_poly_dp, arc_length, contour_area};
use opencv::types::{VectorOfPoint, VectorOfVectorOfPoint};

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

pub fn get_arc_len(max_contour: &Vector<Point>) -> f64 {
    // 図形の周囲の長さ取得

    let result_arc_length = arc_length(&max_contour, true);
    let arc_len;
    match result_arc_length {
        Ok(length) => {
            arc_len = length;
            println!("arc_len: {}", arc_len)
        },
        Err(code) => {
            print!("図形の周囲の長さの取得に失敗しました。 Message: {}", code);
            panic!();
        }
    };
    arc_len
}

pub fn get_vertex_points(max_contour: &Vector<Point>, arc_len: f64) -> Vector<Point> {
    // 図形の頂点抽出

    let mut vertex_points = VectorOfPoint::default();
    let result_approx_contour = approx_poly_dp(&max_contour, &mut vertex_points, 0.1 * arc_len, true);
    if let Err(code) = result_approx_contour {
        println!("頂点抽出に失敗しました。 Message: {}", code);
        panic!();
    }

    println!("vertex_points: {:?}", &vertex_points);

    vertex_points
}