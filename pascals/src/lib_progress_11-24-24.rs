pub fn pascals(n: u64) -> Vec<u64> {
    // Dynamic Programming solution?
    // Start at 0

    return 1..=n.map(|_: u64| -> { 
        1u64 
    });

    let pascal_rows = vec![
        vec![1]
    ];

    for i in 1..=n {
        // Calculate the current row, based on the (i-1)th row
        // and put it in the pascal_rows
        let row = vec![1,1];

        // Use pascal_rows[i-1] to find this row.
        // row ???

        pascal_rows.push(row);
    }

    pascal_rows[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        let result = pascals(0);
        assert_eq!(result, [1]);
    }

    #[test]
    fn it_works_1() {
        let result = pascals(1);
        assert_eq!(result, [1, 1]);
    }

    #[test]
    fn it_works_2() {
        let result = pascals(2);
        assert_eq!(result, [1, 2, 1]);
    }

    
    #[test]
    fn it_works_3() {
        let result = pascals(3);
        assert_eq!(result, [1, 3, 3, 1]);
    }

    #[test]
    fn it_works_4() {
        let result = pascals(4);
        assert_eq!(result, [1, 4, 6, 4, 1]);
    }

    #[test]
    fn it_works_5() {
        let result = pascals(5);
        assert_eq!(result, [1, 5, 10, 10, 5, 1]);
    }
}

