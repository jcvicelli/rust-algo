//see problem as tree
// think the smalles imput
// implement the tree using recursion (bigger to smaller)
// define base case {leaf of the tree}
// test
// make efficient - memoization
//  -   start a empty map
//  -   add map to the list of params
//  -   check if result already in map or
//  -   insert result in map

use std::collections::HashMap;

pub fn grid_traveler(m: u64, n: u64) -> u64 {
    // 2,3 -> 3
    //[1,1] -> 1
    fn grid_traveler_memo(m: u64, n: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
        if let Some(&result) = memo.get(&(m, n)) {
            return result;
        }

        if m == 1 && n == 1 {
            return 1;
        } else if m == 0 || n == 0 {
            return 0;
        }

        let result = grid_traveler_memo(m - 1, n, memo) + grid_traveler_memo(m, n - 1, memo);
        memo.insert((m, n), result);
        result
    }

    //empty map for memoization
    let mut memo = HashMap::new();

    grid_traveler_memo(m, n, &mut memo)
}

pub fn fib(x: u64) -> u64 {
    // fib(6) = fib(5) + fib(4)
    fn fib_memo(x: u64, memo: &mut HashMap<u64, u64>) -> u64 {
        if let Some(&result) = memo.get(&x) {
            return result;
        }

        let result = match x {
            0 => 0,
            1 => 1,
            _ => fib_memo(x - 1, memo) + fib_memo(x - 2, memo),
        };

        memo.insert(x, result);
        result
    }
    //empty map for memoization
    let mut memo = HashMap::new();

    fib_memo(x, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_traveler() {
        assert_eq!(0, grid_traveler(2, 0));
        assert_eq!(1, grid_traveler(1, 1));
        assert_eq!(0, grid_traveler(1, 0));
        assert_eq!(3, grid_traveler(2, 3));
        assert_eq!(6, grid_traveler(3, 3));
        assert_eq!(2333606220, grid_traveler(18, 18));
    }

    #[test]
    fn test_fib() {
        assert_eq!(8, fib(6));
        assert_eq!(13, fib(7));
        assert_eq!(21, fib(8));
        assert_eq!(12586269025, fib(50));
    }
}
