/// Returns a vector with all the possibilities for the content of the two vectors
///
/// # Arguments
/// - a: First vector to compare
/// - b: Second vector to compare
///
/// # Returns
/// Vector of tuples with all possibilities
pub fn cartesian_product<T: Copy>(vec_a: Vec<T>, vec_b: Vec<T>) -> Vec<(T, T)> {
    //TODO: Check empty or == 1
    vec_a
        .into_iter()
        .flat_map(|a| vec_b.iter().map(move |&b| (a, b)))
        .collect()
}

/// Returns all order permutations possibles in a given vector
///
/// # Arguments
/// - elements: vector of things to permute
///
/// # Returns
/// - Vector of all permutations possible
pub fn all_order_permutations<T: Clone>(elements: Vec<T>) -> Vec<Vec<T>> {
    if elements.is_empty() {
        return vec![vec![]];
    }

    if elements.len() == 1 {
        return vec![elements];
    }

    elements
        .iter()
        .enumerate()
        .flat_map(|(i, elem)| {
            let mut remaining = elements.clone();
            remaining.remove(i);

            all_order_permutations(remaining)
                .into_iter()
                .map(move |mut perm| {
                    perm.insert(0, elem.clone());
                    perm
                })
        })
        .collect()
}

#[test]
fn cartesian_product_test() {
    let a = vec![1, 2, 3];
    let b = vec![6, 7, 8];

    assert_eq!(
        cartesian_product(a, b),
        vec![
            (1, 6),
            (1, 7),
            (1, 8),
            (2, 6),
            (2, 7),
            (2, 8),
            (3, 6),
            (3, 7),
            (3, 8)
        ]
    )
}

#[test]
fn all_order_permutation_test() {
    let result = all_order_permutations(vec![1, 2, 3, 4]);
    assert_eq!(result.len(), 24);

    assert!(result.contains(&vec![1, 2, 3, 4]));
    assert!(result.contains(&vec![2, 3, 4, 1]));
    assert!(result.contains(&vec![3, 4, 1, 2]));
    assert!(result.contains(&vec![4, 1, 2, 3]));
    assert!(result.contains(&vec![2, 4, 3, 1]));
}
