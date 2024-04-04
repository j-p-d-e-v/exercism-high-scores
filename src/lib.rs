#[derive(Debug)]
pub struct HighScores<'a> {
    scores_list: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            scores_list: scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores_list
    }

    pub fn latest(&self) -> Option<u32> {
        let last_index: usize = self.scores_list.len();
        if last_index > 0 {
            Some(self.scores_list[last_index - 1])
        }
        else{
            None
        }
    }

    fn scores_to_vec(&self) -> Vec<u32>{
        let mut scores_list: Vec<u32> = Vec::new();
        for score in self.scores_list {
            scores_list.push(*score);
        }
        scores_list.sort();
        scores_list.reverse();
        scores_list
    }

    pub fn personal_best(&self) -> Option<u32> {
        let scores_list: Vec<u32> = self.scores_to_vec();
        if scores_list.len() > 0 {
            Some(scores_list[0])
        }
        else{
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {        
        let scores_list: Vec<u32> = self.scores_to_vec();
        match scores_list.len(){
            _x if _x == 1 && _x > 0 => scores_list[..1].to_vec(),
            _x if _x <= 2 && _x > 1 => scores_list[..2].to_vec(),
            _x if _x >= 3 => scores_list[..3].to_vec(),
            _ => vec![],
        }
    }
}

#[test]
fn list_of_scores() {
    let expected = [30, 50, 20, 70];
    let high_scores = HighScores::new(&expected);
    assert_eq!(high_scores.scores(), &expected);
}
#[test]
fn latest_score() {
    let high_scores = HighScores::new(&[100, 0, 90, 30]);
    assert_eq!(high_scores.latest(), Some(30));
}
#[test]
fn latest_score_empty() {
    let high_scores = HighScores::new(&[]);
    assert_eq!(high_scores.latest(), None);
}
#[test]
fn personal_best() {
    let high_scores = HighScores::new(&[40, 100, 70]);
    assert_eq!(high_scores.personal_best(), Some(100));
}
#[test]
fn personal_best_empty() {
    let high_scores = HighScores::new(&[]);
    assert_eq!(high_scores.personal_best(), None);
}
#[test]
fn personal_top_three() {
    let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);
    assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
}
#[test]
fn personal_top_three_highest_to_lowest() {
    let high_scores = HighScores::new(&[20, 10, 30]);
    assert_eq!(high_scores.personal_top_three(), vec![30, 20, 10]);
}
#[test]
fn personal_top_three_with_tie() {
    let high_scores = HighScores::new(&[40, 20, 40, 30]);
    assert_eq!(high_scores.personal_top_three(), vec![40, 40, 30]);
}
#[test]
fn personal_top_three_with_less_than_three_scores() {
    let high_scores = HighScores::new(&[30, 70]);
    assert_eq!(high_scores.personal_top_three(), vec![70, 30]);
}
#[test]
fn personal_top_three_only_one_score() {
    let high_scores = HighScores::new(&[40]);
    assert_eq!(high_scores.personal_top_three(), vec![40]);
}
#[test]
fn personal_top_three_empty() {
    let high_scores = HighScores::new(&[]);
    assert!(high_scores.personal_top_three().is_empty());
}