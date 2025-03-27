use ringbuf::LocalRb;
use ringbuf::storage::Heap;
use ringbuf::traits::{Consumer, RingBuffer};

pub struct Rsi {
    last_prices_ring_buffer: LocalRb<Heap<f32>>

}

#[derive(Clone)]
pub struct RsiResult {
    pub rsi_line: f32
}

impl Rsi {
    pub fn new(length: usize) -> Self {
        Rsi {
            last_prices_ring_buffer: LocalRb::<Heap<f32>>::new(length+1)
        }
    }

    pub fn next(&mut self, price: f32) -> RsiResult {
        self.last_prices_ring_buffer.push_overwrite(price);
        let (losses, gains): (Vec<f32>, Vec<f32>) = self.last_prices_ring_buffer.iter()
            .zip(self.last_prices_ring_buffer.iter().skip(1))
            .map(|(&a, &b)| b - a)
            .partition(|&diff| diff < 0.0f32);

        let rs = gains.iter().sum::<f32>() / losses.iter().sum::<f32>();
        let rsi = 100.0 - 100.0/(1.0 + rs);

        RsiResult {
            rsi_line: rsi
        }

    }
}

