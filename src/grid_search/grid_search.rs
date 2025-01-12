use crate::grid_search::parameter::Parameter;

pub struct GridSearch {
    parameters: Vec<Parameter>
}

impl GridSearch {
    pub fn new(parameters: Vec<Parameter>) -> Self {
        GridSearch { parameters }
    }

    pub fn search<F, T>(&self, mut f: F) -> Vec<(Vec<f32>, T)>
        where F: FnMut(&[f32]) -> T,
              T: std::fmt::Debug {
        let mut results = Vec::new();
        let mut values = vec![0.0; self.parameters.len()];

        fn grid_recursive<F, T>(
            params: &[Parameter],
            values: &mut [f32],
            depth: usize,
            f: &mut F,
            results: &mut Vec<(Vec<f32>, T)>
        ) where F: FnMut(&[f32]) -> T,
        {
            if depth == params.len() {
                let result = f(values);
                results.push((values.to_vec(), result))
            } else {
                for &val in &params[depth].values() {
                    values[depth] = val;
                    grid_recursive(params, values, depth + 1, f, results);
                }
            }
        }

        grid_recursive(&self.parameters, &mut values, 0, &mut f, &mut results);

        for (config, result) in results.iter().clone() {
            println!("Config: {:?} => Result: {:?}", config, result);
        }
        results
    }
}