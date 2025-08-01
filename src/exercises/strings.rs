// 字符串练习
pub fn run() {
    println!("=== 字符串练习 ===");

    // 1. 字符串字面量
    println!("\n1. 字符串字面量:");
    let s1 = "Hello, world!";
    println!("字符串字面量: {}", s1);

    // 2. String 类型
    println!("\n2. String 类型:");
    let mut s2 = String::from("Hello");
    println!("初始字符串: {}", s2);

    s2.push_str(", world!");
    println!("添加后的字符串: {}", s2);

    // 3. 字符串拼接
    println!("\n3. 字符串拼接:");
    let s3 = String::from("Hello");
    let s4 = String::from(" Rust");
    let s5 = s3 + &s4; // s3 被移动了，不能再使用
    println!("拼接结果: {}", s5);

    // 4. format! 宏
    println!("\n4. format! 宏:");
    let name = "张三";
    let age = 25;
    let message = format!("我的名字是 {}，我今年 {} 岁", name, age);
    println!("{}", message);

    // 5. 字符串切片
    println!("\n5. 字符串切片:");
    let hello = "Hello, world!";
    let hello_slice = &hello[0..5];
    let world_slice = &hello[7..12];
    println!("Hello 切片: {}", hello_slice);
    println!("World 切片: {}", world_slice);

    // 6. 字符串方法
    println!("\n6. 字符串方法:");
    let text = "  Hello Rust  ";
    println!("原始字符串: '{}'", text);
    println!("去除空白: '{}'", text.trim());
    println!("转换为大写: '{}'", text.trim().to_uppercase());
    println!("转换为小写: '{}'", text.trim().to_lowercase());
    println!("包含 'Rust': {}", text.contains("Rust"));
    println!("字符串长度: {}", text.trim().len());

    // 7. 字符串分割
    println!("\n7. 字符串分割:");
    let data = "apple,banana,orange";
    let fruits: Vec<&str> = data.split(',').collect();
    println!("水果列表: {:?}", fruits);

    // 8. 字符迭代
    println!("\n8. 字符迭代:");
    let hello_chinese = "你好";
    print!("逐字符输出: ");
    for c in hello_chinese.chars() {
        print!("{} ", c);
    }
    println!();

    // 9. 字节迭代
    println!("\n9. 字节迭代:");
    let hello_bytes = "Hello";
    print!("字节表示: ");
    for b in hello_bytes.bytes() {
        print!("{} ", b);
    }
    println!();

    // 10. 字符串替换
    println!("\n10. 字符串替换:");
    let text = "Hello World";
    let new_text = text.replace("World", "Rust");
    println!("原文本: {}", text);
    println!("替换后: {}", new_text);

    println!("\n字符串练习完成！");
}
