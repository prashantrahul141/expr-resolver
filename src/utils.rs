const TAYLOR_COEFFICIENTS: [f64; 29] = [
    -0.00000000000000000023,
    0.00000000000000000141,
    0.00000000000000000119,
    -0.00000000000000011813,
    0.00000000000000122678,
    -0.00000000000000534812,
    -0.00000000000002058326,
    0.00000000000051003703,
    -0.00000000000369680562,
    0.00000000000778226344,
    0.00000000010434267117,
    -0.00000000118127457049,
    0.00000000500200764447,
    0.00000000611609510448,
    -0.00000020563384169776,
    0.00000113302723198170,
    -0.00000125049348214267,
    -0.00002013485478078824,
    0.00012805028238811619,
    -0.00021524167411495097,
    -0.00116516759185906511,
    0.00721894324666309954,
    -0.00962197152787697356,
    -0.04219773455554433675,
    0.16653861138229148950,
    -0.04200263503409523553,
    -0.65587807152025388108,
    0.57721566490153286061,
    1.00000000000000000000,
];

const INITIAL_SUM: f64 = 0.00000000000000000002;

// Calculates gamma of a f64.
// https://en.wikipedia.org/wiki/Gamma_function
fn gamma(x: f64) -> f64 {
    TAYLOR_COEFFICIENTS
        .iter()
        .fold(INITIAL_SUM, |sum, coefficient| {
            sum * (x - 1.0) + coefficient
        })
        .recip()
}

// Calculates simple factorial of a f64.
fn simple_factorial(x: f64) -> f64 {
    if x <= 1.0 {
        return 1.0;
    }

    simple_factorial(x - 1.0) * x
}

/// Wrapper function to calculate factorial of a function.
/// Returns simple factorial of x if the number can be converted to integer without loss.
/// Returns gamma of x if the number cannot be converted to integer without loss.
/// # Arguments
/// * x - the number to find factorial of.
pub fn factorial(x: f64) -> f64 {
    log::trace!("getting factorial of {}", x);

    // for integers.
    if x.fract() == 0.0 {
        return simple_factorial(x);
    }

    // for non integers.
    gamma(x + 1 as f64)
}
