use crate::home_device::HomeDevice;

pub struct SmartPlug {
    pub(crate) name: String,
    status: bool,
    capacity: u16,
}

impl SmartPlug {
    pub fn new(name: &str, capacity: u16) -> SmartPlug {
        SmartPlug {
            name: name.to_string(),
            status: false,
            capacity,
        }
    }

    pub fn turn_on(&mut self) {
        self.status = true;
    }

    pub fn turn_off(&mut self) {
        self.status = false;
    }
}

impl HomeDevice for SmartPlug {
    fn report(&self) -> String {
        format!(
            "Smart plug {}, status {}, capacity {}",
            self.name, self.status, self.capacity
        )
    }
}
