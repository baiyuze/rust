pub fn hello() {

    let age = Some(10);
    println!("在匹配前，age是{:?}",age);
    if let Some(b) = age {
      print!("匹配出来的b是{}",b)
    }
    println!("在匹配后，age是{:?}",age);
    let b = vec!['1','2','3','4','5'];
    for (index, value) in b.iter().enumerate() {
        println!("index is {}, value is {}", index, value);
    }

    let x = 1;
    match x {
      1 => {},
      2 | 3 => {},
      _ => {}
    }
     struct Port {
      x: u32,
      y: u32,
    }
    let xx = Port { x: 1, y: 2 };
    // let c = Port { x: 1, y: 2 };
    let zz = true;
    // let Port { x: a,y: b } = c;
    // print!("a{},b{}",a,b)

    let ( (e, h), Port { x:i, y: p} ) = ((1,3), Port{ x: 1, y: 2});
    match 1 {
      // Port {x, y} => println!("x is {}, y is {}", x, y),
      4 | 5 | 6 if x == 2 => println!("x is 100"),
      num @(1 | 3) => println!("bb is {}", num),
      _ => println!("x is not 1"),
    }
}

