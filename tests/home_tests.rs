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
    fn setup_home_with_rooms() {
        let mut home = Home::new(String::from("Country house"));
        let hall = Room::new(String::from("Hall"), 24);

        let mut bedroom = Room::new(String::from("Bed room"), 12);
        bedroom.install(Device::Thermometer(Thermometer::new(
            String::from("t2"),
            12,
        )));
        bedroom.install(Device::SmartPlug(SmartPlug::new(String::from("p2"), 220)));

        home.add_room(bedroom);

        assert!(home.contains("Bed room"));
        assert!(!home.contains("Hall"));
        assert_eq!(home.rooms_count(), 1);

        home.add_room(hall);
        assert!(home.contains("Hall"));
        assert_eq!(home.rooms_count(), 2);
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

        home.add_room(hall);
        home.add_room(bedroom);

        let report = home.report();
        assert_gt!(report.len(), 0);
    }

    #[test]
    fn find_room_in_the_house() {
        let mut home = Home::new(String::from("Country house"));
        let bedroom = Room::new(String::from("Bed room"), 12);
        home.add_room(bedroom);

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
