fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else if b > a {
        b
    } else {
        a
    }
    // or with match expression
    // match a.cmp(&b) {
    //     std::cmp::Ordering::Less => b,
    //     std::cmp::Ordering::Greater => a,
    //     _ => a,
    // }
}

fn main() {
    // You can optionally experiment here.
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
