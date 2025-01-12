pub fn number_of_profitable_investments(rois: Vec<f32>, initial_investment: f32) -> usize {
    rois.iter().filter(|&&a| a > initial_investment).count()
}