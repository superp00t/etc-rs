use std::io::Write;
use std::io::Read;
use std::io::Seek;
use std::io;
use buffer;
use std::fs;
use std::string::*;

pub struct BackendFile {
  w:    i64,
  r:    i64,
  path: String,
  file: fs::File,
}

impl BackendFile {
  pub fn new(path: &str) -> BackendFile {
    let mut file;

    let exists = fs::metadata(path).is_ok();
    if exists {
      let res = match fs::File::open(path) {
        Ok(res) => res,
        Err(res) => panic!("could not open file")
      };

      file = res;
    } else {
      let res = match fs::File::open(path) {
        Ok(res) => res,
        Err(res) => panic!("could not open file")
      };

      file = res;
    }

    BackendFile {
      w:    0,
      r:    0,
      path: String::from(path),
      file: file,
    }
  }

  fn flush(&mut self) -> io::Result<()> {
    self.w = 0;
    self.r = 0;
    self.file.set_len(0);

    Ok(())
  }

  fn write(&mut self, data: &[u8]) -> io::Result<usize> {
    self.file.seek(io::SeekFrom::Start(self.w as u64));
    self.w += data.len() as i64;
    return self.file.write(data);
  }

  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    self.file.seek(io::SeekFrom::Start(self.r as u64));
    return self.file.read(buf);
  }
}

impl buffer::Backend for BackendFile {
  fn flush(&mut self) -> io::Result<()> {
    return self.flush();
  } 

  fn wpos(&mut self) -> i64 {
    self.w
  }

  fn rpos(&mut self) -> i64 {
    self.r
  }

  /**
   We don't need to actually seek here.
   That is done before every call to read and write
  */
  fn seekw(&mut self, v: i64) {
    self.w = v
  }

  fn seekr(&mut self, v: i64) {
    self.r = v
  }

  fn size(&mut self) -> i64 {
    fs::metadata(self.path.clone()).unwrap().len() as i64
  }

  fn finish(&mut self) -> Vec<u8> {
    let curoff: i64 = self.r;

    self.seekr(0);

    let mut v: Vec<u8> = Vec::new();
    io::copy(self, &mut v);

    self.seekr(curoff);

    return v;
  }

  fn write(&mut self, data: &[u8]) -> io::Result<usize> {
    return self.write(data);
  }

  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    return self.read(buf);
  }

  fn close(&mut self) {
  }
}

impl io::Write for BackendFile {
  fn write(&mut self, data: &[u8]) -> io::Result<usize> {
    return self.write(data);
  }

  fn flush(&mut self) -> io::Result<()> {
    return self.flush();
  } 
}

impl io::Read for BackendFile {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    return self.read(buf);
  }
}