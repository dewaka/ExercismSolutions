#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec()
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|&n| n)
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut res = self.scores.clone();
        res.sort();
        res.last().map(|&n| n)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut res = Vec::new();
        let mut scores = self.scores.clone();
        scores.sort();
        let n = scores.len();

        for i in 1..=3 {
            if i <= n {
                let cur = scores[n - i];
                res.push(cur);
            }
        }

        res
    }
}
