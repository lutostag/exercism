pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    front: usize,
    back: usize,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            data: (0..capacity).map(|_| None).collect(),
            front: 0,
            back: 0,
            size: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.size == self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.back = 0;
        self.front = 0;
        self.size = 0;
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::FullBuffer)
        } else {
            self.data[self.front].replace(element);
            self.front = (self.front + 1) % self.data.len();
            self.size += 1;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            let value = self.data[self.back].take().unwrap();
            self.back = (self.back + 1) % self.data.len();
            self.size -= 1;
            Ok(value)
        }
    }

    pub fn overwrite(&mut self, element: T) {
        let mut idx = self.front;
        self.front = (self.front + 1) % self.data.len();
        if self.is_full() {
            idx = self.back;
            self.back = (self.back + 1) % self.data.len();
        } else {
            self.size += 1;
        }
        self.data[idx].replace(element);
    }
}
