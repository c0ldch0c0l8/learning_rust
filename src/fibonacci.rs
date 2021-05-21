
pub fn nth_fibonacci(nth: u32) -> u32 {
    let mut n = 1;
    let mut n_minus_one = 1; // aka fib(1)
    let mut n_minus_two = 0; // ala fib(0)

    if nth == 0 {
        n_minus_two
        
    } else if nth == 1 {
        n_minus_one

    } else {
        for _ in 2..nth {
            n_minus_two = n_minus_one;
            n_minus_one = n;
            n = n_minus_one + n_minus_two;
        }

        n
    }
}