use std::io;

mod exercises;

fn main() {
    println!("🦀 欢迎来到 Rust 学习练习集！");
    println!("Welcome to Rust Learning Exercises!");
    println!("{}", "=".repeat(50));

    loop {
        show_menu();

        let mut input = String::new();
        println!("\n请输入你的选择 (Please enter your choice): ");

        io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败 (Failed to read input)");

        let choice = input.trim();

        match choice {
            "0" | "exit" | "quit" => {
                println!("\n👋 感谢使用，再见！");
                println!("Thanks for using, goodbye!");
                break;
            }
            "all" => {
                run_all_exercises();
            }
            _ => {
                if exercises::run_exercise(choice) {
                    println!("\n按 Enter 键继续...");
                    let mut _dummy = String::new();
                    io::stdin().read_line(&mut _dummy).ok();
                } else {
                    println!("❌ 无效选择，请重试！");
                    println!("Invalid choice, please try again!");
                }
            }
        }

        // 清屏 (在某些终端中有效)
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn show_menu() {
    println!("\n📚 可用的练习 (Available Exercises):");
    println!("{}", "-".repeat(50));

    for (number, description) in exercises::EXERCISES {
        println!("  {number} - {description}");
    }

    println!("\n📋 特殊选项 (Special Options):");
    println!("  all  - 运行所有练习 (Run all exercises)");
    println!("  0    - 退出程序 (Exit program)");
    println!("{}", "=".repeat(50));
}

fn run_all_exercises() {
    println!("\n🚀 运行所有练习...");
    println!("Running all exercises...");
    println!("{}", "=".repeat(60));

    for (number, description) in exercises::EXERCISES {
        println!("\n📍 开始练习 {number} - {description}");
        println!("{}", "-".repeat(40));

        exercises::run_exercise(number);

        println!("\n✅ 练习 {number} 完成");
        println!("{}", "-".repeat(40));

        // 在练习之间稍作停顿
        println!("按 Enter 键继续下一个练习...");
        let mut _dummy = String::new();
        io::stdin().read_line(&mut _dummy).ok();
    }

    println!("\n🎉 所有练习已完成！");
    println!("All exercises completed!");
}
