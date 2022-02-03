pub struct Duration(u64);
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n; impl Planet for $n {
            fn years_during(d: &Duration) -> f64 {
                d.0 as f64/ (31557600.0 * $p)
            }
        }
    }
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
