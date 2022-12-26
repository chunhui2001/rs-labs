use std::str::FromStr;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn parse_number<T: FromStr>(s: &str) -> Option<T> {
    match T::from_str(s) {
        Ok(l) => Some(l),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn it_parse_number() {
        let i: f64 = parse_number("3").expect("error parseing u32");
        assert_eq!(3.0, i);
    }

    #[test]
    fn max1() {
        println!("dddd {}", std::i32::MAX);
        assert_eq!(3.0, 3.0);
    }
}
