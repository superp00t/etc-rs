use std::io;
use std::vec::*;
use buffer;

pub struct BackendMem {
  w: i64,
  r: i64,
  buf: Vec<u8>,
}

impl BackendMem {
  pub fn new() -> BackendMem {
    BackendMem{
      w: 0,
      r: 0,
      buf: Vec::new(),
    }
  }
}

impl buffer::Backend for BackendMem {
  fn wpos(&mut self) -> i64 {
    self.w
  }

  fn rpos(&mut self) -> i64 {
    self.r
  }

  fn seekw(&mut self, v: i64) {
    self.w = v
  }

  fn seekr(&mut self, v: i64) {
    self.r = v
  }

  fn close(&mut self) {
    self.flush();
  }

  fn size(&mut self) -> i64 {
    self.buf.len() as i64 
  }

  fn flush(&mut self) -> io::Result<()> {
    self.w = 0;
    self.r = 0;
    self.buf.truncate(0);

    Ok(())
  }

  fn finish(&mut self) -> Vec<u8> {
    self.buf.clone()
  }

  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    let mut beenread: usize = 0;
    
    for x in 0..buf.len() {
      if self.rpos() >= self.size() {
        return Ok(beenread);
      }

      buf[x] = self.buf[self.r as usize];
      beenread += 1;
      self.r   += 1;
    }

    Ok(beenread)
  }

  fn write(&mut self, data: &[u8]) -> io::Result<usize> {
    let mut written: usize = 0;

    for byte in data {
      if self.size() == self.wpos() {
        self.buf.push(*byte);
      } else {
        self.buf[self.w as usize] = *byte;
      }
      
      self.w  += 1;
      written += 1;
    }

    Ok(written)
  }
} 