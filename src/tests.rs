#[allow(dead_code)]
use buffer;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_wpos() {
    let mut b = buffer::Buffer::new();
    assert_eq!(b.wpos(), 0);
  }

  fn uleb_enc_ass(tst: u64, v: Vec<u8>) {
    let mut b = buffer::Buffer::new();
    b.write_u(tst);
    assert_eq!(b.finish(), v);

    assert_eq!(b.read_u(), tst);
  }

  fn sleb_enc_ass(tst: i64, v: Vec<u8>) {
    let mut b = buffer::Buffer::new();
    b.write_i(tst);
    assert_eq!(b.finish(), v);

    assert_eq!(b.read_i(), tst);
  }

  #[test]
  fn test_floats() {
    let ff32 = 12323.34434123;
    let ff64 = 56565.434123123;
    let mut b = buffer::Buffer::new();

    b.write_f32(ff32);
    b.write_f64(ff64);

    assert_eq!(b.read_f32(), ff32);
    assert_eq!(b.read_f64(), ff64);
  }

  #[test]
  fn test_leb128() {
    //  Unsigned
		uleb_enc_ass(812301234, vec![0xB2, 0xF7, 0xAA, 0x83, 0x03]);
		uleb_enc_ass(234728934709283749, vec![0xA5, 0x97, 0xE9, 0x9E, 0x8F, 0x97, 0xFB, 0xA0, 0x03]);
		uleb_enc_ass(4784583489573475384, vec![0xB8, 0xA8, 0xE7, 0xF1, 0xD7, 0xAA, 0x90, 0xB3, 0x42]);
		uleb_enc_ass(0, vec![0x00]);
		uleb_enc_ass(6545, vec![0x91, 0x33]);
		uleb_enc_ass(4233260, vec![0xAC, 0xB0, 0x82, 0x02]);

		//  Unsigned
		sleb_enc_ass(812301234, vec![0xB2, 0xF7, 0xAA, 0x83, 0x03]);
		sleb_enc_ass(234728934709283749, vec![0xA5, 0x97, 0xE9, 0x9E, 0x8F, 0x97, 0xFB, 0xA0, 0x03]);
		sleb_enc_ass(4784583489573475384, vec![0xB8, 0xA8, 0xE7, 0xF1, 0xD7, 0xAA, 0x90, 0xB3, 0xC2, 0x00]);
		sleb_enc_ass(0, vec![0x00]);
		sleb_enc_ass(6545, vec![0x91, 0x33]);
		sleb_enc_ass(4233260, vec![0xAC, 0xB0, 0x82, 0x02]);
		sleb_enc_ass(-1212312, vec![0xE8, 0x80, 0xB6, 0x7F]);

  }

  #[test]
  fn test_u64() {
    let mut b = buffer::Buffer::new();
    let bits = 0xFFFFFFFFFFFFFFFF;
    b.write_u64(bits);

    assert_eq!(b.finish(), vec![255, 255, 255, 255, 255, 255, 255, 255]);

    assert_eq!(b.read_u64(), bits);
  }

  #[test]
  fn test_write() {
    let mut b = buffer::Buffer::new();
    b.write(&[10, 10, 10]);
    assert_eq!(b.finish(), vec![10,10,10]);
  }

  #[test]
  fn test_write_wpos() {
    let mut b = buffer::Buffer::new();
    b.write(&[1, 0, 0, 10]);
    assert_eq!(b.wpos(), 4);
  }

  #[test]
  fn test_string() {
    let tstr = "一つの妖怪がヨーロッパにあらわれている、――共産主義の妖怪が。";
    let mut b = buffer::Buffer::new();
    b.write_string(String::from(tstr));
    assert_eq!(b.read_string(), tstr);

    b.write_cstring(String::from(tstr));
    assert_eq!(b.read_cstring(), tstr);
  }
}
