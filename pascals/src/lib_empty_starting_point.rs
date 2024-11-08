pub fn pascals(n: u64) -> Vec<u64> {
    [].into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = pascals(0);
        assert_eq!(result, []);
    }
}
