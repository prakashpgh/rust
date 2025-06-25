pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    
    fn it_works_fails() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }
}

/*
cargo test
    assert_eq!
    assert_ne!

cargo test -- --test-threads=1

*/