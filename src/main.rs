use rand::random;

fn main() {
    let random_number: i32 = random();
    println!("{}", random_number);
}

fn div(x: i32, y: i32) -> i32 {
    x / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_test() {
        assert_eq!(div(10, 3), 3);
    }

    #[test]
    #[should_panic]
    fn div_panic_test() {
        div(2, 0);
    }
}
