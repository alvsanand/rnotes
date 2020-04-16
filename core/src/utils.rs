use std::fmt;

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

pub struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    pub fn new<T>(data: &'a T) -> HexSlice<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}

impl fmt::Display for HexSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hex_slice_string() {
        let data = "some-password";
        let result = HexSlice::new(data).to_string();
        let expected = "736F6D652D70617373776F7264";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_hex_slice_bytes() {
        let data = vec![0x0, 0x1,0x11,0xFF];
        let result = HexSlice::new(&data).to_string();
        let expected = "000111FF";

        assert_eq!(result, expected);
    }

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
