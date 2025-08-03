pub fn run() {
    let one = Rmb::Green;
    let hundred = Rmb::Red;

    let mut value = value_in_colors(one);
    println!("绿色人民币价值：{value}");
    value = value_in_colors(hundred);
    println!("红色人民币的价值：{value}");

    let five = Some(5);
    println!("Some(5): {}", five.unwrap());
    let six = plus_one(five);
    println!("Some(5+1): {}", six.unwrap());
    let none = plus_one(None);
    //none is None
}

#[allow(dead_code)]
enum Rmb {
    Red,
    Green,
    Blue,
    Brown,
    Purple,
}

fn value_in_colors(rmb: Rmb) -> u8 {
    //必须包含所有可能性
    match rmb {
        Rmb::Red => 100,
        Rmb::Green => {
            println!("Least!");
            1
        }
        Rmb::Purple => 5,
        _ => 0,
    }
}

#[allow(clippy::manual_map)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    // if let Some(i) = x { Some(i + 1) } else { None }
    /*
    let Some(i) = x else { return None; };
    Some(i + 1)
    */
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
