use crate::room::Room;

pub struct Home {
    name: String,
    rooms: Vec<Room>,
}

impl Home {
    pub fn new(name: String) -> Self {
        Home {
            name,
            rooms: Vec::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn rooms_count(&self) -> usize {
        self.rooms.len()
    }

    pub fn report(&self) -> Vec<String> {
        let mut reports = Vec::from([format!(
            "{} report, {} room(s):",
            &self.name,
            &self.rooms.len()
        )]);
        for room in &self.rooms {
            reports.push(format!("{} report, room area {}", &room.name, &room.area));
            reports.extend_from_slice(&room.report()[..]);
            reports.push(String::from("&&"));
        }
        reports
    }

    pub fn find(&self, name: &str) -> Result<&Room, String> {
        match self.rooms.iter().find(|room| room.name == *name) {
            Some(room) => Ok(room),
            None => Err(String::from("Room not found")),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        matches!(self.rooms.iter().find(|room| room.name == name), Some(_))
    }

    pub fn remove(&mut self, name: &str) -> bool {
        if let Some(index) = self.rooms.iter().position(|room| room.name == name) {
            self.rooms.swap_remove(index);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::home::Home;

    #[test]
    fn create_home() {
        let home = Home::new(String::from("home"));
        assert_eq!(home.name, "home");
        assert_eq!(home.rooms_count(), 0);
    }
}
