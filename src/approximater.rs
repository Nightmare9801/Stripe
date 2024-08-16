use bigdecimal::BigDecimal;

/// The function calculates an approximation of the mathematical constant e using factorials in Rust.
pub(crate) fn approximate() {
    // Initialize e to 1, representing the initial value of Euler's number (e)
    let mut e: BigDecimal = BigDecimal::from(1);
    
    // Initialize factorial to 1, which will be used to calculate factorial values
    let mut factorial: BigDecimal = BigDecimal::from(1);
    
    // Set the upper bound for the loop (70 in this case)
    let upper_bound = 70;

    // Iterate from 1 up to upper_bound (exclusive)
    for i in 1..upper_bound {
        // Calculate the factorial for the current iteration
        // factorial *= (upper_bound - i) represents (n - i)!
        factorial *= BigDecimal::from(upper_bound - i);

        // Add the current factorial value to e
        e += factorial.clone();
    }
    
    // Divide the accumulated value of e by the final factorial value
    e = e / factorial;

    // Print the approximate value of Euler's number (e)
    println!("{}", e);
}

