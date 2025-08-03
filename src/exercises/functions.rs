// 函数练习
pub fn run() {
    println!("=== 函数练习 ===");

    // 1. 基本函数调用
    println!("\n1. 基本函数调用:");
    hello_world();

    // 2. 带参数的函数
    println!("\n2. 带参数的函数:");
    greet("张三");
    print_labeled_measurement(5, '米');

    // 3. 有返回值的函数
    println!("\n3. 有返回值的函数:");
    let x = five();
    println!("five() 返回: {x}");

    let result = plus_one(5);
    println!("plus_one(5) 返回: {result}");

    // 4. 函数表达式
    println!("\n4. 函数表达式:");
    let y = {
        let x = 3;
        x + 1 // 注意没有分号，这是表达式
    };
    println!("表达式块的值: {y}");

    // 5. 多个参数的函数
    println!("\n5. 多个参数的函数:");
    let sum = add(10, 20);
    println!("add(10, 20) = {sum}");

    let product = multiply(4, 7);
    println!("multiply(4, 7) = {product}");

    // 6. 函数作为参数
    println!("\n6. 函数作为参数:");
    apply_operation(5, 3, add);
    apply_operation(5, 3, multiply);

    // 7. 闭包
    println!("\n7. 闭包:");
    let double = |x| x * 2;
    println!("double(5) = {}", double(5));

    let add_closure = |x, y| x + y;
    println!("add_closure(3, 4) = {}", add_closure(3, 4));

    // 8. 早期返回
    println!("\n8. 早期返回:");
    println!("is_even(4) = {}", is_even(4));
    println!("is_even(7) = {}", is_even(7));

    // 9. 递归函数
    println!("\n9. 递归函数:");
    println!("factorial(5) = {}", factorial(5));
    println!("fibonacci(8) = {}", fibonacci(8));

    // 10. 方法语法
    println!("\n10. 方法语法:");
    let mut calculator = Calculator::new();
    calculator.add(10);
    calculator.multiply(3);
    println!("计算器结果: {}", calculator.result());

    println!("\n函数练习完成！");
}

// 简单的问候函数
fn hello_world() {
    println!("Hello, world!");
}

// 带一个参数的函数
fn greet(name: &str) {
    println!("你好, {name}!");
}

// 带多个参数的函数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("测量值是: {value}{unit_label}");
}

// 返回值的函数
fn five() -> i32 {
    5 // 没有分号，这是一个表达式
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// 数学运算函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 高阶函数：接受函数作为参数
fn apply_operation(x: i32, y: i32, op: fn(i32, i32) -> i32) {
    let result = op(x, y);
    println!("操作结果: {result}");
}

// 带条件返回的函数
fn is_even(number: i32) -> bool {
    if number % 2 == 0 {
        return true;
    }
    false
}

// 递归函数
fn factorial(n: u32) -> u32 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 简单的结构体来演示方法
struct Calculator {
    value: i32,
}

impl Calculator {
    fn new() -> Calculator {
        Calculator { value: 0 }
    }

    fn add(&mut self, amount: i32) {
        self.value += amount;
    }

    fn multiply(&mut self, factor: i32) {
        self.value *= factor;
    }

    fn result(&self) -> i32 {
        self.value
    }
}
