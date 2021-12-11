use lehmer::Lehmer;

/// computes the factorial of a number
fn factorial(n: usize) -> usize
{
    let mut product = 1;
    for i in 2..(n + 1)
    {
        product *= i;
    }
    product
}

/// returns the number of permutations possible with the given number of elements
pub fn nb_permutations(nb_elements: usize) -> usize
{
    factorial(nb_elements)
}

/// turns a permutation into a decimal number
pub fn decimal_from_permutation(partial_permutation: &[u8]) -> usize
{
    Lehmer::from_permutation(partial_permutation).to_decimal()
}

/// turns a decimal number into a partial permutation
pub fn permutation_from_decimal(decimal: usize, nb_elements_total: usize) -> Vec<u8>
{
    Lehmer::from_decimal(decimal, nb_elements_total).to_permutation()
}

/// returns the number of permutations possible with nb_elements
/// sampled from a larger nb_elements_total
/// (n pick k computation)
pub fn nb_partial_permutations(nb_elements: usize, nb_elements_total: usize) -> usize
{
    factorial(nb_elements_total) / factorial(nb_elements_total - nb_elements)
}

/// turns a partial permutation into a decimal number
pub fn decimal_from_partial_permutation(partial_permutation: &[u8], nb_elements_total: usize) -> usize
{
    // computes the lehmer code for the permutation (same as non-partial permutation)
    let code = Lehmer::from_permutation(partial_permutation).code;

    // converts the code into a decimal number
    // using an algorithm adapted to partial_permutations
    // see `Indexing Partial Permutations`:
    // https://medium.com/@benjamin.botto/sequentially-indexing-permutations-a-linear-algorithm-for-computing-lexicographic-rank-a22220ffd6e3
    let nb_elements = partial_permutation.len();
    let denom_base = factorial(nb_elements_total - nb_elements);
    let mut product = 1;
    let mut decimal = 0;
    for (i, digit) in code.into_iter().rev().map(|d| d as usize).enumerate().skip(1)
    {
        product *= i;
        // base = (nb_elements_total - 1 - i) pick (nb_elements - 1 - i)
        //      = (nb_elements_total - 1 - i)! / (nb_elements_total - nb_elements)
        let base = product / denom_base;
        decimal += digit * base;
    }
    decimal
}

/// turns a decimal number into a partial permutation
pub fn partial_permutation_from_decimal(decimal: usize,
                                        nb_elements: usize,
                                        nb_elements_total: usize)
                                        -> Vec<u8>
{
    // turns decimal into code
    // https://medium.com/@benjamin.botto/sequentially-indexing-permutations-a-linear-algorithm-for-computing-lexicographic-rank-a22220ffd6e3
    let mut code: Vec<u8> = (0..nb_elements).map(|_| 0).collect();
    let denom_base = factorial(nb_elements_total - nb_elements);
    let mut product = 1;
    let mut iteration = 1;
    for index in (0..(nb_elements - 1)).rev()
    {
        product *= iteration;
        iteration += 1;
        // base = (nb_elements_total - 1 - i) pick (nb_elements - 1 - i)
        //      = (nb_elements_total - 1 - i)! / (nb_elements_total - nb_elements)
        let base = product / denom_base;
        let divisor = decimal / base;
        let remainder = divisor % iteration;
        code[index] = remainder as u8;
    }

    // turns code into actual permutation
    let mut sequence: Vec<u8> = (0..(nb_elements_total as u8)).collect();
    for d in &mut code
    {
        *d = sequence.remove(*d as usize);
    }

    // returns the permutation
    code
}
