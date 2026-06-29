use std::io::Write;

const MASK_FILE_A: u64 = 0x0101010101010101;
const MASK_FILE_H: u64 = 0x8080808080808080;

const MASK_RANK_1: u64 = 0x00000000000000ff;
const MASK_RANK_8: u64 = 0xff00000000000000;

fn main() {
    generate_sliding_attack_tables();
}

fn slide<F: Fn(u64) -> u64>(mut cur: u64, f: F, blockers: u64) -> u64 {
    let mut acc = 0;

    loop {
        cur = f(cur);

        if cur == 0 {
            break;
        }

        acc |= cur;

        if cur & blockers != 0 {
            break;
        }
    }

    acc
}

fn compute_magic_index(mask: u64, magic: u64, shift: u32) -> usize {
    (mask.overflowing_mul(magic).0 >> shift) as _
}

fn magic_good(mask: u64, magic: u64, uses: &mut [u64]) -> bool {
    let mut perm = mask;
    let shift = 64 - mask.count_ones();

    uses.fill(0);

    loop {
        let index = compute_magic_index(perm, magic, shift);

        if uses[index / 64] & (1u64 << (index % 64)) != 0 {
            return false;
        }

        uses[index / 64] |= 1u64 << (index % 64);

        if perm == 0 {
            break;
        }

        perm = (perm - 1) & mask;
    }

    true
}

fn find_magic(mask: u64) -> u64 {
    let mut rng = PCG32::new(9238, 3);

    let bits = mask.count_ones();
    let n = 1 << bits;
    let mut uses = vec![0u64; (n + 63) / 64];

    loop {
        let magic = rng.next64() & rng.next64() & rng.next64();

        if magic_good(mask, magic, &mut uses) {
            return magic;
        }
    }
}

fn rook_mask(sq: usize) -> u64 {
    let cur = 1u64 << sq;

    let left = slide(cur, |bb| (bb >> 1) & !MASK_FILE_H, 0);
    let right = slide(cur, |bb| (bb << 1) & !MASK_FILE_A, 0);

    let up = slide(cur, |bb| bb << 8, 0);
    let down = slide(cur, |bb| bb >> 8, 0);

    let mut hor = left | right;
    let mut ver = up | down;

    hor &= !(MASK_FILE_A | MASK_FILE_H);
    ver &= !(MASK_RANK_1 | MASK_RANK_8);

    hor | ver
}

fn rook_attacks(sq: usize, blockers: u64) -> u64 {
    let cur = 1u64 << sq;

    let left = slide(cur, |bb| (bb >> 1) & !MASK_FILE_H, blockers);
    let right = slide(cur, |bb| (bb << 1) & !MASK_FILE_A, blockers);

    let up = slide(cur, |bb| bb << 8, blockers);
    let down = slide(cur, |bb| bb >> 8, blockers);

    left | right | up | down
}

fn bishop_attacks(sq: usize, blockers: u64) -> u64 {
    let cur = 1u64 << sq;

    let left_up = slide(cur, |bb| (bb << 7) & !MASK_FILE_H, blockers);
    let right_up = slide(cur, |bb| (bb << 9) & !MASK_FILE_A, blockers);
    let right_down = slide(cur, |bb| (bb >> 7) & !MASK_FILE_A, blockers);
    let left_down = slide(cur, |bb| (bb >> 9) & !MASK_FILE_H, blockers);

    left_up | right_up | right_down | left_down
}

fn bishop_mask(sq: usize) -> u64 {
    let edges = MASK_FILE_A | MASK_FILE_H | MASK_RANK_1 | MASK_RANK_8;
    bishop_attacks(sq, 0) & !edges
}

fn generate_sliding_attack_tables() {
    let mut file = std::fs::File::create("src/generated/sliding_attacks.rs").unwrap();
    generate_sliding_attack_table(&mut file, "ROOK", rook_mask, rook_attacks);
    generate_sliding_attack_table(&mut file, "BISHOP", bishop_mask, bishop_attacks);
}

fn generate_sliding_attack_table<M: Fn(usize) -> u64, A: Fn(usize, u64) -> u64>(
    file: &mut std::fs::File,
    name: &str,
    get_mask: M,
    get_attacks: A,
) {
    let mut magics = [0; 64];
    let mut masks = [0; 64];
    let mut shifts = [0; 64];

    for sq in 0..64 {
        let mask = get_mask(sq);

        let magic = find_magic(mask);
        let bits = mask.count_ones();
        let shift = 64 - bits;
        let n = 1 << bits;

        let mut table = vec![0u64; n];

        let mut perm = mask;

        loop {
            let index = compute_magic_index(perm, magic, shift);
            table[index] = get_attacks(sq, perm);

            if perm == 0 {
                break;
            }

            perm = (perm - 1) & mask;
        }

        write!(file, "const {}_TABLE_SQ_{}: [u64; {}] = [\n", name, sq, n).unwrap();

        for i in 0..n {
            if i % 8 == 0 {
                write!(file, "    ").unwrap();
            }

            write!(file, "0x{:x}, ", table[i]).unwrap();

            if (i + 1) % 8 == 0 || i == (n - 1) {
                write!(file, "\n").unwrap();
            }
        }

        write!(file, "];\n\n").unwrap();

        magics[sq] = magic;
        masks[sq] = mask;
        shifts[sq] = shift;
    }

    write_table(file, &format!("{}_MAGICS", name), "u64", &magics);
    write_table(file, &format!("{}_MASKS", name), "u64", &masks);
    write_table(file, &format!("{}_SHIFTS", name), "u32", &shifts);

    write!(file, "pub const {}_TABLES: [&[u64]; 64] = [\n", name).unwrap();
    for sq in 0..64 {
        write!(file, "    &{}_TABLE_SQ_{},\n", name, sq).unwrap();
    }
    write!(file, "];\n\n").unwrap();
}

fn write_table<T: std::fmt::LowerHex>(file: &mut std::fs::File, name: &str, ty: &str, table: &[T]) {
    write!(file, "pub const {}: [{}; {}] = [\n", name, ty, table.len()).unwrap();

    for i in 0..table.len() {
        if i % 8 == 0 {
            write!(file, "    ").unwrap();
        }

        write!(file, "0x{:x}, ", table[i]).unwrap();

        if (i + 1) % 8 == 0 || i == (table.len() - 1) {
            write!(file, "\n").unwrap();
        }
    }

    write!(file, "];\n\n").unwrap();
}

struct PCG32 {
    state: u64,
    inc: u64,
}

impl PCG32 {
    pub fn new(state: u64, inc: u64) -> Self {
        let mut s = Self { state, inc };
        let _ = s.next64();
        s
    }

    fn next32(&mut self) -> u32 {
        let oldstate = self.state;
        // Advance internal state
        self.state = oldstate.overflowing_mul(6364136223846793005u64).0 + (self.inc | 1);
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot: i32 = (oldstate >> 59) as i32;
        return (xorshifted >> rot) | (xorshifted << ((-rot) & 31));
    }

    fn next64(&mut self) -> u64 {
        return (self.next32() as u64) | (self.next32() as u64) << 32;
    }
}
