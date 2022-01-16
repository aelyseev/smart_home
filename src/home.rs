use crate::app_error::AppError;
use crate::room::Room;

pub struct Home {
    name: String,
    rooms: Vec<Room>,
}

impl Home {
    pub fn new(name: &str) -> Self {
        Home {
            name: name.to_string(),
            rooms: Vec::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) -> Result<(), AppError> {
        match self.find(&room.name) {
            Some(_) => Err(AppError::new(
                format!("Room with name {} already exists in the home", &room.name).as_str(),
            )),
            None => {
                self.rooms.push(room);
                Ok(())
            }
        }
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

    pub fn find(&self, name: &str) -> Option<&Room> {
        self.rooms.iter().find(|room| room.name == *name)
    }

    pub fn contains(&self, name: &str) -> bool {
        matches!(self.rooms.iter().find(|room| room.name == name), Some(_))
    }

    pub fn remove(&mut self, name: &str) -> Option<Room> {
        if let Some(index) = self.rooms.iter().position(|room| room.name == name) {
            Some(self.rooms.swap_remove(index))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::home::Home;

    #[test]
    fn create_home() {
        let home = Home::new("home");
        assert_eq!(home.name, "home");
        assert_eq!(home.rooms_count(), 0);
    }
}
