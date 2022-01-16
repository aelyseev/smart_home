#[macro_use]
extern crate more_asserts;
extern crate smart_home_lib;

mod tests {
    use smart_home_lib::device::Device;
    use smart_home_lib::home::Home;
    use smart_home_lib::room::Room;
    use smart_home_lib::smart_plug::SmartPlug;
    use smart_home_lib::thermometer::Thermometer;

    #[test]
    fn check_home_room_adding() {
        let mut home = Home::new("Country house");
        let hall = Room::new("Hall", 24);
        let hall2 = Room::new("Hall", 24);

        let result = home.add_room(hall);
        assert!(result.is_ok());
        assert!(home.contains("Hall"));
        assert_eq!(home.rooms_count(), 1);

        let result = home.add_room(hall2);
        assert!(result.is_err());
        assert_eq!(home.rooms_count(), 1);
    }

    #[test]
    fn room_deleting() {
        let mut home = Home::new("Country house");
        let hall = Room::new("Hall", 24);
        let hall2 = Room::new("Playing room", 24);

        let result = home.add_room(hall);
        assert!(result.is_ok());

        let result = home.add_room(hall2);
        assert!(result.is_ok());
        assert_eq!(home.rooms_count(), 2);

        let room = home.remove("Hall");
        assert!(room.is_some());
        assert_eq!(home.rooms_count(), 1);
    }

    #[test]
    fn setup_home_with_rooms() {
        let mut home = Home::new("Country house");
        let hall = Room::new("Hall", 24);

        let mut bedroom = Room::new("Bed room", 12);
        let result = bedroom.install(Device::Thermometer(Thermometer::new("t2", 12)));
        assert!(result.is_ok());

        let result = bedroom.install(Device::SmartPlug(SmartPlug::new("p2", 220)));
        assert!(result.is_ok());

        let result = home.add_room(bedroom);
        assert!(result.is_ok());

        assert!(home.contains("Bed room"));
        assert!(!home.contains("Hall"));
        assert_eq!(home.rooms_count(), 1);

        let result = home.add_room(hall);
        assert!(result.is_ok());
        assert!(home.contains("Hall"));
        assert_eq!(home.rooms_count(), 2);
    }

    #[test]
    fn check_home_report() {
        let mut home = Home::new("Country house");
        let mut hall = Room::new("Hall", 24);

        let result = hall.install(Device::Thermometer(Thermometer::new("t1", 12)));
        assert!(result.is_ok());
        let result = hall.install(Device::SmartPlug(SmartPlug::new("s1", 220)));
        assert!(result.is_ok());

        let mut plug = SmartPlug::new("s2", 120);
        plug.turn_off();
        plug.turn_on();
        let result = hall.install(Device::SmartPlug(plug));
        assert!(result.is_ok());

        let mut bedroom = Room::new("Bed room", 12);
        let result = bedroom.install(Device::Thermometer(Thermometer::new("t1", 12)));
        assert!(result.is_ok());

        let result = bedroom.install(Device::SmartPlug(SmartPlug::new("s1", 220)));
        assert!(result.is_ok());

        let result = home.add_room(hall);
        assert!(result.is_ok());

        let result = home.add_room(bedroom);
        assert!(result.is_ok());

        let report = home.report();
        assert_gt!(report.len(), 0);
    }

    #[test]
    fn find_room_in_the_house() {
        let mut home = Home::new("Country house");
        let bedroom = Room::new("Bed room", 12);

        let result = home.add_room(bedroom);
        assert!(result.is_ok());

        let search = home.find("Bed room");

        assert_eq!(search.unwrap().name, "Bed room");

        home.remove("Bed room");
        let search = home.find("Bed room");
        assert!(search.is_none());
    }

    #[test]
    fn test_adding_device_into_room() {
        let mut room = Room::new("room", 12);
        let device = Device::SmartPlug(SmartPlug::new("d1", 120));

        let result = room.install(device);
        assert!(result.is_ok());
        assert_eq!(room.devices.len(), 1);

        let device = Device::Thermometer(Thermometer::new("d1", 30));
        let result = room.install(device);
        assert!(result.is_err());
        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn find_device_in_the_room() {
        let mut bedroom = Room::new("Bed room", 12);
        let result = bedroom.install(Device::SmartPlug(SmartPlug::new("s1", 120)));
        assert!(result.is_ok());

        let result = bedroom.install(Device::SmartPlug(SmartPlug::new("s2", 120)));
        assert!(result.is_ok());

        let result = bedroom.install(Device::Thermometer(Thermometer::new("t1", 30)));
        assert!(result.is_ok());

        let device = bedroom.find("t1");
        assert_eq!(bedroom.devices.len(), 3);
        assert!(device.is_some());
        assert_eq!(*device.unwrap().get_name(), "t1");
    }
}
