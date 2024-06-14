use std::io::{self, Write}; // 引入Write用于flush

// 在华氏温度和摄氏度之间转换温度
fn main() {
    // 提示语
    println!("请输入您要转换的类型, 华氏温度转换摄氏度请输入1, 摄氏度转换华氏温度请输入0");

    let mut conversion_type = String::new();
    io::stdin()
        .read_line(&mut conversion_type)
        .expect("读取失败");

    let conversion_type: i32 = conversion_type.trim().parse().expect("请输入有效的数字");

    // 输入温度
    println!("请输入温度值：");

    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("读取失败");

    let temperature: f64 = temperature.trim().parse().expect("请输入有效的温度值");

    // 转换
    let new_temperature = if conversion_type == 1 {
        f_to_c(temperature)
    } else {
        c_to_f(temperature)
    };

    // 输出温度
    println!("转换后的温度为 {}", new_temperature);
}

fn f_to_c(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}

fn c_to_f(x: f64) -> f64 {
    x * 9.0 / 5.0 + 32.0
}
