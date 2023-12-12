fn main() {
    println!("你好世界");
    let t = test();
    print!("{}",t);
    tuple();
    structFn();
    enumFn();
}

fn test() -> String {
    let _a = 1;
    let _b = 's';
    let _c = true;
    let _d = "HH";
    let mut f = String::from("testaskaksj");
    let _bb = &_d;
    let g = String::from("测试字符串切片");
    let h = &g[0..6];
    print!("{},1111{}", g, h);
    f.push_str("去你妈的");
    let ddd = &f;
    // f.insert(111, 1);
    for c in "草打哈哈".chars() {
        print!("{}", c);
    };
    
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
    let a = (3.1,4,5);
    println!("{:?}",a.0);
}

fn structFn() {
    #[derive(Debug)]
    struct User {
        name: String,
        age: u8,
        six: u16,
        active: bool
    }

    let mut user = User {
        name: String::from("张三"),
        age: 20,
        six: 123,
        active: true
    };
    
    let mut _user1 = User {
        name: String::from("张三"),
        ..user
    };
    struct Color (i32,i32,i32);
    // 元组结构体
    let mut _tuple1 = Color(1,2,3);
    // user.age = 30;
    print!("{:?}", _user1);
    dbg!(_user1);
    
}

// 枚举
fn enumFn() {
    #[derive(Debug)]
    enum IpAddr {
        HH,
        BB,
        DD
    }
    let hh = IpAddr::HH;
    let dd = IpAddr::DD;
    // print!("{:?}", hh);
    dbg!(dd);
    #[derive(Debug)]

    enum Tcp {
        TcpBox(u32),
        TLSBox(u64)
    }
    let b = Tcp::TLSBox(11);
        dbg!(b);
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None
    }
    let a = Value::null;
    let absent_number: Option<String> = Option::None;
        dbg!(absent_number);
        print!("{}", a)


}