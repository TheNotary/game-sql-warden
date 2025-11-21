#[derive(Debug, Clone)]
pub struct EvaluationRow {
    pub cube_id: i64,
    pub monster_id: i64,
    pub is_correct: bool,
}

#[derive(Debug, Clone)]
pub struct EvaluationResult {
    pub rows: Vec<EvaluationRow>,
    pub all_correct: bool,
}
