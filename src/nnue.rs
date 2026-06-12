use crate::*;

const INPUT_SIZE: usize = 768;
const HL0_SIZE: usize = 128; 

type TypeW0 = [[i16;HL0_SIZE/2];INPUT_SIZE];
type TypeB0 = [i16;HL0_SIZE/2];
type TypeW1 = [[i16;1];HL0_SIZE];
type TypeB1 = [i16;1];

const W0: TypeW0 = load_model().0;
const B0: TypeB0 = load_model().1;
const W1: TypeW1 = load_model().2;
const B1: TypeB1 = load_model().3;

const QA: isize = 255;
const QB: isize = 64;

#[derive(Clone, PartialEq, Debug)]
pub struct Accumulator([i32;HL0_SIZE]);

impl Accumulator {
    pub fn piece<const SIGN: i32>(&mut self, piece: Piece, sq: usize, side: Side) {
        let p_white = piece.id().unwrap()+side.id()*6;
        let p_black = (p_white + 6) % 12;

        let sq_white = sq;
        let sq_black = sq_white ^ 56;

        let feature_white = p_white*64+sq_white;
        let feature_black = p_black*64+sq_black;

        for j in 0..HL0_SIZE/2 {
            self.0[j] += SIGN * W0[feature_white][j] as i32;
            self.0[j+HL0_SIZE/2] += SIGN * W0[feature_black][j] as i32;
        }
    }

    pub fn new(bb: &[u64]) -> Self {
        let mut data = [0;HL0_SIZE];

        compute_y0_half(bb, &mut data[..HL0_SIZE/2], Side::White);
        compute_y0_half(bb, &mut data[HL0_SIZE/2..], Side::Black);

        Self(
            data
        )
    }
    
    pub fn forward(&self) -> i32 {
        forward(&self.0)
    }
}

fn screlu(x: i32) -> i32 {
    let y = x.clamp(0, QA as _);
    (y as i32)*(y as i32)
}

fn compute_y0_half(bb: &[u64], y0_half: &mut [i32], side: Side) {
    for p in 0..12 {
        let mut x = bb[p];

        while x != 0 {
            let sq = x.trailing_zeros() as usize;

            let sq_index = if side == Side::White {sq} else {sq^56};
            let p_index = if side == Side::White {p} else {(p+6)%12};

            let i = p_index*64+sq_index;

            for j in 0..y0_half.len() {
                y0_half[j] += W0[i][j] as i32;
            }

            x &= x-1;
        }
    }
} 

pub fn forward(y0: &[i32;HL0_SIZE]) -> i32 {
    let a0: [i32;HL0_SIZE] = std::array::from_fn(|i|{
        screlu(y0[i] + B0[i%(HL0_SIZE/2)] as i32)
    });

    let mut a1: [i32;1] = [0;_];

    for j in 0..a1.len() {
        for i in 0..a0.len() {
            a1[j] += (W1[i][j] as i32) * a0[i];
        }
    }

    a1[0] /= QA as i32;
    a1[0] += B1[0] as i32;
    a1[0] *= 400;
    a1[0] /= QA as i32 * QB as i32;

    a1[0]
}

pub fn compute(bb: &[u64]) -> i32 {
    let mut y0 = [0;HL0_SIZE];

    compute_y0_half(bb, &mut y0[0..HL0_SIZE/2], Side::White);
    compute_y0_half(bb, &mut y0[HL0_SIZE/2..], Side::Black);

    forward(&y0)
}

const unsafe fn read_object<T>(x: &mut T, buffer: &[u8], cursor: &mut usize) {
    let required: usize = std::mem::size_of::<T>();

    assert!(*cursor + required <= buffer.len());

    unsafe {
        let src = buffer.as_ptr().byte_add(*cursor) as *const u8;
        let dst = x as *mut T as *mut u8;
        std::ptr::copy_nonoverlapping(src, dst, required); 
    }

    *cursor += required;
}

const fn load_model() -> (TypeW0, TypeB0, TypeW1, TypeB1) {
    let raw = include_bytes!("../quantised.bin");

    let mut w0 = std::mem::MaybeUninit::uninit();
    let mut b0 = std::mem::MaybeUninit::uninit();
    let mut w1 = std::mem::MaybeUninit::uninit();
    let mut b1 = std::mem::MaybeUninit::uninit();

    let mut cursor = 0;

    unsafe {
        read_object(&mut w0, raw, &mut cursor);
        cursor = ((cursor + 63) / 64) * 64;

        read_object(&mut b0, raw, &mut cursor);
        cursor = ((cursor + 63) / 64) * 64;
        
        read_object(&mut w1, raw, &mut cursor);
        cursor = ((cursor + 63) / 64) * 64;

        read_object(&mut b1, raw, &mut cursor);
        cursor = ((cursor + 63) / 64) * 64;
    }

    assert!(cursor == raw.len());

    unsafe{
        (
            w0.assume_init(),
            b0.assume_init(),
            w1.assume_init(),
            b1.assume_init()
        )
    }
}