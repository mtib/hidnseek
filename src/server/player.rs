#![allow(dead_code)]
use std::fmt;
use ::server;

/// The servers view of the clients.
/// The clients only know about other clients via the player
/// interface if the server tells them.
pub struct Player {
    name: String,
    pos_x: u32,
    pos_y: u32,
    upstream: Option<*const server::Server>
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: "Anonymus".to_owned(),
            pos_x: 0,
            pos_y: 0,
            upstream: None,
        }
    }
    pub fn set_name(&mut self, ns: &str) {
        self.name = ns.to_owned();
    }
    // this is not exactly necessary, but was useful for testing out
    // how to use unsafe, this will mean, that the Server will not
    // have to pass all the Configuration to the "Player" object on
    // every function call.
    pub unsafe fn give_upstream(&mut self, s: *const server::Server) {
        self.upstream = Some(s);
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player<name: {}>", self.name)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player<{}>", self.name)
    }
}
