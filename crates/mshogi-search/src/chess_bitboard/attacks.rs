//! Attack tables for the Chess search position: leapers, pawns and magic sliders.
//!
//! The tables are built one time at first use from fixed magic constants. Bit 0 is a1 and bit 63 is h8, the same map
//! as the native `Position::attacks_from`.
use std::sync::LazyLock;

pub(crate) const WHITE: usize = 0;
pub(crate) const BLACK: usize = 1;

struct Magic {
    mask: u64,
    magic: u64,
    shift: u32,
    offset: usize,
}

pub(crate) struct Tables {
    pub(crate) knight: [u64; 64],
    pub(crate) king: [u64; 64],
    pub(crate) pawn: [[u64; 64]; 2],
    rook: Vec<Magic>,
    bishop: Vec<Magic>,
    sliders: Vec<u64>,
}

const ROOK_STEPS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_STEPS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

fn leaper(sq: usize, steps: &[(i8, i8)]) -> u64 {
    let (file, rank) = ((sq % 8) as i8, (sq / 8) as i8);
    let mut map = 0;
    for &(df, dr) in steps {
        let (f, r) = (file + df, rank + dr);
        if (0..8).contains(&f) && (0..8).contains(&r) {
            map |= 1u64 << (r * 8 + f);
        }
    }
    map
}

/// Ray attacks up to and including the first occupied square in each direction.
fn slide(sq: usize, occupied: u64, steps: &[(i8, i8); 4]) -> u64 {
    let (file, rank) = ((sq % 8) as i8, (sq / 8) as i8);
    let mut map = 0;
    for &(df, dr) in steps {
        let (mut f, mut r) = (file + df, rank + dr);
        while (0..8).contains(&f) && (0..8).contains(&r) {
            let bit = 1u64 << (r * 8 + f);
            map |= bit;
            if occupied & bit != 0 {
                break;
            }
            f += df;
            r += dr;
        }
    }
    map
}

/// Relevant blocker squares: the rays without their last (edge) square.
fn relevant(sq: usize, steps: &[(i8, i8); 4]) -> u64 {
    let (file, rank) = ((sq % 8) as i8, (sq / 8) as i8);
    let mut map = 0;
    for &(df, dr) in steps {
        let (mut f, mut r) = (file + df, rank + dr);
        while (0..8).contains(&(f + df)) && (0..8).contains(&(r + dr)) {
            map |= 1u64 << (r * 8 + f);
            f += df;
            r += dr;
        }
    }
    map
}

/// Magic multipliers found one time with a fixed-seed search; the test
/// `magic_lookups_equal_direct_ray_walks` proves each one.
const ROOK_MAGICS: [u64; 64] = [
    0x1080004008801020,
    0x0840092002C03000,
    0x1900200010400900,
    0x0880100008000480,
    0x4200100420080200,
    0x8100020100080400,
    0x0200040110886200,
    0x0200008040220411,
    0x0404800084400220,
    0x0000401000402000,
    0x0086001081220440,
    0x0408800800100280,
    0x000A001201040820,
    0x8848800200840080,
    0x4001000100040200,
    0x0442000102105084,
    0x9080010020804100,
    0x0040404000201009,
    0x0000808010002009,
    0x2200090021D00100,
    0x0008008008040080,
    0x0004004002010040,
    0x0011040008015042,
    0x00000A0001768104,
    0x0000800080204009,
    0x2010004140002001,
    0x9800200280100080,
    0x1000100080080080,
    0x0442000A00049020,
    0x2100040080020080,
    0x0800120400900148,
    0x0010040A00128541,
    0x2800804000800030,
    0x1010002000400041,
    0x4000200011004100,
    0x0610008410800800,
    0x0400802402800800,
    0xC100020080800400,
    0x0002000802000401,
    0x0182085882000401,
    0x0220204000808000,
    0x2860100040024022,
    0x0001002004110040,
    0x99101042000A0020,
    0x0004080004008080,
    0x0010040002008080,
    0x2012004881020004,
    0x8300842444820011,
    0x0088403882010200,
    0x0820400080210100,
    0x0110910040A00300,
    0x0801100280080480,
    0x0242009008200600,
    0x1002000489500200,
    0x0040800200010080,
    0x0091800041000080,
    0x0000209300488001,
    0x04C1002414824001,
    0x020020000B001041,
    0x7000100004200901,
    0x8002002004100802,
    0x30010002084C0007,
    0x0888221800813004,
    0x4000002840840112,
];

/// Magic multipliers found one time with a fixed-seed search; the test
/// `magic_lookups_equal_direct_ray_walks` proves each one.
const BISHOP_MAGICS: [u64; 64] = [
    0xA010041108003100,
    0x006082020A002900,
    0x6810010619200000,
    0x08281A0520000408,
    0x0001104001000400,
    0x0018901008048400,
    0x00040A0210245280,
    0x000200210808A402,
    0x9140048410821200,
    0x0800091010820041,
    0x20504804832202C0,
    0x0100091401081000,
    0x8021011140000012,
    0x0810020804450400,
    0x208B0542109008A2,
    0x0080084A08040204,
    0x0040E2A80811244C,
    0x2505022008008108,
    0x0430220100420040,
    0x010A040420220040,
    0x1105000290400000,
    0x0093001200822120,
    0x4000A62048043004,
    0x280120048A015004,
    0x006090002A020814,
    0x44042000240800D0,
    0x01102800040A4400,
    0x1004080080220040,
    0x0001001011004024,
    0x0010044000805040,
    0x0914041200820100,
    0x0004821012821480,
    0x0024040500C05021,
    0x0088611002080200,
    0x0116080A00040020,
    0x4000020080080080,
    0x2450450140840040,
    0x0000880201484100,
    0x0222020404020092,
    0x8081110600002E00,
    0x2842101105000801,
    0x1100809008001025,
    0x00020202221C0400,
    0x0422014022009020,
    0x0210046102100C00,
    0xC004008082029102,
    0x00AA461801101200,
    0x0404080080201108,
    0x020542108C205002,
    0x0410544804100100,
    0x0040910841100000,
    0x0400200042021100,
    0x00004204850400C0,
    0x0200100410A42102,
    0x1040020801210102,
    0x0805040410420000,
    0x2884804130100200,
    0x800C262201242000,
    0x1058000194108800,
    0x0014221054420204,
    0x0104000012A02200,
    0x0200881003300100,
    0x0140400202840100,
    0x0402020801010201,
];

fn fill_magics(steps: &[(i8, i8); 4], magics: &[u64; 64], sliders: &mut Vec<u64>) -> Vec<Magic> {
    let mut result = Vec::with_capacity(64);
    for (sq, &magic) in magics.iter().enumerate() {
        let mask = relevant(sq, steps);
        let shift = 64 - mask.count_ones();
        let offset = sliders.len();
        sliders.resize(offset + (1usize << mask.count_ones()), 0);
        let mut subset = 0u64;
        loop {
            let index = (subset.wrapping_mul(magic) >> shift) as usize;
            sliders[offset + index] = slide(sq, subset, steps);
            subset = subset.wrapping_sub(mask) & mask;
            if subset == 0 {
                break;
            }
        }
        result.push(Magic {
            mask,
            magic,
            shift,
            offset,
        });
    }
    result
}

pub(crate) static TABLES: LazyLock<Tables> = LazyLock::new(|| {
    let mut knight = [0; 64];
    let mut king = [0; 64];
    let mut pawn = [[0; 64]; 2];
    for sq in 0..64 {
        knight[sq] = leaper(
            sq,
            &[
                (1, 2),
                (2, 1),
                (2, -1),
                (1, -2),
                (-1, -2),
                (-2, -1),
                (-2, 1),
                (-1, 2),
            ],
        );
        king[sq] = leaper(
            sq,
            &[
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ],
        );
        pawn[WHITE][sq] = leaper(sq, &[(1, 1), (-1, 1)]);
        pawn[BLACK][sq] = leaper(sq, &[(1, -1), (-1, -1)]);
    }
    let mut sliders = Vec::new();
    let rook = fill_magics(&ROOK_STEPS, &ROOK_MAGICS, &mut sliders);
    let bishop = fill_magics(&BISHOP_STEPS, &BISHOP_MAGICS, &mut sliders);
    Tables {
        knight,
        king,
        pawn,
        rook,
        bishop,
        sliders,
    }
});

impl Tables {
    #[inline(always)]
    pub(crate) fn rook(&self, sq: usize, occupied: u64) -> u64 {
        let m = &self.rook[sq];
        self.sliders[m.offset + ((occupied & m.mask).wrapping_mul(m.magic) >> m.shift) as usize]
    }
    #[inline(always)]
    pub(crate) fn bishop(&self, sq: usize, occupied: u64) -> u64 {
        let m = &self.bishop[sq];
        self.sliders[m.offset + ((occupied & m.mask).wrapping_mul(m.magic) >> m.shift) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_lookups_equal_direct_ray_walks() {
        let t = &*TABLES;
        let mut state = 7u64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for sq in 0..64 {
            for _ in 0..2_000 {
                let occupied = next() & next();
                assert_eq!(t.rook(sq, occupied), slide(sq, occupied, &ROOK_STEPS));
                assert_eq!(t.bishop(sq, occupied), slide(sq, occupied, &BISHOP_STEPS));
            }
            assert_eq!(t.rook(sq, 0).count_ones(), 14);
        }
        assert_eq!(t.knight[0], (1 << 10) | (1 << 17));
        assert_eq!(t.pawn[WHITE][8], 1 << 17);
        assert_eq!(t.pawn[BLACK][55], 1 << 46);
    }
}
