use std::error::Error;
use std::fs::File;
use csv::Writer;

pub trait SaveVecToCsv {
    fn save_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>>;
}

impl SaveVecToCsv for Vec<f32> {
    fn save_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path)?;
        let mut wtr = Writer::from_writer(file);

        for &value in self {
            wtr.serialize(value)?;
        }

        wtr.flush()?;
        Ok(())
    }
}

impl SaveVecToCsv for Vec<(Vec<f32>, f32)> {
    fn save_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path)?;
        let mut wtr = Writer::from_writer(file);

        for (params, result) in self {
            let row: Vec<_> = params
                .iter()
                .map(|p| p.to_string())
                .chain(Some(result.to_string()))
                .collect();

            wtr.write_record(&row)?;
        }

        wtr.flush()?;
        Ok(())
    }
}