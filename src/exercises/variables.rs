// 变量练习
pub fn run() {
    println!("=== 变量练习 ===");

    // 1. 不可变变量
    println!("\n1. 不可变变量:");
    let x = 5;
    println!("x 的值是: {}", x);

    // 2. 可变变量
    println!("\n2. 可变变量:");
    let mut y = 10;
    println!("y 的初始值: {}", y);
    y = 20;
    println!("y 的新值: {}", y);

    // 3. 常量
    println!("\n3. 常量:");
    const MAX_POINTS: u32 = 100_000;
    println!("最大分数: {}", MAX_POINTS);

    // 4. 变量遮蔽(Shadowing)
    println!("\n4. 变量遮蔽:");
    let z = 5;
    println!("z 的第一个值: {}", z);

    let z = z + 1;
    println!("z 的第二个值: {}", z);

    {
        let z = z * 2;
        println!("内部作用域中 z 的值: {}", z);
    }

    println!("外部作用域中 z 的值: {}", z);

    // 5. 类型转换
    println!("\n5. 类型转换:");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("字符串的长度: {}", spaces);

    // 6. 元组解构
    println!("\n6. 元组解构:");
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("元组解构: x={}, y={}, z={}", x, y, z);

    // 7. 数组
    println!("\n7. 数组:");
    let months = [
        "一月",
        "二月",
        "三月",
        "四月",
        "五月",
        "六月",
        "七月",
        "八月",
        "九月",
        "十月",
        "十一月",
        "十二月",
    ];
    println!("第一个月: {}", months[0]);
    println!("数组长度: {}", months.len());

    println!("\n变量练习完成！");
}
