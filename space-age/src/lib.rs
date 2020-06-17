#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self::new(s)
    }
}

impl Duration {
    fn new(seconds: u64) -> Self {
        Self { seconds }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (31557600 as f64 * Self::ORBITAL_PERIOD)
    }
}

#[macro_export]
macro_rules! planet {
    ($x:ident, $y:expr) => {
        pub struct $x;

        impl Planet for $x {
            const ORBITAL_PERIOD: f64 = $y;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
