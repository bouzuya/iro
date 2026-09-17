use crate::rgb::Rgb;

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

    pub fn to_rgb(&self) -> Rgb {
        // <https://www.w3.org/TR/css-color-4/#hsl-to-rgb>
        fn f(h: f64, s: f64, l: f64, n: f64) -> f64 {
            let k = (n + h / 30.0) % 12.0;
            let a = s * l.min(1.0 - l);
            l - a * (-1.0_f64).max((k - 3.0_f64).min(9.0_f64 - k).min(1.0_f64))
        }

        let h = self.h;
        let s = self.s;
        let l = self.l;

        let r = (f(h, s, l, 0.0) * 255.0).round() as u8;
        let g = (f(h, s, l, 8.0) * 255.0).round() as u8;
        let b = (f(h, s, l, 4.0) * 255.0).round() as u8;

        Rgb::new(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_new() -> ::anyhow::Result<()> {
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

    #[test]
    fn test_hsl_to_rgb() -> ::anyhow::Result<()> {
        assert_eq!(Hsl::new(0.0, 0.0, 0.0)?.to_rgb(), Rgb::new(0, 0, 0));
        assert_eq!(Hsl::new(0.0, 0.0, 0.01)?.to_rgb(), Rgb::new(3, 3, 3));
        assert_eq!(Hsl::new(0.0, 0.0, 0.02)?.to_rgb(), Rgb::new(5, 5, 5));
        assert_eq!(Hsl::new(0.0, 0.0, 1.0)?.to_rgb(), Rgb::new(255, 255, 255));
        assert_eq!(Hsl::new(90.0, 0.0, 1.0)?.to_rgb(), Rgb::new(255, 255, 255));
        assert_eq!(Hsl::new(180.0, 0.0, 1.0)?.to_rgb(), Rgb::new(255, 255, 255));
        assert_eq!(Hsl::new(270.0, 0.0, 1.0)?.to_rgb(), Rgb::new(255, 255, 255));
        assert_eq!(Hsl::new(90.0, 0.0, 0.5)?.to_rgb(), Rgb::new(128, 128, 128));
        assert_eq!(Hsl::new(0.0, 0.0, 0.5)?.to_rgb(), Rgb::new(128, 128, 128));
        assert_eq!(Hsl::new(120.0, 0.75, 0.25)?.to_rgb(), Rgb::new(16, 112, 16));
        Ok(())
    }
}
