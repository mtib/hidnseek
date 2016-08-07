pub struct Player {
    name: String,
}

impl Player {
    pub fn new() -> Self {
        Player{
            name: "Anonymus".to_owned()
        }
    }
    pub fn set_name(&mut self, ns: &str) {
        self.name = ns.to_owned();
    }
}
