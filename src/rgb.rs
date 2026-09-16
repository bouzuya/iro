#[derive(Debug, ::thiserror::Error)]
#[error("rgb error")]
pub struct RgbError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

// associated functions
impl Rgb {
    pub fn bouzuya_green() -> Self {
        // bouzuya-green: #4e6a41 rgb(78, 106, 65)
        Self {
            r: 78,
            g: 106,
            b: 65,
        }
    }

    pub fn from_hex(s: &str) -> Result<Self, RgbError> {
        if s.len() != 7 || !s.starts_with('#') || s.chars().skip(1).any(|c| !c.is_ascii_hexdigit())
        {
            return Err(RgbError);
        }
        let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| RgbError)?;
        let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| RgbError)?;
        let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| RgbError)?;
        Ok(Self { r, g, b })
    }

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

// methods
impl Rgb {
    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_bouzuya_green() {
        let rgb = Rgb::bouzuya_green();
        assert_eq!(rgb.r(), 78);
        assert_eq!(rgb.g(), 106);
        assert_eq!(rgb.b(), 65);
        assert_eq!(rgb.to_hex(), "#4E6A41");
    }

    #[test]
    fn test_rgb_from_hex() -> ::anyhow::Result<()> {
        let rgb = Rgb::from_hex("#FF8000")?;
        assert_eq!(rgb.r(), 255);
        assert_eq!(rgb.g(), 128);
        assert_eq!(rgb.b(), 0);
        Ok(())
    }

    #[test]
    fn test_rgb_new() {
        let rgb = Rgb::new(255, 128, 0);
        assert_eq!(rgb.r(), 255);
        assert_eq!(rgb.g(), 128);
        assert_eq!(rgb.b(), 0);
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
