use std;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpotifyId(u128);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpotifyIdError;

const BASE62_DIGITS: &'static [u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &'static [u8] = b"0123456789abcdef";

// Shameless copy from byteorder because he doesn't expose this macro
macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => ({
        assert!($size == ::std::mem::size_of::<$ty>());
        assert!($size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                $src.as_ptr(),
                &mut data as *mut $ty as *mut u8,
                $size);
        }
        data.$which()
    });
}

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $n:expr, $dst:expr, $which:ident) => ({
        assert!($size <= $dst.len());
        unsafe {
            // N.B. https://github.com/rust-lang/rust/issues/22776
            let bytes = *(&$n.$which() as *const _ as *const [u8; $size]);
            ::std::ptr::copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
        }
    });
}

impl SpotifyId {
    pub fn from_base16(id: &str) -> Result<SpotifyId, SpotifyIdError> {
        let data = id.as_bytes();

        let mut n: u128 = 0;
        for c in data {
            let d = match BASE16_DIGITS.iter().position(|e| e == c) {
                None => return Err(SpotifyIdError),
                Some(x) => x as u128,
            };
            n = n * 16;
            n = n + d;
        }

        Ok(SpotifyId(n))
    }

    pub fn from_base62(id: &str) -> Result<SpotifyId, SpotifyIdError> {
        let data = id.as_bytes();

        let mut n: u128 = 0;
        for c in data {
            let d = match BASE62_DIGITS.iter().position(|e| e == c) {
                None => return Err(SpotifyIdError),
                Some(x) => x as u128,
            };
            n = n * 62;
            n = n + d;
        }

        Ok(SpotifyId(n))
    }

    pub fn from_raw(data: &[u8]) -> Result<SpotifyId, SpotifyIdError> {
        if data.len() != 16 {
            return Err(SpotifyIdError);
        };

        let id = read_num_bytes!(u128, 16, &data[0..16], to_be);

        Ok(SpotifyId(id))
    }

    pub fn to_base16(&self) -> String {
        let &SpotifyId(ref n) = self;

        let mut data = [0u8; 32];
        for i in 0..32 {
            data[31 - i] = BASE16_DIGITS[(n.wrapping_shr(4 * i as u32)) as usize];
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_base62(&self) -> String {
        let &SpotifyId(mut n) = self;

        let mut data = [0u8; 22];
        let sixty_two = 62;
        for i in 0..22 {
            data[21 - i] = BASE62_DIGITS[(n % sixty_two) as usize];
            n /= sixty_two;
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_raw(&self) -> [u8; 16] {
        let &SpotifyId(ref n) = self;

        let mut data = [0u8; 16];

        write_num_bytes!(u128, 16, n, &mut data[0..16], to_be);

        data
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub [u8; 20]);

impl FileId {
    pub fn to_base16(&self) -> String {
        self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .concat()
    }
}

impl fmt::Debug for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("FileId").field(&self.to_base16()).finish()
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_base16())
    }
}
