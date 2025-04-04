#[derive(Debug)]
struct Calculator {
    value: i32,
    operator: Message,
    numbers: Vec<i32>,
}

#[derive(Debug)]
enum Message {
    Sub,
    Add,
    Equal,
    Zero,
    One,
    Two,
    None,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            value: 0,
            operator: Message::None,
            numbers: Vec::new(),
        }
    }

    fn add(&mut self, num: i32) {
        self.value += num;
    }

    fn push_int(&mut self, num: i32) {
        self.numbers.push(num);
    }
}

fn main() {
    let mut my_val = Calculator::new();

    my_val.push_int(5);
    dbg!(&my_val);
    println!("{:?}", my_val);

    my_val.add(3);
    dbg!(&my_val);
    println!("{:?}", my_val);
}
