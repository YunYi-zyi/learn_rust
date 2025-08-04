pub fn run() {
    let mut v1: Vec<i32> = Vec::new(); // 当向量不再声明时初始化的话，应当显示规范类型
    #[allow(clippy::useless_vec)]
    let v2 = vec![1, 2, 3]; // 这种情况就不用标注类型

    v1.push(0);
    v1.push(1);

    let third: &i32 = &v2[2]; // 访问可以通过下标
    println!("The third of value is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(i) => println!("The third of value is {i}"),
        None => println!("The index is larger than the vector"),
    };

    // 可使用循环来遍历向量
    for i in &v1 {
        print!("{i} ");
    }

    // 或者通过循环遍历可变引用来修改向量
    println!("Add 50 to each value");
    for i in &mut v1 {
        *i += 50;
        print!("{i} ");
    }

    // 如果想要让向量储存多个类型的值，可使用集合
    // let v3 = vec![v::Text(String::from("Hello")), v::Int(1), v::Float(1.23)];
}

/*
enum v {
    Text(String),
    Int(i32),
    Float(f32),
}
*/
