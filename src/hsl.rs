#[derive(Debug, ::thiserror::Error)]
#[error("hsl error")]
pub struct HslError;

pub struct Hsl {
    /// hue [0.0, 360.0)
    h: f64,
    /// saturation [0.0, 1.0]
    s: f64,
    /// lightness [0.0, 1.0]
    l: f64,
}

// associated functions
impl Hsl {
    pub fn new(h: f64, s: f64, l: f64) -> Result<Self, HslError> {
        if !(0.0..360.0).contains(&h) {
            return Err(HslError);
        }
        if !(0.0..=1.0).contains(&s) {
            return Err(HslError);
        }
        if !(0.0..=1.0).contains(&l) {
            return Err(HslError);
        }
        Ok(Self { h, s, l })
    }
}

// methods
impl Hsl {
    pub fn h(&self) -> f64 {
        self.h
    }

    pub fn l(&self) -> f64 {
        self.l
    }

    pub fn s(&self) -> f64 {
        self.s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_new() -> anyhow::Result<()> {
        assert!(Hsl::new(0.0, 0.0, 0.0).is_ok());
        assert!(Hsl::new(359.9, 1.0, 1.0).is_ok());
        assert!(Hsl::new(360.0, 0.0, 0.0).is_err());
        assert!(Hsl::new(0.0, 1.1, 0.0).is_err());
        assert!(Hsl::new(0.0, 0.0, 1.1).is_err());

        let hsl = Hsl::new(120.0, 0.75, 0.25)?;
        assert_eq!(hsl.h(), 120.0);
        assert_eq!(hsl.s(), 0.75);
        assert_eq!(hsl.l(), 0.25);
        Ok(())
    }
}
