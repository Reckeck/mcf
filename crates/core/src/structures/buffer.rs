use std::mem;

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Buffer {
    OwnedU8(Vec<u8>),
    OwnedI16(Vec<i16>),
    OwnedI32(Vec<i32>),
    OwnedF32(Vec<f32>),
}

impl Buffer {
    pub fn len_elems(&self) -> usize {
        match self {
            Buffer::OwnedU8(v) => v.len(),
            Buffer::OwnedI16(v) => v.len(),
            Buffer::OwnedI32(v) => v.len(),
            Buffer::OwnedF32(v) => v.len(),
        }
    }

    pub fn len_bytes(&self) -> usize {
        match self {
            Buffer::OwnedU8(v) => v.len(),
            Buffer::OwnedI16(v) => v.len() * mem::size_of::<i16>(),
            Buffer::OwnedI32(v) => v.len() * mem::size_of::<i32>(),
            Buffer::OwnedF32(v) => v.len() * mem::size_of::<f32>(),
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        match self {
            Buffer::OwnedU8(v) => v.as_ptr(),
            Buffer::OwnedI16(v) => v.as_ptr() as *const u8,
            Buffer::OwnedI32(v) => v.as_ptr() as *const u8,
            Buffer::OwnedF32(v) => v.as_ptr() as *const u8,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        match self {
            Buffer::OwnedU8(v) => v.as_mut_ptr(),
            Buffer::OwnedI16(v) => v.as_mut_ptr() as *mut u8,
            Buffer::OwnedI32(v) => v.as_mut_ptr() as *mut u8,
            Buffer::OwnedF32(v) => v.as_mut_ptr() as *mut u8,
        }
    }
}
