use aspect_macro::{get, loggable};

#[get(path = "/user", params = "name=1,age=30")]
fn get_user() {
    // 这里是原始函数的代码
    println!("Fetching user...");
}

#[loggable(time = true, args = true)]
fn test_function(x: i32, y: i32) -> i32 {
    std::thread::sleep(std::time::Duration::from_millis(500));
    x + y
}

fn main() {
    let result = test_function(5, 10);
    println!("Result: {}", result);
}
