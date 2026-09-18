use crate::rgb::Rgb;

#[derive(Debug, ::thiserror::Error)]
#[error(transparent)]
pub struct HslError(E);

#[derive(Debug, ::thiserror::Error)]
enum E {
    #[error("invalid hue")]
    InvalidHue,
    #[error("invalid saturation")]
    InvalidSaturation,
    #[error("invalid lightness")]
    InvalidLightness,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hsl {
    /// hue [0, 360)
    h: u16,
    /// saturation [0, 100]
    s: u8,
    /// lightness [0, 100]
    l: u8,
}

// associated functions
impl Hsl {
    pub fn from_rgb(rgb: Rgb) -> Self {
        // <https://www.w3.org/TR/css-color-4/#rgb-to-hsl>
        let r = rgb.r() as f64 / 255.0;
        let g = rgb.g() as f64 / 255.0;
        let b = rgb.b() as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let (mut h, mut s, l) = (f64::NAN, 0.0, (min + max) / 2.0);
        let d = max - min;
        let epsilon = 1.0 / 100000.0;

        if d != 0.0 {
            s = if l == 0.0 || l == 1.0 {
                0.0
            } else {
                (max - l) / l.min(1.0 - l)
            };

            if r > g && r > b {
                h = (g - b) / d + if g < b { 6.0 } else { 0.0 };
            } else if g > r && g > b {
                h = (b - r) / d + 2.0;
            } else if b > r && b > g {
                h = (r - g) / d + 4.0;
            } else {
                // ...
                h = (r - g) / d + 4.0;
            }

            h *= 60.0;
        }

        if s < 0.0 {
            h += 180.0;
            s = s.abs();
        }

        if h >= 360.0 {
            h -= 360.0;
        }

        if s <= epsilon {
            // h = f64::NAN;
            h = 0.0;
        }

        let h = h.round() as u16;
        let s = (s * 100.0).round() as u8;
        let l = (l * 100.0).round() as u8;

        Self::new(h, s, l).expect("rgb should convert to valid hsl")
    }

    pub fn new(h: u16, s: u8, l: u8) -> Result<Self, HslError> {
        if !(0..360).contains(&h) {
            return Err(HslError(E::InvalidHue));
        }
        if !(0..=100).contains(&s) {
            return Err(HslError(E::InvalidSaturation));
        }
        if !(0..=100).contains(&l) {
            return Err(HslError(E::InvalidLightness));
        }

        Ok(Self { h, s, l })
    }
}

// methods
impl Hsl {
    pub fn h(&self) -> u16 {
        self.h
    }

    pub fn l(&self) -> u8 {
        self.l
    }

    pub fn s(&self) -> u8 {
        self.s
    }

    pub fn to_rgb(&self) -> Rgb {
        // <https://www.w3.org/TR/css-color-4/#hsl-to-rgb>
        fn f(h: f64, s: f64, l: f64, n: f64) -> f64 {
            let k = (n + h / 30.0) % 12.0;
            let a = s * l.min(1.0 - l);
            l - a * (-1.0_f64).max((k - 3.0_f64).min(9.0_f64 - k).min(1.0_f64))
        }

        let h = self.h as f64;
        let s = self.s as f64 / 100.0;
        let l = self.l as f64 / 100.0;

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
    fn test_hsl_from_rgb() -> ::anyhow::Result<()> {
        assert_eq!(Hsl::from_rgb(Rgb::new(16, 112, 16)), Hsl::new(120, 75, 25)?);
        assert_eq!(Hsl::from_rgb(Rgb::new(0, 255, 0)), Hsl::new(120, 100, 50)?);
        assert_eq!(Hsl::from_rgb(Rgb::new(0, 128, 0)), Hsl::new(120, 100, 25)?);
        assert_eq!(
            Hsl::from_rgb(Rgb::new(128, 255, 128)),
            Hsl::new(120, 100, 75)?
        );

        assert_eq!(Hsl::from_rgb(Rgb::new(3, 3, 3)), Hsl::new(0, 0, 1)?);
        assert_eq!(
            Hsl::from_rgb(Rgb::new(188, 245, 188)),
            Hsl::new(120, 74, 85)?
        );
        assert_eq!(Hsl::new(120, 74, 85)?.to_rgb(), Rgb::new(188, 245, 188));
        assert_eq!(Hsl::new(120, 75, 85)?.to_rgb(), Rgb::new(188, 245, 188));

        // TODO: <https://www.w3.org/TR/css-color-4/#hsl-examples>
        Ok(())
    }

    #[test]
    fn test_hsl_new() -> ::anyhow::Result<()> {
        assert!(Hsl::new(0, 0, 0).is_ok());
        assert!(Hsl::new(359, 100, 100).is_ok());
        assert!(Hsl::new(360, 0, 0).is_err());
        assert!(Hsl::new(0, 101, 0).is_err());
        assert!(Hsl::new(0, 0, 101).is_err());

        let hsl = Hsl::new(120, 75, 25)?;
        assert_eq!(hsl.h(), 120);
        assert_eq!(hsl.s(), 75);
        assert_eq!(hsl.l(), 25);
        Ok(())
    }

    #[test]
    fn test_hsl_to_rgb() -> ::anyhow::Result<()> {
        assert_eq!(Hsl::new(0, 0, 0)?.to_rgb(), Rgb::new(0, 0, 0));
        assert_eq!(Hsl::new(0, 0, 1)?.to_rgb(), Rgb::new(3, 3, 3));
        assert_eq!(Hsl::new(0, 0, 2)?.to_rgb(), Rgb::new(5, 5, 5));
        assert_eq!(Hsl::new(0, 0, 100)?.to_rgb(), Rgb::new(255, 255, 255));
        assert_eq!(Hsl::new(90, 0, 100)?.to_rgb(), Rgb::new(255, 255, 255));
        assert_eq!(Hsl::new(180, 0, 100)?.to_rgb(), Rgb::new(255, 255, 255));
        assert_eq!(Hsl::new(270, 0, 100)?.to_rgb(), Rgb::new(255, 255, 255));
        assert_eq!(Hsl::new(90, 0, 50)?.to_rgb(), Rgb::new(128, 128, 128));
        assert_eq!(Hsl::new(0, 0, 50)?.to_rgb(), Rgb::new(128, 128, 128));
        assert_eq!(Hsl::new(120, 75, 25)?.to_rgb(), Rgb::new(16, 112, 16));
        Ok(())
    }
}
