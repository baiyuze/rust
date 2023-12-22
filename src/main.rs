mod test;
use std::collections::HashMap;
fn main() {
    println!("你好世界");
    let t = test();
    print!("{}", t);
    tuple();
    struct_fn();
    enum_fn();
    arr_fn();
    process_fn();
    test::hello();
    trait_fn();
    fn_vec();
}

fn test() -> String {
    let _a = 1;
    let _b = 's';
    let _c = true;
    let _d = "HH";
    let mut f = String::from("test");
    let _bb = &_d;
    let g = String::from("测试字符串切片");
    let h = &g[0..6];
    print!("{},1111{}", g, h);
    f.push_str("去你妈的");
    let ddd = &f;
    // f.insert(111, 1);
    for c in "草打哈哈".chars() {
        print!("{}", c);
    }

    // f.replace_range(range, replace_with)//替换的范围
    println!("{},{},{},{},{}", _a, _b, _c, f, ddd);
    // size_of_val(b)
    print!("我是哈哈");
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{},{}", hello, world);
    return f;
}
/*  */

fn tuple() {
    let a = (3.1, 4, 5);
    println!("{:?}", a.0);
}

fn struct_fn() {
    #[derive(Debug)]
    struct User {
        name: String,
        age: u8,
        six: u16,
        active: bool,
    }

    let mut user = User {
        name: String::from("张三"),
        age: 20,
        six: 123,
        active: true,
    };

    let mut _user1 = User {
        name: String::from("张三"),
        ..user
    };
    struct Color(i32, i32, i32);
    // 元组结构体
    let mut _tuple1 = Color(1, 2, 3);
    // user.age = 30;
    print!("{:?}", _user1);
    dbg!(_user1);
}

// 枚举
/**
 * 初始化实例
 */
fn enum_fn() {
    #[derive(Debug)]
    enum IpAddr {
        DD,
    }
    let dd = IpAddr::DD;
    dbg!(dd);
    #[derive(Debug)]

    enum Tcp {
        // TcpBox(u32),
        TLSBox(u64),
    }
    let b = Tcp::TLSBox(11);
    dbg!(b);
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
    }

    let absent_number: Option<String> = Option::Some(String::from("212"));
    dbg!(absent_number);
    // print!("{}", a)
    let _a = [1, 2, 3];
    let _b = [5; 5];
    let a = [1, 2, 3, 4, 5];
    // const BOX_TITLE: String = String::from("哈哈");
    const _MAX_POINTS: u32 = 100_000; // 定义一个不可变的 u32 类型的常量 MAX_POINTS
                                      // let mut index = String::new();
                                      // 读取控制台的输出
                                      // io::stdin()
                                      //     .read_line(&mut index)
                                      //     .expect("Failed to read line");
                                      // let index: usize = index
                                      //     .trim()
                                      //     .parse()
                                      //     .expect("报错");
                                      // .parseInt()
                                      // .expect("Index entered was not a number");

    // let element = a[index];

    // println!("值元素 index {} is: {}", index, element);
}

fn arr_fn() {
    let arr: [String; 8] = std::array::from_fn(|_i| String::from("哈哈"));
    println!("{:#?}", arr);
    let s = &arr[1..3];
    println!("{:#?}", s);
}

fn process_fn() {
    let _condition = true;
    let arr: [String; 8] = std::array::from_fn(|_i| String::from("哈哈"));

    // 循环
    for i in &arr {
        println!("{:#?}111", i);
    }
    for j in 0..arr.len() {
        if j > 2 {
            print!("{:#?}---", arr[j]);
            continue;
        } else {
            break;
        }
    }
    enum Box {
        Test,
        Npx,
    }

    // while arr.len() {

    // }
    // if _condition {
    //     return 9;
    // } else {
    //     8
    // }
    #[derive(Debug)]
    enum Dir {
        XX,
        YY,
        HH,
    }
    let bb = Dir::XX;
    let _cc = match bb {
        Dir::XX => println!("{:#?}", Dir::XX),
        Dir::YY => println!("{:#?}", Dir::YY),
        Dir::HH => println!("{:#?}", Dir::HH),
        _ => print!("呵呵"),
    };

    if let Dir::XX = bb {
        println!("{:#?}", Dir::XX);
    } else {
        println!("{:#?}", Dir::YY);
    }
    let x: u8 = 1;
    let y: u8 = 2;
    let x1 = "data";
    let y1 = "test1";

    fn test(x: u8, y: u8) -> u8 {
        if x > y {
            x
        } else {
            y
        }
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    let bb = test(x, y);
    let longest = longest(x1, y1);
    println!("{}, {}-------", bb, longest);
}

fn trait_fn() {
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    fn draw1(x: Box<dyn Draw>) -> String {
        println!("----u8----: {}", x.draw());
        x.draw()
    }

    draw1(Box::new(11));
    // UI 组件

    // 定义一个结构体
    struct Screen {
        // 里面的元素都是Draw 的特征对象
        components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        fn run(&self) {
            for component in self.components.iter() {
                let b = component.draw();
                println!("==={}===", b);
            }
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) -> String {
            format!("SelectBox: {}", self.width * self.height)
        }
    }

    let bb = String::from("Hello");

    impl Draw for String {
        fn draw(&self) -> String {
            format!("{}", *self)
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(33),
            Box::new(4),
            Box::new(5u8),
            Box::new(bb),
        ],
    };
    screen.run();
}

fn fn_vec() {
    trait IpAddr {
        fn display(&self);
    }
    #[derive(Debug)]
    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0)
        }
    }
    let mut v = Vec::new();
    let mut _v2 = vec![1, 2, 3];
    let mut _v3: Vec<Box<String>> = vec![Box::new(String::from("hello"))];
    let mut _v4 = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V4(String::from("127.0.0.1"))),
    ];
    v.push(1);
    v.push(2);
    v.pop();

    // hashMap

    let mut map = HashMap::new();
    map.insert("div", 1);
}
