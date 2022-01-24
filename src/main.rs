fn main() {
    // 变量绑定
    println!("// 变量绑定");
    let x = 5;
    let x2 = 'Z';
    let y = "Z";

    println!("变量绑定：x: {}, x2: {}, y: {} ", x, x2, y);

    // 变量解构
    println!("// 变量解构");
    let (foo, bar): (bool, char) = (false, 'Y');

    println!("复杂变量解构：foo: {:?}, bar: {} ", foo, bar);

    // 常量（常量不允许修改）
    println!("// 常量（常量不允许修改");
    const CONST_STR: &str = "我是常量";
    println!("常量：CONST_STR: {} ", CONST_STR);

    // 可变变量，目的是：告诉后人，我这个变量后面会发生改变
    println!("// 可变变量，目的是：告诉后人，我这个变量后面会发生改变");
    let mut mut_args = "mut args";
    println!("常量 mut 前：CONST_STR: {} ", mut_args);
    mut_args = "it mut";
    println!("常量 mut 后：CONST_STR: {} ", mut_args);

    // 函数返回值
    fn test() -> i32{
        let (a, b) = (3, 4);
        let d = 0;
        let c = 4;

        return d + c;
    }

    let t: i32= test();

    println!("{}", t);

    // 所有权
    let obj = String::from("张三");

    obj.clone();

    // 这里大致懂所有权是啥意思了，所有权意味着转移，而不是引用

    // rust 是不用关心 GC 的，保持变量的所有权，在编译阶段就回收掉

    // 引用

    let x2 = 45;

    let y2 = &x2;

    println!("x2: {{{}}}, y2: {}", x2, *y2);



}
