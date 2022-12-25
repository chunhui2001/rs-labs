// 悬空指针
fn main() {
    let r;
    {
        let v = vec![1, 2, 3];
        r = &v[1];
        println!("{}", r);
    }
}
