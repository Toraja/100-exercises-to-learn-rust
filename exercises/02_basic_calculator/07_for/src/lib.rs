// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    let mut fac = 2;
    for i in 3..=n {
        println!("i: {}", i);
        fac *= i;
    }
    fac
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
