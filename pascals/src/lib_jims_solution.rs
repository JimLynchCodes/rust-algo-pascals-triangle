pub fn pascals(n: u64) -> Vec<u64> {
    
    if n == 0 {
        return vec![1];
    };

    let mut current_row = vec![1];
    
    for _ in 0..n {
        
        let mut next_row: Vec<u64> = vec![];

        next_row.push(1);

        for i in 0..(current_row.len() - 1) {

            let left = current_row.get(i).unwrap();
            let right = current_row.get(i+1).unwrap();

            next_row.push(left + right);
        }

        next_row.push(1);

        // More efficient because it avoids reallocating memory for current_row in each iteration
        std::mem::swap(&mut current_row, &mut next_row);

        // Works fine since it effectively discards the old current_row by overwriting it.
        // current_row = next_row;

    }

    current_row
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    
    #[rstest]
    #[case(0, vec![1])]
    #[case(1, vec![1, 1])]
    #[case(2, vec![1, 2, 1])]
    #[case(3, vec![1, 3, 3, 1])]
    #[case(4, vec![1, 4, 6, 4, 1])]
    fn it_works(#[case] n: u64, #[case] expected: Vec<u64>) {
            assert_eq!(pascals(n), expected);
    }
}
