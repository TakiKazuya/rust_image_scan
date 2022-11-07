#[no_mangle]
pub extern fn run(path: String) -> String {
    println!("path: {}", path);
    "hoge".to_string()
}