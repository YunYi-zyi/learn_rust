// 练习模块
pub mod conditionals;
pub mod enums;
pub mod functions;
pub mod loops;
pub mod ownership;
pub mod strings;
pub mod structlearn;
pub mod variables;

// 导出所有练习的运行函数
pub use conditionals::run as run_conditionals;
pub use enums::run as run_enums;
pub use functions::run as run_functions;
pub use loops::run as run_loops;
pub use ownership::run as run_ownership;
pub use strings::run as run_strings;
pub use structlearn::run as run_struct;
pub use variables::run as run_variables;

// 练习列表，用于菜单显示
pub const EXERCISES: &[(&str, &str)] = &[
    ("1", "变量和常量 (Variables and Constants)"),
    ("2", "字符串操作 (String Operations)"),
    ("3", "循环结构 (Loops)"),
    ("4", "函数定义 (Functions)"),
    ("5", "条件语句 (Conditionals)"),
    ("6", "所有权（Ownership）"),
    ("7", "结构体(struct)"),
    ("8", "枚举 (Enums)"),
];

// 根据选择运行对应的练习
pub fn run_exercise(choice: &str) -> bool {
    match choice {
        "1" => {
            run_variables();
            true
        }
        "2" => {
            run_strings();
            true
        }
        "3" => {
            run_loops();
            true
        }
        "4" => {
            run_functions();
            true
        }
        "5" => {
            run_conditionals();
            true
        }
        "6" => {
            run_ownership();
            true
        }
        "7" => {
            run_struct();
            true
        }
        "8" => {
            run_enums();
            true
        }
        _ => false,
    }
}
