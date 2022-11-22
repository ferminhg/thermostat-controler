const MAX_TEMPERATURE_VALUE:f32 = 10000_f32;
pub struct TemperatureValue(f32);

impl TryFrom<f32> for TemperatureValue {
    type Error = ();

    fn try_from(n: f32) -> Result<Self, Self::Error> {
        if n < MAX_TEMPERATURE_VALUE {
            return Ok(Self(n));
        }
        Err(())
    }
}

impl From<TemperatureValue> for f32 {
    fn from(n: TemperatureValue) -> f32 {
        n.0
    }
}

pub enum TemperatureScale {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TryFrom<String> for TemperatureScale {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Celsius" => Ok(TemperatureScale::Celsius),
            "Fahrenheit" => Ok(TemperatureScale::Fahrenheit),
            "Kelvin" => Ok(TemperatureScale::Kelvin),
            _ => Err(()),
        }
    }
}

