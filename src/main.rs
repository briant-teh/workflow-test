fn main() {
    println!("Hello, world!");

    let x = 12;
    let y = 39;

    println!("The sum pf {x} and {y} is {}", sum(x, y))
}

fn sum(x: i64, y: i64) -> i64 {
    x + y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(3, 5), 8)
    }
}
