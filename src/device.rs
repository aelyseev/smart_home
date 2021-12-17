use crate::home_device::HomeDevice;
use crate::smart_plug::SmartPlug;
use crate::thermometer::Thermometer;

pub enum Device {
    SmartPlug(SmartPlug),
    Thermometer(Thermometer),
}

impl Device {
    pub fn create_plug(name: String, capacity: u16) -> Self {
        Device::SmartPlug(SmartPlug::new(name, capacity))
    }

    pub fn create_thermometer(name: String, temperature: u16) -> Self {
        Device::Thermometer(Thermometer::new(name, temperature))
    }

    pub(crate) fn report(&self) -> String {
        match self {
            Device::SmartPlug(plug) => plug.report(),
            Device::Thermometer(t) => t.report(),
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            Device::SmartPlug(plug) => &plug.name,
            Device::Thermometer(t) => &t.name,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_plug_device() {
        let d = Device::create_plug(String::from("name"), 120);
        match d {
            Device::SmartPlug(_) => assert!(true),
            _ => unreachable!(),
        };
    }
    #[test]
    fn create_plug_thermometer() {
        let d = Device::create_thermometer(String::from("name"), 34);
        match d {
            Device::Thermometer(_) => assert!(true),
            _ => unreachable!(),
        };
    }
}
