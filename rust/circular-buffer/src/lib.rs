pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    used: usize,
    writes: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

fn index_then_increment(counter: &mut usize, modulo: usize) -> usize {
    let idx = *counter % modulo;
    *counter = (*counter).wrapping_add(1);
    idx
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            data: (0..capacity).map(|_| None).collect(),
            used: 0,
            writes: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.used.wrapping_add(self.data.len()) == self.writes
    }

    pub fn is_empty(&self) -> bool {
        self.writes == self.used
    }

    pub fn clear(&mut self) {
        self.used = 0;
        self.writes = 0;
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::FullBuffer)
        } else {
            let idx = index_then_increment(&mut self.writes, self.data.len());
            self.data[idx].replace(element);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            let idx = index_then_increment(&mut self.used, self.data.len());
            Ok(self.data[idx].take().unwrap())
        }
    }

    pub fn overwrite(&mut self, element: T) {
        let idx = if self.is_full() {
            index_then_increment(&mut self.writes, self.data.len());
            index_then_increment(&mut self.used, self.data.len())
        } else {
            index_then_increment(&mut self.writes, self.data.len())
        };
        self.data[idx].replace(element);
    }
}
