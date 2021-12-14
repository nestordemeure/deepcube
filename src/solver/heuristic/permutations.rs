/*
One could use lehmer indices but recoding everything from scratch felt easier to deal with partial permutations correctly
here is a recurcive version of the following algorithm for ease of understanding

encoding([]) = 0
encoding([x0]) = x0 = 0 + 1*encoding([]) (0 is the only possibility for x0)
encoding([x0,x1]) = x1 + 2*encoding([x0])
encoding([x0,x1,x2]) = x2 + 3*encoding([x1,x0]) = x2 + 3*(x1 + 2*x0)) = x2 + 3*x1 + 3*2*x0
encoding(n element) is between 0 and n! excluded
(before going to the recursive step, one should be careful to push all values below the one we just used)

using a modulo one can easily reverse the operation:
decoding(x, nb_elements) = [decoding(x/nb_elements, nb_elements-1).., x%nb_elements]
*/

/// returns the number of permutations possible with the given number of elements
/// NOTE: this function should be const
pub fn nb_permutations(nb_elements: usize) -> usize
{
    // factorial(nb_elements)
    let mut product = 1;
    for i in 2..=nb_elements
    {
        product *= i;
    }
    product
}

/// turns a permutation into a decimal number
pub fn decimal_from_permutation<const NB_ELEMENTS: usize>(permutation: &[u8; NB_ELEMENTS]) -> usize
{
    // the result we will return
    let mut result = 0;
    // represents the indices shifted after each value removal
    let mut shifted_indices = [0; NB_ELEMENTS];
    for i in 0..NB_ELEMENTS
    {
        shifted_indices[i] = i;
    }
    // how many elements are left to process
    let mut nb_elements_left = NB_ELEMENTS;
    // the base by which we are multiplying
    let mut base = 1;

    // adds elements one after the other
    // we skip the last elements as ot will necesearily be 0
    for i in permutation.iter().take(NB_ELEMENTS - 1).map(|i| *i as usize)
    {
        // gets shifted index
        let shifted_i = shifted_indices[i];
        // updates shift
        for j in (i + 1)..NB_ELEMENTS
        {
            shifted_indices[j] -= 1;
        }
        // updates result
        result += base * shifted_i;
        // updates base for next iteration
        base *= nb_elements_left;
        nb_elements_left -= 1;
    }

    result
}

/// turns a decimal number into a partial permutation
pub fn permutation_from_decimal<const NB_ELEMENTS: usize>(mut decimal: usize) -> [u8; NB_ELEMENTS]
{
    // the permutation we will return
    let mut permutation = [0; NB_ELEMENTS];
    // represents the indices shifted after each value removal
    let mut unshifted_indices: Vec<usize> = (0..NB_ELEMENTS).collect();

    // rebuild elements one after the other
    for (i, nb_elements_left) in (1..=NB_ELEMENTS).rev().enumerate()
    {
        // gets index and updates decimal
        let shifted_i = decimal % nb_elements_left;
        decimal /= nb_elements_left;
        // unshifts index and update unshifting table
        let unshifted_i = unshifted_indices[shifted_i];
        unshifted_indices.remove(shifted_i);
        // updates permutation
        permutation[i] = unshifted_i as u8;
    }

    permutation
}

/// returns the number of permutations possible with nb_elements
/// sampled from a larger nb_elements_total (n pick k computation)
/// NOTE: this function should be const
pub fn nb_partial_permutations(nb_elements: usize, nb_elements_total: usize) -> usize
{
    // factorial(nb_elements_total) / factorial(nb_elements_total - nb_elements)
    let mut product = 1;
    let lower_nb_elements = nb_elements_total - nb_elements + 1;
    for i in lower_nb_elements..=nb_elements_total
    {
        product *= i;
    }
    product
}

/// turns a partial permutation into a decimal number
///
/// this function does the same thing as decimal_from_permutation but stops early
pub fn decimal_from_partial_permutation<const NB_ELEMENTS: usize, const PERMUTATION_SIZE: usize>(partial_permutation: &[u8; PERMUTATION_SIZE])
                                                                                                 -> usize
{
    // the result we will return
    let mut result = 0;
    // represents the indices shifted after each value removal
    let mut shifted_indices = [0; NB_ELEMENTS];
    for i in 0..NB_ELEMENTS
    {
        shifted_indices[i] = i;
    }
    // how many elements are left to process
    let mut nb_elements_left = NB_ELEMENTS;
    // the base by which we are multiplying
    let mut base = 1;

    // adds elements one after the other
    for i in partial_permutation.iter().map(|i| *i as usize)
    {
        // gets shifted index
        let shifted_i = shifted_indices[i];
        // updates shift
        for j in (i + 1)..NB_ELEMENTS
        {
            shifted_indices[j] -= 1;
        }
        // updates result
        result += base * shifted_i;
        // updates base for next iteration
        base *= nb_elements_left;
        nb_elements_left -= 1;
    }

    result
}

/// turns a decimal number into a partial permutation
///
/// this is the same as permutation_from_decimal but stopping early
pub fn partial_permutation_from_decimal<const NB_ELEMENTS: usize, const PERMUTATION_SIZE: usize>(
    mut decimal: usize)
    -> [u8; PERMUTATION_SIZE]
{
    // the permutation we will return
    let mut permutation = [0; PERMUTATION_SIZE];
    // represents the indices shifted after each value removal
    let mut unshifted_indices: Vec<usize> = (0..NB_ELEMENTS).collect();

    // rebuild elements one after the other
    for (i, nb_elements_left) in (1..=NB_ELEMENTS).rev().take(PERMUTATION_SIZE).enumerate()
    {
        // gets index and updates decimal
        let shifted_i = decimal % nb_elements_left;
        decimal /= nb_elements_left;
        // unshifts index and update unshifting table
        let unshifted_i = unshifted_indices[shifted_i];
        unshifted_indices.remove(shifted_i);
        // updates permutation
        permutation[i] = unshifted_i as u8;
    }

    permutation
}
