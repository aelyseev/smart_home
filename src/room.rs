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

    pub fn find(&self, name: &str) -> Result<&Device, String> {
        let search = self.devices.iter().find(|device| {
            let device_name = match device {
                Device::Thermometer(t) => &t.name,
                Device::SmartPlug(p) => &p.name,
            };
            *name == *device_name
        });
        match search {
            Some(device) => Ok(device),
            None => Err(format!("Device {} not found", name)),
        }
    }

    pub fn new(name: String, area: u8) -> Self {
        Self {
            name,
            area,
            devices: Vec::new(),
        }
    }

    pub fn install(&mut self, device: Device) {
        self.devices.push(device);
    }
}
