use anyhow::anyhow;
use rand::seq::SliceRandom;

pub struct SequentialVec<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> SequentialVec<T> {
    pub fn new(data: Vec<T>) -> anyhow::Result<Self> {
        if data.is_empty() {
            Err(anyhow!("Cannot create SequentialVec from empty vec"))
        } else {
            Ok(Self { data, index: 0 })
        }
    }

    pub fn random(&self) -> &T {
        self.data.choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn next_value(&mut self) -> &T {
        if self.index >= self.data.len() {
            self.index = 0;
            &self.data[0]
        } else {
            let idx = self.index;
            self.index += 1;
            &self.data[idx]
        }
    }
}
