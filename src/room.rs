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

    pub fn uninstall(&mut self, device_name: &str) -> Option<Device> {
        if let Some(index) = self.devices.iter().position(|device| device.get_name() == device_name) {
            Some(self.devices.swap_remove(index))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uninstall_device() {
        let mut room = Room::new("home", 20);
        let _ = room.install(Device::create_thermometer("t1", 100));

        let res = room.uninstall("t");
        assert!(res.is_none());
        assert_eq!(room.devices.len(), 1);

        let res = room.uninstall("t1");
        assert!(res.is_some());
        assert_eq!(room.devices.len(), 0);
        assert_eq!(res.unwrap().get_name(), "t1");
    }
}
