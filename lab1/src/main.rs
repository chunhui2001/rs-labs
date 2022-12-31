struct MyStruct<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn main() {
    let x = 10;
    let r;

    {
        let y = 20;
        {
            let s = MyStruct { x: &x, y: &y };
            r = s.x;
            println!("{}, {}", s.x, s.y);
        }
        println!("{}", r);
    }
}
