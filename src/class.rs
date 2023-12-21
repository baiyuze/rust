use std::ops;

fn main() {
    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
        o: u8,
    }

    impl Color {
        fn new(red: u8, green: u8, blue: u8, o: u8) -> Color {
            Color {
                red,
                green,
                blue,
                o,
            }
        }
        fn test(red: u8, green: u8, blue: u8, o: u8) -> Color {
            Color {
                red,
                green,
                blue,
                o,
            }
        }

        fn test2(red: u8, green: u8, blue: u8) -> Color {
            Color {
                red,
                green,
                blue,
                o: 255,
            }
        }

        fn hh(&self) -> String {
            return String::from("哈哈,{}");
        }

        fn h2(&self) -> String {
            return String::from("哈哈");
        }
    }
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            if let Message::Write(s) = self {
                println!("{:#?}", s);
            }
            // 在这里定义方法体
        }
    }

    let msg = Message::Write(String::from("hello-->>>"));
    msg.call();

    let color = Color::test(255, 255, 255, 1);
    let color_class = Color::new(255, 255, 255, 1);

    let color2 = Color {
        red: 255,
        green: 255,
        blue: 255,
        o: 1,
    };

    let c3 = color2.h2();
    println!("{:?}, {:#?}, {:#?}", color, c3, color_class);
    fn add<T: ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    let df = add(1, 2);
    println!("{:?}------", df);

    // 特征
    pub trait MyTrait {
        fn get_age(&self) -> String;
    }

    pub struct Person {
        pub title: String,
        pub name: String,
        pub age: u16,
    }
    // 相同特征的共用
    // 不就是相当于类的继承吗
    impl MyTrait for Person {
        fn get_age(&self) -> String {
            // format!("文章{}, 作者是{}", self.title, self.name)
            return self.name.to_string();
        }
    }
    let m = Person {
        title: "人员".to_string(),
        name: "光光".to_string(),
        age: 18,
    };

    println!("{:#?}--", m.get_age());

    // fn test(item: &impl MyTrait) {
    //     format!("{:#?}", item.get_age());
    // }

    // 特征对象

    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    fn draw1(x: Box<dyn Draw>) -> String {
        x.draw()
    }

    draw1(Box::new(11));
}
