use opencv::core::{Mat, Vector, Point, Size, DECOMP_LU, Point2f};
use opencv::imgproc::{approx_poly_dp, arc_length, contour_area, get_perspective_transform, warp_perspective};
use opencv::imgcodecs::{IMREAD_COLOR};
use opencv::types::{VectorOfPoint, VectorOfPoint2f, VectorOfVectorOfPoint};
use crate::SOURCE_IMAGE_PATH;

mod contours;

// 台形補正
pub fn correct_trapezoid(image: Mat) -> Mat {

    // 輪郭の抽出
    let contours = get_contours(image);

    // 面積が最大になる輪郭を取得
    let max_contour = get_max_contours(contours);

    // 図形の周囲の長さ取得
    let arc_len = get_arc_len(&max_contour);

    // 図形の頂点抽出
    let vertex_points = get_vertex_points(&max_contour, arc_len);

    // 座標を左上、右上、右下、左下に分類する
    let (left_up, left_down, right_up, right_down) = split_vertex_points(&vertex_points);


    let upper_line = (right_up.x - left_up.x).abs() + (right_up.y - left_up.y).abs();
    let downer_line = (right_down.x - left_down.x).abs() + (right_down.y - left_down.y).abs();
    let left_line = (left_up.x - left_down.x).abs() + (left_up.y - left_down.y).abs();
    let right_line = (right_up.x - right_down.x).abs() + (right_up.y - right_down.y).abs();

    let max_x = if upper_line > downer_line {
        upper_line
    } else {
        downer_line
    };

    let max_y = if left_line > right_line {
        left_line
    } else {
        right_line
    };

    // 元となる座標をVector<Point2f>に変換する

    let mut vec_vertex_points2f: Vec<Point2f> = vec![];
    for p in vertex_points.iter() {
        vec_vertex_points2f.push(Point2f::new(p.x as f32, p.y as f32));
    };

    let vertex_points= VectorOfPoint2f::from(vec_vertex_points2f);

    let left_up = Point2f::new(0.0,0.0);
    let left_down = Point2f::new(0.0, max_y as f32);
    let right_down = Point2f::new(max_x as f32, max_y as f32);
    let right_up = Point2f::new(max_x as f32, 0.0);

    let coordinate: Vector<Point2f> = Vector::from(vec![left_up, left_down, right_down, right_up]);

    let m;
    match get_perspective_transform(&vertex_points, &coordinate, DECOMP_LU) {
        Ok(mat) => {
            println!("{:?}", mat);
            m= mat;
        },
        Err(code) => {
            panic!("Error. Message: {}", code)
        }
    };

    let src_img;
    let result_read_img = opencv::imgcodecs::imread(SOURCE_IMAGE_PATH, IMREAD_COLOR);
    match result_read_img {
        Ok(img) => src_img = img,
        Err(code) => {
            print!("code: {:?}", code);
            panic!();
        }
    };

    let mut img_corrected = Mat::default();
    match warp_perspective(&src_img, &mut img_corrected, &m, Size::new(max_x, max_y), 0, 0, Default::default()) {
        Ok(_) => {
            println!("success");
        },
        Err(code) => {
            panic!("{}", code);
        }
    }
    img_corrected
}

// 輪郭の抽出
fn get_contours(image: Mat) -> VectorOfVectorOfPoint {
    contours::extract(image)
}

//面積が最大になる輪郭を取得
fn get_max_contours(contours: VectorOfVectorOfPoint) -> Vector<Point> {
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

fn get_arc_len(max_contour: &Vector<Point>) -> f64 {
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

// 図形の頂点抽出
fn get_vertex_points(max_contour: &Vector<Point>, arc_len: f64) -> Vector<Point> {
    let mut vertex_points = VectorOfPoint::default();
    let result_approx_contour = approx_poly_dp(&max_contour, &mut vertex_points, 0.1 * arc_len, true);
    if let Err(code) = result_approx_contour {
        println!("頂点抽出に失敗しました。 Message: {}", code);
        panic!();
    }

    println!("vertex_points: {:?}", &vertex_points);

    vertex_points
}


// 座標を左上、右上、右下、左下に分類する
fn split_vertex_points(vertex_points: &Vector<Point>) -> (Point, Point, Point, Point) {
    // 抽出した頂点が４つであること。4つではない場合はエラーを返す
    if vertex_points.len() != 4 {
        panic!("頂点の数は4つである必要があります。抽出した頂点の数：{}", vertex_points.len());
    }

    // Vector<Point>型からVec<Point>型に変換
    let mut vec_vertex_points = vertex_points.to_vec();
    // 座標のxの値でソート
    vec_vertex_points.sort_by(|p, p1| {
        p.x.cmp(&p1.x)
    });

    // 左右で座標を分割
    let mut left = vec_vertex_points.get(0..2).unwrap().to_owned();
    let mut right = vec_vertex_points.get(2..4).unwrap().to_owned();

    // y軸でソートし、0番目を上側、1番目を下側とする
    // 左側
    left.sort_by(|p, p1| {
        p.y.cmp(&p1.y)
    });
    let left_up = left.first().unwrap().to_owned();
    let left_down = left.last().unwrap().to_owned();

    // 右側
    right.sort_by(|p, p1| {
        p.y.cmp(&p1.y)
    });
    let right_up = right.first().unwrap().to_owned();
    let right_down = right.last().unwrap().to_owned();

    println!("left_up: {:?}", left_up);
    println!("left_down: {:?}", left_down);
    println!("right_up: {:?}", right_up);
    println!("right_down: {:?}", right_down);

    (left_up, left_down, right_up, right_down)
}
