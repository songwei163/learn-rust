use std::{io}; // prelude
use rand::Rng;
use std::cmp::Ordering;

const _MAX_POINTS: u32 = 1000_000;

fn main() {
    println!("Hello, world!");
    // test01();
    // test02();
    // test03();
    test04();
}

fn test04() {
    let _guess: u32 = "42".parse().expect("not a number");

    // 整数类型
    let _x = 2;
    // 浮点类型
    let _x = 2.0;
    // 布尔类型
    let _x = true;
    // 字符类型
    let _x = 'a'; 
    println!("-> {}", _x);// 占用四个字节大小
    // tuple
    let tuple = (0, 2.0, true, 'a');
    let (x, y, z, o) = tuple; // 解构
    println!(" {} {} {} {}", x, y, z, o);
    // 按照索引访问
    println!(" {}, {}, {}, {}", tuple.0, tuple.1, tuple.2, tuple.3);
    // 数组 stack
    let _a: [i32;5] = [1,2,3,4,5];
    let _a = [3;5];
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
                return ;
            }
        }

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(res) => {
                res
            }
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
            return ;
        }
    }

    println!("output -> {}", guess_number);
}
