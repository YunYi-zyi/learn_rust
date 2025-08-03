use std::fmt;

pub fn run() {
    let user1 = create_user(String::from("user1"), String::from("user1@test.com"));
    let user2 = User {
        email: "user2@test.com".to_string(),
        ..user1
    }; //此时user1中的username和email转移了所有权，因此user1不能够再使用，除非再次定义这两个值
    println!("{user2}");

    let a = Color(0, 0, 0);
    println!("{a}");

    println!("现在展示计算长方体面积的示例");
    let r = Rectangle {
        height: 30,
        width: 20,
    };
    println!("{r}");
    let a = area(&r); //使用函数
    println!("长方形的面积为：{a}（使用函数）");
    println!("长方形的面积为：{}（使用方法）", r.area_self());
}

struct User {
    //正常结构体
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
    //结构体并不能直接输出，而需要使用Display Trait来实现自定义输出
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User: {} ({}), Active: {}, Sign in count: {}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}

struct Color(i32, i32, i32); //元组结构体，每个值没有名字，使用下标表示

impl fmt::Display for Color {
    //同理，需要自定义Display Trait
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x:{}, y:{}, z:{}", self.0, self.1, self.2)
    }
}

fn create_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// 以下是为了展示结构体的使用方式，定义了一个计算长方形面积的结构体和函数
struct Rectangle {
    width: u32,
    height: u32,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "height: {}, width: {}", self.height, self.width)
    }
}

impl Rectangle {
    fn area_self(&self) -> u32 {
        self.height * self.width
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
