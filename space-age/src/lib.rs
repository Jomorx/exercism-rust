// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64/31557600_f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}


macro_rules! planet {
    ($planet:ident, $period:expr) => {
        pub struct $planet;
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.0 / $period
            }
        }
    };
}

planet!(Mercury,0.2408467);
planet!(Venus,0.61519726);
planet!(Earth,1.0);
planet!(Mars,1.8808158);
planet!(Jupiter,11.862615);
planet!(Saturn,29.447498);
planet!(Uranus,84.016846);
planet!(Neptune,164.79132);
