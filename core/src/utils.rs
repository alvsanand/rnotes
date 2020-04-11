use std::error::Error;

pub type GenericError = Box<dyn Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, GenericError>;

pub fn eq_no_ord<T>(a: &[T], b: &[T]) -> bool
where
    T: PartialEq + Ord + std::fmt::Debug,
{
    let mut a: Vec<_> = a.iter().collect();
    let mut b: Vec<_> = b.iter().collect();
    a.sort();
    b.sort();

    println!("{:?} vs {:?}", a, b);

    a == b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eq_no_ord() {
        let vec_a = vec![0, 1, 2];
        let vec_b = vec![2, 1, 0];
        let vec_c = vec![0, 1];

        assert!(eq_no_ord(&vec_a, &vec_a));
        assert!(eq_no_ord(&vec_b, &vec_b));
        assert!(eq_no_ord(&vec_a, &vec_b));
        assert!(eq_no_ord(&vec_b, &vec_a));
        assert!(!eq_no_ord(&vec_a, &vec_c));
        assert!(!eq_no_ord(&vec_b, &vec_c));
    }
}
