use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use sha2::{Digest, Sha256};

const BUFFER_SIZE: usize = 8192;

struct Sha256Hasher<R> {
    inner: Sha256,
    reader: R,
    buffer_size: usize,
}

#[allow(dead_code)]
impl<R: Read> Sha256Hasher<R> {
    pub fn new(reader: R) -> Self {
        Self { inner: Sha256::new(), reader, buffer_size: BUFFER_SIZE }
    }

    pub fn buffer_size(mut self, n: usize) -> Self {
        self.buffer_size = n;
        self
    }

    pub fn calculate(mut self) -> anyhow::Result<sha2::digest::Output<Sha256>> {
        let mut buffer = vec![0; self.buffer_size];
        loop {
            let n = self.reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            self.inner.update(&buffer[..n]);
        }
        Ok(self.inner.finalize())
    }

    pub fn hex(self) -> anyhow::Result<String> {
        Ok(format!("{:x}", self.calculate()?))
    }
}

pub fn calculate_file_sha256<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    Sha256Hasher::new(buf).hex()
}

pub fn calculate_url_sha256(url: &str) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?;
    Sha256Hasher::new(response).hex()
}
