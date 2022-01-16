use crate::home_device::HomeDevice;

pub struct Thermometer {
    pub(crate) name: String,
    temperature: u16,
}

impl Thermometer {
    pub fn new(name: &str, temperature: u16) -> Self {
        Thermometer {
            name: name.to_string(),
            temperature,
        }
    }

    fn current_temperature(&self) -> u16 {
        self.temperature
    }
}

impl HomeDevice for Thermometer {
    fn report(&self) -> String {
        format!(
            "Thermometer {}: current temperature {}",
            self.name,
            self.current_temperature()
        )
    }
}
