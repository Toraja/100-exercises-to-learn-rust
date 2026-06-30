fn intro() -> &'static str {
    // fix me 👇
    "I'm ready to learn about traints!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to learn about traits!");
    }
}
