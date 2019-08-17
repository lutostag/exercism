use std::io::{Read, Result, Write};

pub struct TransferStats<T> {
    wrapped: T,
    bytes: usize,
    calls: usize,
}

pub use TransferStats as ReadStats;
pub use TransferStats as WriteStats;

impl<T> TransferStats<T> {
    pub fn new(wrapped: T) -> ReadStats<T> {
        ReadStats {
            wrapped,
            bytes: 0,
            calls: 0,
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    fn called(&mut self, result: Result<usize>) -> Result<usize> {
        self.calls += 1;
        if let Ok(size) = result {
            self.bytes += size;
        }
        result
    }
}

impl<R: Read> TransferStats<R> {
    pub fn reads(&self) -> usize {
        self.calls
    }
}

impl<W: Write> TransferStats<W> {
    pub fn writes(&self) -> usize {
        self.calls
    }
}

impl<R: Read> Read for TransferStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.wrapped.read(buf);
        self.called(result)
    }
}

impl<W: Write> Write for TransferStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.wrapped.write(buf);
        self.called(result)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
