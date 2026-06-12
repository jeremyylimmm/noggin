use crate::*;

const INPUT_SIZE: usize = 768;
const HL0_SIZE: usize = 128; 

type TypeW0 = [[f32;HL0_SIZE/2];INPUT_SIZE];
type TypeB0 = [f32;HL0_SIZE/2];
type TypeW1 = [[f32;1];HL0_SIZE];
type TypeB1 = [f32;1];

const W0: TypeW0 = load_model().0;
const B0: TypeB0 = load_model().1;
const W1: TypeW1 = load_model().2;
const B1: TypeB1 = load_model().3;

fn crelu(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

fn compute_a0_half(bb: &[u64], a0_half: &mut [f32], side: Side) {
    for p in 0..12 {
        let mut x = bb[p];

        while x != 0 {
            let sq = x.trailing_zeros() as usize;

            let sq_index = if side == Side::White {sq} else {sq^56};
            let p_index = if side == Side::White {p} else {(p+6)%12};

            let i = p_index*64+sq_index;

            for j in 0..a0_half.len() {
                a0_half[j] += W0[i][j];
            }

            x &= x-1;
        }
    }

    for i in 0..a0_half.len() {
        a0_half[i] = crelu(a0_half[i]+B0[i]);
    }
} 

pub fn compute(bb: &[u64]) -> f32 {
    let mut a0 = [0.0;HL0_SIZE];
    compute_a0_half(bb, &mut a0[0..HL0_SIZE/2], Side::White);
    compute_a0_half(bb, &mut a0[HL0_SIZE/2..], Side::Black);

    let mut a1 = B1;
    for j in 0..a1.len() {
        for i in 0..a0.len() {
            a1[j] += W1[i][j] * a0[i];
        }
    }

    a1[0] * 400.0
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
    // 2. Load your file into the wrapper
    let raw = include_bytes!("../raw.bin");

    let mut w0 = std::mem::MaybeUninit::uninit();
    let mut b0 = std::mem::MaybeUninit::uninit();
    let mut w1 = std::mem::MaybeUninit::uninit();
    let mut b1 = std::mem::MaybeUninit::uninit();

    let mut cursor = 0;

    unsafe {
        read_object(&mut w0, raw, &mut cursor);
        read_object(&mut b0, raw, &mut cursor);
        read_object(&mut w1, raw, &mut cursor);
        read_object(&mut b1, raw, &mut cursor);
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