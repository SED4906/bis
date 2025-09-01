use crate::instruments::Instrument;

pub struct Module {
    instruments: Vec<Instrument>,
    channels: Vec<Channel>,
    patterns: Vec<Pattern>,
}

pub struct Channel {
    orders: Vec<usize>,
}

pub struct Pattern {
    notes: Vec<Note>,
}

pub struct Note {
    frequency: f32,
    volume: f32,
    duration: f32,
}

impl Module {
    pub fn render(&self) -> Vec<u8> {
        let mut result = vec![];
        for channel in self.channels.iter() {
            
        }
        result
    }
}
