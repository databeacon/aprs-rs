
pub struct Feet(f32);
impl Feet {
    pub fn from_meters(v : f32) -> Feet {
        Feet(v/0.3048)
    }
}
impl From<f32> for Feet {
    fn from(v : f32) -> Feet {
        Feet(v)
    }
}

pub struct Knots(f32);
impl Knots {
    pub fn from_kmh(v : f32) -> Knots {
        Knots(v/1.852)
    }
}
impl From<f32> for Knots {
    fn from(v : f32) -> Knots {
        Knots(v)
    }
}

pub struct Degrees(f32);
impl From<f32> for Degrees {
    fn from(v : f32) -> Degrees {
        Degrees(v % 360.0)
    }
}

pub struct Fahrenheits(f32);
impl From<f32> for Fahrenheits {
    fn from(v : f32) -> Fahrenheits {
        Fahrenheits(v)
    }
}