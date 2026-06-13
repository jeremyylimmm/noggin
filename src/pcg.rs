pub struct PCG32 {
    state: u64,
    inc: u64,
}

impl PCG32 {
    pub fn new(state: u64, inc: u64) -> Self {
        let mut s = Self { state, inc };

        let _ = s.next64();

        s
    }

    pub fn next32(&mut self) -> u32 {
        let oldstate = self.state;
        // Advance internal state
        self.state = oldstate.overflowing_mul(6364136223846793005u64).0 + (self.inc | 1);
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot: i32 = (oldstate >> 59) as i32;
        return (xorshifted >> rot) | (xorshifted << ((-rot) & 31));
    }

    pub fn next64(&mut self) -> u64 {
        return (self.next32() as u64) | (self.next32() as u64) << 32;
    }

    pub fn range(&mut self, lo: usize, hi: usize) -> usize {
        let x = self.next32() as u64;
        let r = (hi - lo) as u64;
        (x * r / (1u64 << 32) as u64) as usize + lo
    }
}
