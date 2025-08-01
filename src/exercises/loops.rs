// 循环练习
pub fn run() {
    println!("=== 循环练习 ===");

    // 1. loop 循环
    println!("\n1. loop 循环:");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop 循环结果: {}", result);

    // 2. while 循环
    println!("\n2. while 循环:");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("发射！");

    // 3. for 循环遍历数组
    println!("\n3. for 循环遍历数组:");
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("数组元素: {}", element);
    }

    // 4. for 循环使用范围
    println!("\n4. for 循环使用范围:");
    for number in 1..4 {
        println!("数字: {}", number);
    }

    // 5. for 循环使用包含范围
    println!("\n5. for 循环使用包含范围:");
    for number in 1..=3 {
        println!("包含范围数字: {}", number);
    }

    // 6. for 循环反向遍历
    println!("\n6. for 循环反向遍历:");
    for number in (1..4).rev() {
        println!("反向数字: {}", number);
    }

    // 7. 嵌套循环
    println!("\n7. 嵌套循环:");
    for i in 1..=3 {
        for j in 1..=3 {
            print!("({},{}) ", i, j);
        }
        println!();
    }

    // 8. 循环标签和 break
    println!("\n8. 循环标签和 break:");
    'outer: loop {
        println!("进入外层循环");

        loop {
            println!("进入内层循环");
            break 'outer; // 跳出外层循环
        }

        // println!("这行不会被执行");
    }
    println!("跳出了外层循环");

    // 9. continue 语句
    println!("\n9. continue 语句:");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        println!("奇数: {}", i);
    }

    // 10. 使用 enumerate 获取索引
    println!("\n10. 使用 enumerate 获取索引:");
    let names = ["Alice", "Bob", "Charlie"];
    for (index, name) in names.iter().enumerate() {
        println!("索引 {}: {}", index, name);
    }

    // 11. while let 循环
    println!("\n11. while let 循环:");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }

    // 12. 迭代器方法
    println!("\n12. 迭代器方法:");
    let numbers: Vec<i32> = (1..6).collect();
    println!("原数组: {:?}", numbers);

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("翻倍后: {:?}", doubled);

    let sum: i32 = numbers.iter().sum();
    println!("总和: {}", sum);

    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);

    println!("\n循环练习完成！");
}
