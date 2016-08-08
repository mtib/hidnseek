#![allow(dead_code)]
#![allow(unused_variables)]
extern crate rand;
use self::rand::Rng;

use server::Server;
use std::fmt;

#[derive(Copy)]
#[derive(Clone)]
pub struct Tile {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

pub struct Layout {
    tiles: Vec<Vec<Tile>>,
    width: u32,
    height: u32,
}

pub struct Visuals {
    seekers: Vec<(u32,u32)>,
    hiders: Vec<(u32,u32)>,
    layout: Layout,
}

impl Tile {
    pub fn new(n: bool, e: bool, s: bool, w: bool) -> Self {
        Tile {north: n, east: e, south: s, west: w}
    }
    pub fn new_close() -> Self {
        Tile {north: false, east: false, south: false, west: false}
    }
    pub fn new_open() -> Self {
        Tile {north: true, east: true, south: true, west: true}
    }
    pub fn new_rand() -> Self {
        let mut r = rand::thread_rng();
        Tile {north: r.gen(), east: r.gen(),south: r.gen(), west: r.gen()}
    }
    pub fn randomize(&mut self) {
        let mut r = rand::thread_rng();
        self.north = r.gen();
        self.east = r.gen();
        self.south = r.gen();
        self.west = r.gen();
    }
    pub fn text_vis(&self) -> (String, String, String) {
        let mut text = (" ".to_owned(), "".to_owned(), " ".to_owned());
        if self.north {
            text.0.push_str("|");
        } else {
            text.0.push_str(" ");
        }
        if self.east {
            text.1.push_str("-");
        } else {
            text.1.push_str(" ");
        }
        text.1.push_str("O");
        if self.west {
            text.1.push_str("-");
        } else {
            text.1.push_str(" ");
        }
        if self.south {
            text.2.push_str("|");
        } else {
            text.2.push_str(" ");
        }
        text.2.push_str(" ");
        text.0.push_str(" ");
        text
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "N: {}, E: {}, S: {}, W: {}", self.north, self.east, self.south, self.west)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = self.text_vis();
        write!(f, "{}\n{}\n{}", text.0, text.1, text.2)
    }
}

impl Layout {
    pub fn new(width: u32, height: u32) -> Self {
        Layout{
            tiles: vec![vec![Tile::new_close();width as usize];height as usize],
            width: width,
            height: height,
        }
    }
    pub fn new_random(width: u32, height: u32) -> Self {
        let mut l = Self::new(width, height);
        for ta in &mut l.tiles {
            for t in ta {
                // TODO make sure, tiles link up.
                t.randomize();
            }
        }
        l
    }
}

impl fmt::Debug for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ta in &self.tiles {
            for t in ta {
                try!(write!(f, "{}", t.text_vis().0));
            }
            try!(write!(f, "\n"));
            for t in ta {
                try!(write!(f, "{}", t.text_vis().1));
            }
            try!(write!(f, "\n"));
            for t in ta {
                try!(write!(f, "{}", t.text_vis().2));
            }
            try!(write!(f, "\n"));
        }
        write!(f, "")
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vvt = self.tiles.iter().map(|x| x.iter().map(|v| v.text_vis()).collect::<Vec<(String, String, String)>>()).collect::<Vec<Vec<(String, String, String)>>>();
        for x in 0..vvt[0].len() {
            unimplemented!();
        }
        for y in vvt.iter().take(vvt.len()-1).skip(1) {
            try!(write!(f, "O"));
            for x in 1..y.len()-1 {
                unimplemented!();
            }
            try!(write!(f, "O"));
        }
        write!(f, "\n")
    }
}

// TODO impl Dispaly for Layout, merging paths

impl Visuals {
    pub fn new(width: u32, height: u32) -> Self{
        Visuals {
            seekers: vec![],
            hiders: vec![],
            layout: Layout::new(width, height),
        }
    }
    pub fn update(&mut self, s: &Server) {
    }
}
