use std::collections::{BTreeMap, VecDeque};

pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    fn to_rail(&self, idx: usize) -> usize {
        let stride_len = self.rails + self.rails - 2;
        let idx = idx % stride_len;
        if idx >= self.rails {
            return stride_len - idx;
        }
        idx
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![String::with_capacity(text.len() / 2); self.rails];
        for (idx, c) in text.char_indices() {
            rails[self.to_rail(idx)].push(c);
        }
        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rail_lengths = BTreeMap::new();
        for (idx, _) in cipher.char_indices() {
            *rail_lengths.entry(self.to_rail(idx)).or_default() += 1;
        }

        let mut chars = cipher.chars();
        let mut rails = Vec::with_capacity(self.rails);
        for &rail_length in rail_lengths.values() {
            let rail = (&mut chars).take(rail_length).collect::<VecDeque<_>>();
            rails.push(rail);
        }

        (0..cipher.len())
            .map(|idx| rails[self.to_rail(idx)].pop_front().unwrap())
            .collect()
    }
}
