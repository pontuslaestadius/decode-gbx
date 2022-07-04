use std::io::Read;

pub fn read_gbx_or_panic<T: Read>(
    readable: &mut T,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut buffer: [u8; 3] = [0; 3];
    readable.read(&mut buffer)?;
    if buffer != [71, 66, 88] {
        panic!("Doesn't start with magic 'GBX' string ([71, 66, 88])")
    }
    Ok(())
}

pub fn read_u16<T: Read>(readable: &mut T) -> Result<u16, Box<dyn std::error::Error + 'static>> {
    let mut buffer: [u8; 2] = [0; 2];
    readable.read(&mut buffer)?;
    let res: u16 = sum_bytes_u16(&buffer);
    Ok(res)
}

pub fn read_u32<T: Read>(readable: &mut T) -> Result<u32, Box<dyn std::error::Error + 'static>> {
    let mut buffer: [u8; 4] = [0; 4];
    readable.read(&mut buffer)?;
    let res: u32 = sum_bytes_u32(&buffer);
    Ok(res)
}

// Convert byte to uint32 by shifting the bytes.
// Since we use little-endian for GBX, we don't use .rev() here.
pub fn sum_bytes_u16(buf: &[u8]) -> u16 {
    let mut val: u16 = 0;
    for (idx, data) in buf.iter().enumerate() {
        val += (*data as u16) << (8 * idx)
    }
    val
}

pub fn sum_bytes_u32(buf: &[u8]) -> u32 {
    let mut val: u32 = 0;
    for (idx, data) in buf.iter().enumerate() {
        val += (*data as u32) << (8 * idx)
    }
    val
}

pub fn read_fixed_vec<T: Read>(
    readable: &mut T,
    size: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
    let mut vec: Vec<u8> = fixed_vec_len(size);
    readable.read(&mut vec)?;
    Ok(vec)
}

pub fn fixed_vec_len<T>(len: usize) -> Vec<T> {
    let mut vec: Vec<T> = Vec::with_capacity(len);
    unsafe {
        vec.set_len(len);
    }
    vec
}
