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


}
