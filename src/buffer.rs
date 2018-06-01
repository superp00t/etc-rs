extern crate num_bigint;
extern crate num_traits;

use std::f32;
use std::f64;
use std::string;
use backend_mem;
use backend_file;
use std::io;
use self::num_traits::{FromPrimitive, ToPrimitive};
use self::num_bigint::{BigUint, BigInt};

pub trait Backend {
    fn close(&mut self);
    fn flush(&mut self) -> io::Result<()>;
    fn finish(&mut self) -> Vec<u8>;
    fn wpos(&mut self) -> i64;
    fn rpos(&mut self) -> i64;

    fn size(&mut self) -> i64;

    fn seekw(&mut self, v: i64);
    fn seekr(&mut self, v: i64);

    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

pub struct Buffer {
    backend: Box<Backend>,
}

impl io::Write for Buffer {
  fn write(&mut self, data: &[u8]) -> io::Result<usize> {
    return self.write(data);
  }

  fn flush(&mut self) -> io::Result<()> {
    return self.flush();
  } 
}

impl io::Read for Buffer {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    return self.read(buf);
  }
}

impl Buffer {
    pub fn new() -> Buffer {
        let buf = backend_mem::BackendMem::new();
        return Buffer{
            backend: Box::new(buf)
        }
    }

    pub fn from_vec(v: Vec<u8>) -> Buffer {
        let mut buf = Buffer::new();
        buf.write(v.as_slice());
        return buf;
    }

    pub fn from_file(v: &str) -> Buffer {
        let buf = backend_file::BackendFile::new(v);
        return Buffer{
            backend: Box::new(buf)
        }
    }

    pub fn seekw(&mut self, v: i64) {
        self.backend.seekw(v);
    }

    pub fn seekr(&mut self, v: i64) {
        self.backend.seekr(v);
    }

    pub fn wpos(&mut self) -> i64 {
        return self.backend.wpos();
    }

    pub fn rpos(&mut self) -> i64 {
        return self.backend.rpos();
    }

    pub fn size(&mut self) -> i64 {
        return self.backend.size();
    }

    pub fn finish(&mut self) -> Vec<u8> {
        return self.backend.finish();
    }

    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        return self.backend.read(buf);
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        return self.backend.write(buf);
    }

    /* Integer serialization/deserialization
    *
    *    (Unless suffixed with _be,
    *     All encoding is Little-endian)
    */
    pub fn write_u8(&mut self, v: u8) {
        self.backend.write(&[v]);
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.backend.read(&mut buf);

        return buf[0];
    }

    pub fn write_u64(&mut self, v: u64) {
        let mut buf = [0u8; 8];

        buf[0] = (v)       as u8;
        buf[1] = (v >> 8)  as u8;
        buf[2] = (v >> 16) as u8;
        buf[3] = (v >> 24) as u8;
        buf[4] = (v >> 32) as u8;
        buf[5] = (v >> 40) as u8;
        buf[6] = (v >> 48) as u8;
        buf[7] = (v >> 56) as u8;

        self.backend.write(&mut buf);
    }

    pub fn write_i64(&mut self, v: i64) {
        let mut buf = [0u8; 8];

        buf[0] = (v)       as u8;
        buf[1] = (v >> 8)  as u8;
        buf[2] = (v >> 16) as u8;
        buf[3] = (v >> 24) as u8;
        buf[4] = (v >> 32) as u8;
        buf[5] = (v >> 40) as u8;
        buf[6] = (v >> 48) as u8;
        buf[7] = (v >> 56) as u8;

        self.backend.write(&mut buf);
    }

    pub fn write_u32(&mut self, v: u32) {
        let mut buf = [0u8; 4];

        buf[0] = (v)       as u8;
        buf[1] = (v >> 8)  as u8;
        buf[2] = (v >> 16) as u8;
        buf[3] = (v >> 24) as u8;

        self.backend.write(&mut buf);
    }

    pub fn write_i32(&mut self, v: i32) {
        let mut buf = [0u8; 4];

        buf[0] = (v)       as u8;
        buf[1] = (v >> 8)  as u8;
        buf[2] = (v >> 16) as u8;
        buf[3] = (v >> 24) as u8;

        self.backend.write(&mut buf);
    }

    pub fn write_u16(&mut self, v: u16) {
        let mut buf = [0u8; 2];

        buf[0] = (v)       as u8;
        buf[1] = (v >> 8)  as u8;

        self.backend.write(&mut buf);
    }

    pub fn write_i16(&mut self, v: i16) {
        let mut buf = [0u8; 2];

        buf[0] = (v)       as u8;
        buf[1] = (v >> 8)  as u8;

        self.backend.write(&mut buf);
    }

    // Deserialization functions
    pub fn read_u64(&mut self) -> u64 {
        let mut b = [0u8; 8];
        self.backend.read(&mut b);

        (b[0] as u64)        |
        (b[1] as u64) << 8   |
        (b[2] as u64) << 16  |
        (b[3] as u64) << 24  |
        (b[4] as u64) << 32  |
        (b[5] as u64) << 40  |
        (b[6] as u64) << 48  |
        (b[7] as u64) << 56
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut b = [0u8; 4];
        self.backend.read(&mut b);

        (b[0] as u32)        |
        (b[1] as u32) << 8   |
        (b[2] as u32) << 16  |
        (b[3] as u32) << 24 
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut b = [0u8; 2];
        self.backend.read(&mut b);

        (b[0] as u16)        |
        (b[1] as u16) << 8 
    }

    pub fn read_i64(&mut self) -> i64 {
        let mut b = [0u8; 8];
        self.backend.read(&mut b);

        (b[0] as i64)        |
        (b[1] as i64) << 8   |
        (b[2] as i64) << 16  |
        (b[3] as i64) << 24  |
        (b[4] as i64) << 32  |
        (b[5] as i64) << 40  |
        (b[6] as i64) << 48  |
        (b[7] as i64) << 56
    }

    pub fn read_i32(&mut self) -> i32 {
        let mut b = [0u8; 4];
        self.backend.read(&mut b);

        (b[0] as i32)        |
        (b[1] as i32) << 8   |
        (b[2] as i32) << 16  |
        (b[3] as i32) << 24 
    }

    pub fn read_i16(&mut self) -> i16 {
        let mut b = [0u8; 2];
        self.backend.read(&mut b);

        (b[0] as i16)        |
        (b[1] as i16) << 8   
    }

    pub fn write_ubig(&mut self, v: num_bigint::BigUint) {
        let mut x = v.clone();
        
        let zro: BigUint = FromPrimitive::from_u64(0).unwrap();
        let max: BigUint = FromPrimitive::from_u64(0x80).unwrap();

        while x > max {
            self.write_u8(x.to_u64().unwrap() as u8 | 0x80);
            x = x >> 7;
        }

        self.write_u8(x.to_u64().unwrap() as u8);
    }

    pub fn write_ibig(&mut self, v: num_bigint::BigInt) {
        let mut x = v.clone();
        
        let zro: BigInt = FromPrimitive::from_i64(0).unwrap();
        let neg: BigInt = FromPrimitive::from_i64(-1).unwrap();

        let sign: BigInt = FromPrimitive::from_i64(0x7F).unwrap();
        
        while true {
            let y = x.clone();
            let mut byte = (y & sign.clone()).to_u64().unwrap() as u8;
            x = x >> 7;
            if ((x == zro && byte & 0x40 == 0) || (x == neg && byte & 0x40 != 0)) == false {
                byte |= 0x80;
            }

            self.write_u8(byte as u8);
            if byte & 0x80 == 0 {
                break;
            }
        }
    }

    pub fn read_ubig(&mut self, limit: i32) -> num_bigint::BigUint {
        let mut byte = self.read_u8();

        if byte < 128 {
            return FromPrimitive::from_u64(byte as u64).unwrap();
        }

        let mut value = FromPrimitive::from_u64((byte & 0x7F) as u64).unwrap();
        let mut shift: usize = 7;

        let mut off: i32 = 0;

        while byte >= 128 {
            off += 1;
            if limit > 0 {
              if off > limit {
                  return FromPrimitive::from_u64(0).unwrap();
              }
            }
            byte = self.read_u8();
            let part: BigUint = FromPrimitive::from_u64((byte & 0x7F) as u64).unwrap();
            value = value | (part << shift);
            shift += 7;
        }

        return value;
    }

    pub fn read_ibig(&mut self, limit: i32) -> num_bigint::BigInt {
        let mut res: BigInt = FromPrimitive::from_i64(0).unwrap();
        let mut more = true;
        let mut val: i32 = 0;
        let mut off: i32 = 0;
        let mut shift: usize = 0;

        while more {
            let byte = self.read_u8();
            val = (byte & 0x7F) as i32;

            let part: BigInt = FromPrimitive::from_i64(val as i64).unwrap();
            res = res | (part << shift);

            shift += 7;
            more = (byte & 0x80) >> 7 != 0;

            off += 1;

            if limit > 0 {
              if off > limit {
                  return FromPrimitive::from_i64(0).unwrap();
              }
            }
        }

        let pt: BigInt = FromPrimitive::from_i64(1).unwrap();

        let mut ux = res.clone();
        let mut nd = res & (pt << shift-1);

        if nd != FromPrimitive::from_i64(0).unwrap() {
            let sign: BigInt = FromPrimitive::from_i64((1 << shift) as i64).unwrap();
            return ux - sign;
        }

        return ux;
    }

    pub fn write_i(&mut self, v: i64) {
        self.write_ibig(FromPrimitive::from_i64(v).unwrap());
    }

    pub fn write_u(&mut self, v: u64) {
        self.write_ubig(FromPrimitive::from_u64(v).unwrap());
    }

    pub fn read_i(&mut self) -> i64 {
        return self.read_ibig(20).to_i64().unwrap();
    }

    pub fn read_u(&mut self) -> u64 {
        return self.read_ubig(20).to_u64().unwrap();
    }

    pub fn write_string(&mut self, s: string::String) {
        let arr = s.as_bytes();
        self.write_u(arr.len() as u64);
        self.write(arr);
    }

    pub fn read_string(&mut self) -> string::String {
        let sz = self.read_u() as usize;
        let mut bytes = vec![0u8;sz];
        self.read(&mut bytes);

        return string::String::from_utf8(bytes).unwrap();
    }

    pub fn write_f32(&mut self, v: f32) {
        self.write_u32(v.to_bits());
    }

    pub fn write_f64(&mut self, v: f64) {
        self.write_u64(v.to_bits());
    }

    pub fn read_f32(&mut self) -> f32 {
        return f32::from_bits(self.read_u32());
    }

    pub fn read_f64(&mut self) -> f64 {
        return f64::from_bits(self.read_u64());
    }

    pub fn read_bool(&mut self) -> bool {
        return self.read_u8() == 1;
    }

    pub fn write_bool(&mut self, v: bool)  {
        if v {
            self.write_u8(1);
        } else {
            self.write_u8(0);
        }
    }

    pub fn write_cstring(&mut self, v: string::String) {
        self.write(v.as_bytes());
        self.write_u8(0);
    }

    pub fn read_cstring(&mut self) -> string::String {
        let mut svec = vec![];

        while true {
            let byte = self.read_u8();
            if byte == 0 {
                break;
            }
            svec.push(byte);    
        }

        return string::String::from_utf8(svec).unwrap();
    }
}