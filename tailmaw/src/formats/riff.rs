use std::collections::HashMap;

pub struct Riff<'a> {
    id: u32,
    chunks: HashMap<u32, Vec<&'a [u8]>>,
}

impl<'a> Riff<'a> {
    pub fn new(input: &'a [u8]) -> Option<Riff<'a>> {
        if input[0..4] != *b"RIFF" {
            return None;
        }
        let length = u32::from_le_bytes(input[4..8].try_into().ok()?);
        let id = u32::from_le_bytes(input[8..12].try_into().ok()?);
        let chunks = Self::subchunks(&input[12..length as usize + 8])?;
        Some(Self { id, chunks })
    }

    pub fn get_chunks(&self, format: &str) -> Option<Vec<&[u8]>> {
        self.chunks.get(&u32::from_le_bytes(format.as_bytes().try_into().unwrap())).cloned()
    }

    pub fn has_id(&self, id: &str) -> bool {
        self.id == u32::from_le_bytes(id.as_bytes().try_into().unwrap())
    }
}

impl Riff<'_> {
    fn subchunks(input: &[u8]) -> Option<HashMap<u32, Vec<&[u8]>>> {
        let mut result: HashMap<u32, Vec<&[u8]>> = HashMap::new();
        let mut index = 0;
        while let Some((format, chunk)) = Self::chunk(&input[index..]) && index < input.len() {
            if result.contains_key(&format) {
                result.get_mut(&format).unwrap().push(chunk);
            } else {
                result.insert(format, vec![chunk]);
            }
            index += 8 + chunk.len();
            if index & 1 == 1 {
                index += 1;
            }
        }
        Some(result)
    }

    fn chunk(input: &[u8]) -> Option<(u32, &[u8])> {
        if input.len() < 8 {
            return None;
        }
        let format = u32::from_le_bytes(input[0..4].try_into().ok()?);
        let length = u32::from_le_bytes(input[4..8].try_into().ok()?);
        let data = &input[8..length as usize + 8];
        Some((format, data))
    }
}
