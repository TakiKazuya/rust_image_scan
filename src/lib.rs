#[no_mangle]
pub extern fn run(path: &str) -> &str {
    println!("path: {}", path);
    "hoge"
}