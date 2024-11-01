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
