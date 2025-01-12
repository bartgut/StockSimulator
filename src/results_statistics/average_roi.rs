pub fn average_return_of_investment(rois: Vec<f32>) -> f32 {
    rois.iter().sum::<f32>() / rois.iter().len() as f32
}