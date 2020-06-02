// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return factorial of num
    // Do not use:
    // - return
    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - additional variables
    // For the most fun don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // let mut res = num;
    // let mut temp_num = num;
    // while temp_num > 1 {
    //     res = res * (temp_num -1);
    //     temp_num -= 1;
    // }
    // res

    // But you can also use ranges and iterators to solve this in rust.
    (1..=num).into_iter().fold(1,|acc,x| acc*x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
