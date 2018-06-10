use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)] pub struct Meters(pub f32);
impl Meters {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}
impl Display for Meters {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\u{202f}m", self.0)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)] pub struct MetersPerSecond(pub f32);
impl MetersPerSecond {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}
impl Display for MetersPerSecond {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\u{202f}m/s", self.0)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)] pub struct Radians(pub f32);
impl Radians {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}
impl Display for Radians {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)] pub struct Celsius(pub f32);
impl Celsius {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}
impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}°C", self.0)
    }
}


#[derive(Debug, Clone, Copy, PartialEq)] pub struct Feet(pub f32);
impl Feet {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
    #[deprecated(since="0.2.0", note="use `Feet::from(Meters(v))` instead")]
    pub fn from_meters(v : f32) -> Feet {
        Feet::from(Meters(v))
    }
}
impl Display for Feet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\u{202f}ft", self.0)
    }
}
const FEET_IN_METER : f32 = 3.28084;
impl From<Meters> for Feet {
    fn from(v : Meters) -> Self {
        Feet(v.0 * FEET_IN_METER)
    }
}
impl From<Feet> for Meters {
    fn from(v : Feet) -> Self {
        Meters(v.0 / FEET_IN_METER)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)] pub struct Knots(pub f32);
impl Knots {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
    #[deprecated(since="0.2.0", note="use `Knots::from(KilometersPerHour(v))` instead")]
    pub fn from_kmh(v : f32) -> Knots {
        Knots(0.539957*v)
    }
}
impl Display for Knots {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\u{202f}kn", self.0)
    }
}
const KNOTS_IN_MPS : f32 = 1.94384;
impl From<MetersPerSecond> for Knots {
    fn from(v : MetersPerSecond) -> Self {
        Knots(v.0 * KNOTS_IN_MPS)
    }
}
impl From<KilometersPerHour> for Knots {
    fn from(v : KilometersPerHour) -> Self {
        Knots::from(MetersPerSecond::from(KilometersPerHour(v.0)))
    }
}
impl From<Knots> for MetersPerSecond {
    fn from(v : Knots) -> Self {
        MetersPerSecond(v.0 / KNOTS_IN_MPS)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)] pub struct MilesPerHour(pub f32);
impl MilesPerHour {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}
impl Display for MilesPerHour {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\u{202f}mph", self.0)
    }
}
const MPH_IN_MPS : f32 = 2.23694;
impl From<MetersPerSecond> for MilesPerHour {
    fn from(v : MetersPerSecond) -> Self {
        MilesPerHour(v.0 * MPH_IN_MPS)
    }
}
impl From<MilesPerHour> for MetersPerSecond {
    fn from(v : MilesPerHour) -> Self {
        MetersPerSecond(v.0 / MPH_IN_MPS)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)] pub struct KilometersPerHour(pub f32);
impl KilometersPerHour {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}
impl Display for KilometersPerHour {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\u{202f}km/h", self.0)
    }
}
const KMH_IN_MPS : f32 = 3.6;
impl From<MetersPerSecond> for KilometersPerHour {
    fn from(v : MetersPerSecond) -> Self {
        KilometersPerHour(v.0 * KMH_IN_MPS)
    }
}
impl From<KilometersPerHour> for MetersPerSecond {
    fn from(v : KilometersPerHour) -> Self {
        MetersPerSecond(v.0 / KMH_IN_MPS)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)] pub struct Degrees(pub f32);
impl Degrees {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}
impl Display for Degrees {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}°", self.0)
    }
}
impl From<Radians> for Degrees {
    fn from(v : Radians) -> Self {
        Degrees(v.0.to_degrees())
    }
}
impl From<Degrees> for Radians {
    fn from(v : Degrees) -> Self {
        Radians(v.0.to_radians())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)] pub struct Fahrenheits(pub f32);
impl Fahrenheits {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}
impl Display for Fahrenheits {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}°F", self.0)
    }
}
impl From<Fahrenheits> for Celsius {
    fn from(v : Fahrenheits) -> Self {
        Celsius( 5.0*(v.0 - 32.0)/9.0 )
    }
}
impl From<Celsius> for Fahrenheits {
    fn from(v : Celsius) -> Self {
        Fahrenheits( 32.0 + 9.0*v.0/5.0 )
    }
}


#[cfg(test)]
mod tests {
    use super::{Meters, Feet, Knots, MetersPerSecond, KilometersPerHour, MilesPerHour, Celsius, Fahrenheits};
    const EPSILON : f32 = 0.0001;

    #[test]
    fn test_as_f64() {
         assert_eq!(Meters(1.0).as_f64(), 1.0);
         assert_eq!(Knots(1.0).as_f64(), 1.0);
    }

    #[test]
    fn test_feet_to_meters() {
         assert_abs_diff_eq!(Meters::from(Feet(100.0)).0, Meters(30.48).0, epsilon=EPSILON);
         assert_abs_diff_eq!(Feet::from(Meters(30.48)).0, Feet(100.0).0, epsilon=EPSILON);
    }

    #[test]
    fn test_knots_to_mps() {
         assert_abs_diff_eq!(MetersPerSecond::from(Knots(20.0)).0, MetersPerSecond(10.2889).0, epsilon=EPSILON);
         assert_abs_diff_eq!(Knots::from(MetersPerSecond(10.2889)).0, Knots(20.0).0, epsilon=EPSILON);
    }

    #[test]
    fn test_kmh_to_mps() {
         assert_abs_diff_eq!(MetersPerSecond::from(KilometersPerHour(90.0)).0, MetersPerSecond(25.0).0, epsilon=EPSILON);
         assert_abs_diff_eq!(KilometersPerHour::from(MetersPerSecond(25.0)).0, Knots(90.0).0, epsilon=EPSILON);
    }

    #[test]
    fn test_mph_to_mps() {
         assert_abs_diff_eq!(MetersPerSecond::from(MilesPerHour(55.9234)).0, MetersPerSecond(25.0).0, epsilon=EPSILON);
         assert_abs_diff_eq!(MilesPerHour::from(MetersPerSecond(25.0)).0, Knots(55.9234).0, epsilon=EPSILON);
    }

    #[test]
    fn test_mph_to_kmh() {
         assert_abs_diff_eq!(KilometersPerHour::from(MetersPerSecond::from(MilesPerHour(60.0))).0, KilometersPerHour(96.5605).0, epsilon=EPSILON);
         assert_abs_diff_eq!(MilesPerHour::from(MetersPerSecond::from(KilometersPerHour(96.5605))).0, MilesPerHour(60.0).0, epsilon=EPSILON);
    }

    #[test]
    fn test_kmh_to_knots() {
         //assert_abs_diff_eq!(KilometersPerHour::from(Knots(30.0)).0, KilometersPerHour(55.56).0, epsilon=EPSILON);
         assert_abs_diff_eq!(Knots::from(KilometersPerHour(55.56)).0, Knots(30.0).0, epsilon=EPSILON);
    }

    #[test]
    fn test_celsius_to_fahrenheits() {
         assert_abs_diff_eq!(Celsius::from(Fahrenheits(80.0)).0, Celsius(26.6667).0, epsilon=EPSILON);
         assert_abs_diff_eq!(Fahrenheits::from(Celsius(26.6667)).0, Fahrenheits(80.0).0, epsilon=EPSILON);
    }

    #[test]
    fn into_itself() {
        let x : Meters = Meters(5.0).into();
        assert_eq!(x.0, 5.0);
    }

    #[test]
    fn convert_into() {
        let x : Meters = Feet(100.0).into();
        assert_eq!(x.0, 30.48);

        let x : Feet = Meters(30.48).into();
        assert_eq!(x.0, 100.0);
    }

    #[test]
    fn test_debug_fmt() {
         assert_eq!(format!("{:?}", Meters(5.0)), "Meters(5.0)");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Meters(5.0)), "5 m");
        assert_eq!(format!("{}", Fahrenheits(5.0)), "5°F");
    }

    #[test] #[allow(deprecated)]
    fn test_deprecated() {
        assert_eq!(Feet::from_meters(100.0), Feet(328.08398));
    }

}
