// Generate the nth Fibonacci number.

fn main() {
    // slow
    let result_value: u128 = fibbo_recursive(0);
    println!("fibbo_recursive {}", result_value);

    //fast
    let result_value: u128 = fibbo_iterative(99);
    println!("fibbo_iterative {}", result_value)
}

fn fibbo_iterative(n: u128) -> u128 {
    let mut prev_prev: u128 = 0;
    let mut prev: u128 = 1;
    let mut current_number: u128 = 0;

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        for _ in 2..=n {
            current_number = prev_prev + prev;
            prev_prev = prev;
            prev = current_number;
        }
        return current_number;
    }
}

fn fibbo_recursive(n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibbo_recursive(n - 1) + fibbo_recursive(n - 2);
    }
}
