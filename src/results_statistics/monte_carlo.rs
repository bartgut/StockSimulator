use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn monte_carlo_simulation(rois: Vec<f32>, num_simulations: usize, how_much_to_pick: usize) -> Vec<f32> {
    let mut rng = thread_rng();
    let mut outcome = vec![];

    for _ in 0..num_simulations {
        let picked_stocks: Vec<&f32> = rois.choose_multiple(&mut rng, how_much_to_pick).collect();
        let picked_stocks_roi: f32 =  picked_stocks.into_iter().sum();
        outcome.push(picked_stocks_roi);
    }
    return outcome
}