// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }

    let mut fac = n;
    let mut x = n - 1;
    while x > 1 {
        fac *= x;
        x -= 1;
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
