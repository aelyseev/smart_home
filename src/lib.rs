#[macro_use]
extern crate more_asserts;

pub trait HomeDevice {
    fn report(&self) -> String;
}

pub struct Thermometer {
    name: String,
    temperature: u16,
}

impl Thermometer {
    fn new(name: String, temperature: u16) -> Self {
        Thermometer { name, temperature }
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

pub struct SmartPlug {
    name: String,
    status: bool,
    capacity: u16,
}

impl SmartPlug {
    fn new(name: String, capacity: u16) -> SmartPlug {
        SmartPlug {
            name,
            status: false,
            capacity,
        }
    }

    fn turn_on(&mut self) {
        self.status = true;
    }

    fn turn_off(&mut self) {
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

pub enum Device {
    SmartPlug(SmartPlug),
    Thermometer(Thermometer),
}

impl Device {
    fn report(&self) -> String {
        match self {
            Device::SmartPlug(plug) => plug.report(),
            Device::Thermometer(t) => t.report(),
        }
    }

    fn get_name(&self) -> &String {
        match self {
            Device::SmartPlug(plug) => &plug.name,
            Device::Thermometer(t) => &t.name,
        }
    }
}

pub struct Room {
    name: String,
    area: u8,
    devices: Vec<Device>,
}

impl Room {
    fn report(&self) -> Vec<String> {
        let mut report = Vec::with_capacity(self.devices.len());
        for device in &self.devices {
            report.push(device.report());
        }
        report
    }

    fn find(&self, name: &str) -> Result<&Device, String> {
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

    fn new(name: String, area: u8) -> Self {
        Self {
            name,
            area,
            devices: Vec::new(),
        }
    }

    fn install(&mut self, device: Device) {
        self.devices.push(device);
    }
}

pub struct Home {
    _name: String,
    rooms: Vec<Room>,
}

impl Home {
    fn new(name: String) -> Self {
        Home {
            _name: name,
            rooms: Vec::new(),
        }
    }

    fn report(&self) -> Vec<String> {
        let mut reports = Vec::from([format!(
            "{} report, {} room(s):",
            &self._name,
            &self.rooms.len()
        )]);
        for room in &self.rooms {
            reports.push(format!("{} report, room area {}", &room.name, &room.area));
            reports.extend_from_slice(&room.report()[..]);
            reports.push(String::from("&&"));
        }
        reports
    }

    fn find(&self, name: &str) -> Result<&Room, String> {
        match self.rooms.iter().find(|room| room.name == *name) {
            Some(room) => Ok(room),
            None => Err(String::from("Room not found")),
        }
    }

    fn contains(&self, name: &str) -> bool {
        matches!(self.rooms.iter().find(|room| room.name == name), Some(_))
    }

    fn remove(&mut self, name: &str) -> bool {
        if let Some(index) = self.rooms.iter().position(|room| room.name == name) {
            self.rooms.swap_remove(index);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Device, Home, Room, SmartPlug, Thermometer};

    #[test]
    fn setup_home_with_rooms() {
        let mut home = Home::new(String::from("Country house"));
        let hall = Room::new(String::from("Hall"), 24);

        let mut bedroom = Room::new(String::from("Bed room"), 12);
        bedroom.install(Device::Thermometer(Thermometer::new(
            String::from("t2"),
            12,
        )));
        bedroom.install(Device::SmartPlug(SmartPlug::new(String::from("p2"), 220)));

        home.rooms.push(bedroom);

        assert!(home.contains("Bed room"));
        assert!(!home.contains("Hall"));
        assert_eq!(home.rooms.len(), 1);

        home.rooms.push(hall);
        assert!(home.contains("Hall"));
        assert_eq!(home.rooms.len(), 2);
    }

    #[test]
    fn check_home_report() {
        let mut home = Home::new(String::from("Country house"));
        let mut hall = Room::new(String::from("Hall"), 24);

        hall.install(Device::Thermometer(Thermometer::new(
            String::from("t1"),
            12,
        )));
        hall.install(Device::SmartPlug(SmartPlug::new(String::from("s1"), 220)));

        let mut plug = SmartPlug::new(String::from("s2"), 120);
        plug.turn_off();
        plug.turn_on();
        hall.install(Device::SmartPlug(plug));

        let mut bedroom = Room::new(String::from("Bed room"), 12);
        bedroom.install(Device::Thermometer(Thermometer::new(
            String::from("t1"),
            12,
        )));
        bedroom.install(Device::SmartPlug(SmartPlug::new(String::from("s1"), 220)));

        home.rooms.push(hall);
        home.rooms.push(bedroom);

        let report = home.report();
        assert_gt!(report.len(), 0);
    }

    #[test]
    fn find_room_in_the_house() {
        let mut home = Home::new(String::from("Country house"));
        let bedroom = Room::new(String::from("Bed room"), 12);
        home.rooms.push(bedroom);

        let search = home.find("Bed room");

        assert!(search.is_ok());
        assert_eq!(search.unwrap().name, "Bed room");

        home.remove("Bed room");
        let search = home.find("Bed room");
        assert!(search.is_err());
    }

    #[test]
    fn find_device_in_the_room() {
        let mut bedroom = Room::new(String::from("Bed room"), 12);
        bedroom.install(Device::SmartPlug(SmartPlug::new(String::from("s1"), 120)));
        bedroom.install(Device::SmartPlug(SmartPlug::new(String::from("s2"), 120)));
        bedroom.install(Device::Thermometer(Thermometer::new(
            String::from("t1"),
            30,
        )));
        let device = bedroom.find("t1");
        assert_eq!(bedroom.devices.len(), 3);
        assert!(device.is_ok());
        assert_eq!(*device.unwrap().get_name(), "t1");
    }
}
