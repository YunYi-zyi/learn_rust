// 条件语句练习
pub fn run() {
    println!("=== 条件语句练习 ===");

    // 1. 基本 if 语句
    println!("\n1. 基本 if 语句:");
    let number = 5;
    if number < 10 {
        println!("{number} 小于 10");
    }

    // 2. if-else 语句
    println!("\n2. if-else 语句:");
    let age = 18;
    if age >= 18 {
        println!("你已经成年了");
    } else {
        println!("你还未成年");
    }

    // 3. 多重 if-else if-else
    println!("\n3. 多重 if-else if-else:");
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    };
    println!("分数 {score} 对应等级: {grade}");

    // 4. if 在 let 语句中
    println!("\n4. if 在 let 语句中:");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("根据条件选择的数字: {number}");

    // 5. 逻辑运算符
    println!("\n5. 逻辑运算符:");
    let x = 5;
    let y = 10;

    // && (逻辑与)
    if x > 0 && y > 0 {
        println!("x 和 y 都是正数");
    }

    // || (逻辑或)
    if x < 0 || y < 0 {
        println!("x 或 y 中至少有一个是负数");
    } else {
        println!("x 和 y 都不是负数");
    }

    // ! (逻辑非)
    let is_sunny = true;
    if !is_sunny {
        println!("今天不是晴天");
    } else {
        println!("今天是晴天");
    }

    // 6. 嵌套条件
    println!("\n6. 嵌套条件:");
    let temperature = 25;
    let is_raining = false;

    if temperature > 20 {
        if is_raining {
            println!("天气温暖但在下雨，带把伞吧");
        } else {
            println!("天气温暖且晴朗，适合外出");
        }
    } else {
        println!("天气有点冷");
    }

    // 7. match 表达式 - 基本用法
    println!("\n7. match 表达式 - 基本用法:");
    let coin = "一角";
    let value = match coin {
        "一分" => 0.01,
        "五分" => 0.05,
        "一角" => 0.10,
        "五角" => 0.50,
        "一元" => 1.00,
        _ => 0.0, // 默认情况
    };
    println!("{coin} 硬币的价值: {value} 元");

    // 8. match 与数字
    println!("\n8. match 与数字:");
    let number = 13;
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("{number} 是一个小的质数"),
        13..=19 => println!("{number} 是青少年数字"),
        _ => println!("不是特殊数字"),
    }

    // 9. match 绑定值
    println!("\n9. match 绑定值:");
    let point = (0, 5);
    match point {
        (0, y) => println!("在 y 轴上的点，y = {y}"),
        (x, 0) => println!("在 x 轴上的点，x = {x}"),
        (x, y) => println!("点的坐标: ({x}, {y})"),
    }

    // 10. if let 语法糖
    println!("\n10. if let 语法糖:");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用你最喜欢的颜色 {color} 作为背景");
    } else if is_tuesday {
        println!("星期二是绿色的日子！");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色作为背景颜色");
        } else {
            println!("使用橙色作为背景颜色");
        }
    } else {
        println!("使用蓝色作为背景颜色");
    }

    // 11. while let 循环
    println!("\n11. while let 循环:");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("弹出: {top}");
    }

    // 12. 条件表达式的复杂示例
    println!("\n12. 条件表达式的复杂示例:");
    check_number(42);
    check_number(-5);
    check_number(0);

    println!("\n条件语句练习完成！");
}

// 辅助函数：检查数字的各种属性
fn check_number(num: i32) {
    println!("\n检查数字: {num}");

    // 正负性
    let sign = if num > 0 {
        "正数"
    } else if num < 0 {
        "负数"
    } else {
        "零"
    };

    // 奇偶性
    let parity = if num % 2 == 0 { "偶数" } else { "奇数" };

    // 大小范围
    let size = match num.abs() {
        0 => "零",
        1..=10 => "小",
        11..=100 => "中",
        _ => "大",
    };

    println!("  {num} 是一个{size}的{sign}");
    if num != 0 {
        println!("  它是{parity}");
    }

    // 特殊数字判断
    match num {
        0 => println!("  这是零，既不是正数也不是负数"),
        1 => println!("  这是最小的正整数"),
        -1 => println!("  这是最大的负整数"),
        42 => println!("  这是生命、宇宙以及一切的答案！"),
        _ => println!("  这是一个普通的数字"),
    }
}
