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

pub fn all_construct(target: &str, bank: &[&str]) -> Vec<Vec<String>> {

    if target.is_empty() {
        return vec![vec![]];
    }

    let mut result = Vec::new();

    for word in bank {
        if target.starts_with(word) {
            let suffix = &target[word.len()..];
            let suffix_ways = all_construct(suffix, bank);

            //combine successful combinations
            for mut way in suffix_ways {
                way.insert(0, word.to_string());
                result.push(way);
            }
        }
    }

    return result;
}

pub fn can_construct(target: &str, vector: &Vec<&str>) -> bool {
    if target.is_empty() {
        return true;
    }

    for word in vector {
        if target.starts_with(word) {
            let suffix = &target[word.len()..];
            if can_construct(suffix, vector) {
                return true;
            }
        }
    }

    return false;
}

pub fn best_sum(target: i64, v: &Vec<i64>) -> Option<Vec<i64>> {
    if target == 0 {
        return Some(vec![]);
    }
    if target < 0 {
        return None;
    }

    let mut shortest: Option<Vec<i64>> = None;

    for num in v {
        if let Some(mut res) = best_sum(target - num, &v) {
            res.push(*num);
            if shortest.is_none() || res.len() < shortest.as_ref().unwrap().len() {
                shortest = Some(res);
            }
        }
    }

    return shortest;
}

pub fn how_sum(target: i64, v: &Vec<i64>) -> Option<Vec<i64>> {
    fn how_sum_memo(
        target: i64,
        v: &Vec<i64>,
        memo: &mut HashMap<i64, Option<Vec<i64>>>,
    ) -> Option<Vec<i64>> {
        if memo.contains_key(&target) {
            return memo.get(&target).unwrap().clone();
        }

        if target == 0 {
            return Some(vec![]);
        }
        if target < 0 {
            return None;
        }

        for num in v {
            if let Some(mut res) = how_sum_memo(target - num, &v, memo) {
                res.push(*num);
                memo.insert(target, Some(res.clone()));
                return Some(res);
            }
        }

        memo.insert(target, None);
        return None;
    }

    let mut memo = HashMap::new();
    how_sum_memo(target, v, &mut memo)
}

pub fn can_sum(m: i64, v: &Vec<i64>) -> bool {
    fn can_sum_memo(m: i64, v: &Vec<i64>, memo: &mut HashMap<i64, bool>) -> bool {
        if memo.contains_key(&m) {
            return *memo.get(&m).unwrap();
        }

        if m == 0 {
            return true;
        }
        if m < 0 {
            return false;
        }

        for num in v {
            let result = m - num;
            if can_sum_memo(result, &v, memo) {
                memo.insert(m, true);
                return true;
            }
        }

        memo.insert(m, false);
        return false;
    }

    let mut memo = HashMap::new();
    can_sum_memo(m, v, &mut memo)
}

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
    fn test_all_construct() {
        assert_eq!(
            vec![vec!["purp", "le"], vec!["p", "ur", "p", "le"]],
            all_construct("purple", &["purp", "p", "ur", "le", "purpl"].to_vec())
        );
    }

    #[test]
    fn test_can_construct() {
        assert_eq!(true, can_construct("", &["ska", "rd", "te"].to_vec()));
        assert_eq!(
            true,
            can_construct(
                "skate",
                &["ska", "rd", "te", "t", "ka", "sk", "boar"].to_vec()
            )
        );
        assert_eq!(false, can_construct("mouse", &["mo", "se"].to_vec()));
        assert_eq!(
            false,
            can_construct(
                "skateboard",
                &["bo", "rd", "ate", "t", "ska", "sk", "boar"].to_vec()
            )
        );
    }
    #[test]
    fn test_best_sum() {
        assert_eq!(Some(vec![5, 3]), best_sum(8, &[2, 3, 5].to_vec()));
        assert_eq!(Some(vec![]), best_sum(0, &[5, 3, 4, 7].to_vec()));
        assert_eq!(None, best_sum(7, &[2, 4].to_vec()));
        assert_eq!(Some(vec![4, 4]), best_sum(8, &[1, 4, 5].to_vec()));
        // assert_eq!(None, best_sum(300, &[7, 14].to_vec()));
    }

    #[test]
    fn test_how_sum() {
        assert_eq!(Some(vec![4, 3]), how_sum(7, &[5, 3, 4, 7].to_vec()));
        assert_eq!(Some(vec![]), how_sum(0, &[5, 3, 4, 7].to_vec()));
        assert_eq!(None, how_sum(7, &[2, 4].to_vec()));
        assert_eq!(Some(vec![1]), how_sum(1, &[1].to_vec()));
        assert_eq!(None, how_sum(300, &[7, 14].to_vec()));
    }

    #[test]
    fn test_can_sum() {
        assert_eq!(true, can_sum(7, &[5, 3, 4, 7].to_vec()));
        assert_eq!(true, can_sum(0, &[5, 3, 4, 7].to_vec()));
        assert_eq!(false, can_sum(7, &[2, 4].to_vec()));
        assert_eq!(true, can_sum(1, &[1].to_vec()));
        assert_eq!(false, can_sum(300, &[7, 14].to_vec()));
    }

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
