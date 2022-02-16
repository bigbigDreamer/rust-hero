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

    // 字符串

    // 创建一个全新的字符串
    let str = String::from("Hello World");

    for s in str.chars() {
        println!("单个字符s: {}", s)
    }

    // 原组
    let (a,b): (bool, i32) = (true, 12);

    println!("a: {}, b: {}", a, b);

    // 结构体
    struct Animal {
        name: String,
        animal_type: String,
    }

    let dog = Animal {
        name: String::from("Lily"),
        animal_type: String::from("Dog"),
    };

    println!("Animal.name==> {:?}", dog.name);
    println!("Animal.type==> {:?}", dog.animal_type);

    // 结构体原组
    struct Color ();

    let i: u32 = 12;

    println!("{}", i);

    // 复习
    // 所有权
    let x1 = String::from("所有权");

    // 至此，x1 的内存在下面这段代码执行完已经释放
    let x2 = x1;

    // 此段代码会报错
    // println!("x1: {}, x2: {}", x1, x2)

    // 同一时刻只能拥有一个变量的可变引用，或者多个不可变引用

    // 枚举
    enum PokerSuit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
    }

    // let test_m1 = PokerSuit::Clubs("sdd");

    // println!("枚举类型：{}", test_m1)

    // 数组
    const ARR: [i32; 4] = [1, 2, 3, 3];

    for a in 0..ARR.len()  {
        println!("ARR 的遍历结果：{}", ARR[a]);
    }


    let test_a = 13;
    let test_b = 25;

    if test_a < test_b {
        println!("他比我大")
    }

    fn test_loop() {
        let mut a = 1;

        loop {
            a+=1;

            println!("{}", a);

            if a==5 {
                println!("我要跳出循环");
                break;
            }
        }
    }

    test_loop();




}
