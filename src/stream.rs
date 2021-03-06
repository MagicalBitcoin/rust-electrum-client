use log::error;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct ClonableStream<T: Read + Write>(Arc<Mutex<T>>);

impl<T: Read + Write> Read for ClonableStream<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0
            .lock()
            .map_err(|_| {
                error!("Unable to acquire lock on ClonableStream read operation");
                io::Error::from(io::ErrorKind::BrokenPipe)
            })?
            .read(buf)
    }
}

impl<T: Read + Write> Write for ClonableStream<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .map_err(|_| {
                error!("Unable to acquire lock on ClonableStream write operation");
                io::Error::from(io::ErrorKind::BrokenPipe)
            })?
            .write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0
            .lock()
            .map_err(|_| {
                error!("Unable to acquire lock on ClonableStream flush operation");
                io::Error::from(io::ErrorKind::BrokenPipe)
            })?
            .flush()
    }
}

impl<T: Read + Write> From<T> for ClonableStream<T> {
    fn from(stream: T) -> Self {
        Self(Arc::new(Mutex::new(stream)))
    }
}

impl<T: Read + Write> Clone for ClonableStream<T> {
    fn clone(&self) -> Self {
        ClonableStream(Arc::clone(&self.0))
    }
}
