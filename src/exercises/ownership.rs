pub fn run() {
    {
        // See https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html
        let mut s = String::from("hello");

        let s1 = &s;
        let len = calculate_length(s1);
        println!("The length of '{s1}' is {len}.");

        let s2 = &mut s;
        change(s2);
        println!("Now the String is {s2}");
    }

    {
        // See https://kaisery.github.io/trpl-zh-cn/ch04-03-slices.html
        let my_string = String::from("hello world");

        // `first_word` 适用于 `String`（的 slice），部分或全部
        let word = first_word(&my_string[0..6]);
        println!("Now word is {word}");
        let word = first_word(&my_string[..]);
        println!("Now word is {word}");
        // `first_word` 也适用于 `String` 的引用，
        // 这等价于整个 `String` 的 slice
        let word = first_word(&my_string);
        println!("Now word is {word}");

        let my_string_literal = "hello world";

        // `first_word` 适用于字符串字面值，部分或全部
        let word = first_word(&my_string_literal[0..6]);
        println!("Now word is {word}, it is from a &str but not String");
        let word = first_word(&my_string_literal[..]);
        println!("Now word is {word}, it is from a &str but not String");

        // 因为字符串字面值已经 **是** 字符串 slice 了，
        // 这也是适用的，无需 slice 语法！
        let word = first_word(my_string_literal);
        println!("Now word is {word}, no need for '&',because &str can be a slice");
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", string");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
