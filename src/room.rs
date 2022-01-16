use crate::app_error::AppError;
use crate::device::Device;

pub struct Room {
    pub name: String,
    pub(crate) area: u8,
    pub devices: Vec<Device>,
}

impl Room {
    pub(crate) fn report(&self) -> Vec<String> {
        let mut report = Vec::with_capacity(self.devices.len());
        for device in &self.devices {
            report.push(device.report());
        }
        report
    }

    pub fn find(&self, name: &str) -> Option<&Device> {
        self.devices
            .iter()
            .find(|device| name == *device.get_name())
    }

    pub fn new(name: &str, area: u8) -> Self {
        Self {
            name: name.to_string(),
            area,
            devices: Vec::new(),
        }
    }

    pub fn install(&mut self, device: Device) -> Result<(), AppError> {
        match self.find(device.get_name()) {
            Some(_) => Err(AppError::new(
                format!(
                    "The device with name {} already installed",
                    device.get_name()
                )
                .as_str(),
            )),
            None => {
                self.devices.push(device);
                Ok(())
            }
        }
    }
}
