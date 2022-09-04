use rand::Rng;
use std::cmp::Ordering;
use std::io; // prelude

const _MAX_POINTS: u32 = 1000_000;

fn main() {
    println!("Hello, world!");
    // test01();
    // test02();
    // test03();
    // test04();
    // test05();
    // test06();
    // test07();
    // test08();
    // test09();
    // test10();
    // test11();
    // test12();
    // test13();
    // test14();
    test15();
}

// 所有权与函数
// https://www.bilibili.com/video/BV1hp4y1k7SV?p=17&spm_id_from=pageDriver&vd_source=4e596b40d4a4d41673da5c9dd4195a21
fn test15() {}

// https://www.bilibili.com/video/BV1hp4y1k7SV?p=15&spm_id_from=pageDriver&vd_source=4e596b40d4a4d41673da5c9dd4195a21
fn _test14() {
    let _str = "hello, world".to_string();
    // String
    // &str
    let mut str = String::from("hello");
    str.push_str(", world");

    println!("{}", str);
    // drop()
    // move
    let str1 = str;
    println!("-> {}", str1);
    // println!("-> {}", str); // error borrow of moved value: "str" value borrowed here after move
}

fn _test13() {
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        print!("-> {}", arr[index]);
        index += 1;
    }
    println!();

    // index: i32
    for index in arr {
        print!("-> {}", index);
    }
    println!();

    // index: &i32
    for index in arr.iter() {
        print!("-> {}", index);
    }
    println!();

    for number in (1..4).rev() {
        print!("-> {}", number);
    }
    println!();
}

fn _test12() {
    let mut counter = 10;
    while counter != 0 {
        println!("-> {}", counter);
        counter -= 1;
    }
    println!("end");
}

fn _test11() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // support to return loop with value
            break counter * 2;
        }
    };
    println!("result -> {}", result);
}

fn _test10() {
    // match
    let condition = true;
    let number = match condition {
        true => "true",
        false => "false",
    };
    println!("number -> {}", number);
}

fn _test09() {
    let number = 6;
    // else if按照顺序判断
    if number % 4 == 0 {
        println!("-> 4");
    } else if number % 3 == 0 {
        println!("-> 3");
    } else if number % 2 == 0 {
        println!("-> 2");
    } else {
        println!("other");
    }
}

fn _test08() {
    let number = 9;
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }
}

fn _test07() {
    fn five(x: i32) -> i32 {
        x + 5
    }
    let x = five(6);
    println!("-> {}", x);
}

fn _test06() {
    // 语句和表达式
    let _x = 5;
    let y = {
        let x = 1;
        x + 3
    };
    println!("y -> {}", y);
}

fn _test05() {
    println!("start call a new function");
    fn another_function(x: i32, y: i32) {
        println!("call me succeed");
        println!("-> {} {}", x, y);
    }
    another_function(5, 6);
}

fn _test04() {
    let _guess: u32 = "42".parse().expect("not a number");

    // 整数类型
    let _x = 2;
    // 浮点类型
    let _x = 2.0;
    // 布尔类型
    let _x = true;
    // 字符类型
    let _x = 'a';
    println!("-> {}", _x); // 占用四个字节大小
                           // tuple
    let tuple = (0, 2.0, true, 'a');
    let (x, y, z, o) = tuple; // 解构
    println!(" {} {} {} {}", x, y, z, o);
    // 按照索引访问
    println!(" {}, {}, {}, {}", tuple.0, tuple.1, tuple.2, tuple.3);
    // 数组 stack
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];
    // println!("{}", _a[16]);
    for i in _a {
        print!("{} ", i);
    }
    println!("");
}

fn _test03() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);

    {
        let _x = 5;
        let _x = _x + 1; // 同名隐藏 shadowing
        let _x = "string";
    }
}

// guess number
fn _test02() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("secret number is {}", secret_number);

    loop {
        let mut guess_number = String::new();

        // io::stdin().read_line(&mut guess_number).expect("error read line");
        match io::stdin().read_line(&mut guess_number) {
            Ok(_) => {
                // println!("read {} bytes", n);
            }
            Err(e) => {
                println!("error {}", e);
                return;
            }
        }

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(res) => res,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small");
                continue;
            }
            Ordering::Greater => {
                println!("too big");
                continue;
            }
            Ordering::Equal => {
                println!("you will");
                break;
            }
        }
    }
}

// guess number
fn _test01() {
    println!("please input a number");
    let mut guess_number = String::new();

    // io::stdin().read_line(&mut guess_number).expect("error read line");
    match io::stdin().read_line(&mut guess_number) {
        Ok(n) => {
            println!("read {} bytes", n);
        }
        Err(e) => {
            println!("error {}", e);
            return;
        }
    }

    println!("output -> {}", guess_number);
}
