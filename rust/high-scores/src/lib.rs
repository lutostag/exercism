#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
    top_3: Vec<u32>,
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        let mut top_3 = scores.to_vec();
        top_3.sort();
        top_3.reverse();
        top_3.truncate(3);
        Self { scores, top_3 }
    }

    pub fn scores(&self) -> &'a [u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.top_3.first().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top_3.clone()
    }
}
