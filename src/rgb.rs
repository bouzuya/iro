#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_new() {
        let rgb = Rgb::new(255, 128, 0);
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 128);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn test_rgb_to_hex() {
        let rgb = Rgb::new(255, 128, 0);
        assert_eq!(rgb.to_hex(), "#FF8000");
    }

    #[test]
    fn test_impl_clone() {
        fn assert_impl<T: Clone>() {}
        assert_impl::<Rgb>();
    }

    #[test]
    fn test_impl_copy() {
        fn assert_impl<T: Copy>() {}
        assert_impl::<Rgb>();
    }

    #[test]
    fn test_impl_debug() {
        fn assert_impl<T: std::fmt::Debug>() {}
        assert_impl::<Rgb>();
    }

    #[test]
    fn test_impl_eq() {
        fn assert_impl<T: Eq>() {}
        assert_impl::<Rgb>();
    }

    #[test]
    fn test_impl_partial_eq() {
        fn assert_impl<T: PartialEq>() {}
        assert_impl::<Rgb>();
    }
}
