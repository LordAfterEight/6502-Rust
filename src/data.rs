pub struct ASCII {
    A: u8, a: u8, exclamation: u8,
    B: u8, b: u8, quotation: u8,
    C: u8, c: u8, hashtag: u8,
    D: u8, d: u8, dollar: u8,
    E: u8, e: u8, bracket_1: u8,
    F: u8, f: u8, bracket_2: u8,
    G: u8, g: u8, slash: u8,
    H: u8, h: u8, less_than: u8,
    I: u8, i: u8, greater_than: u8,
    J: u8, j: u8, equal: u8,
    K: u8, k: u8, hyphen: u8,
    L: u8, l: u8, wave: u8,
    M: u8, m: u8, colon: u8,
    N: u8, n: u8, underscore: u8,
    O: u8, o: u8, vertical_bar: u8,
    P: u8, p: u8, ampersand: u8,
    Q: u8, q: u8, question: u8,
    R: u8, r: u8, at: u8,
    S: u8, s: u8,
    T: u8, t: u8,
    U: u8, u: u8,
    V: u8, v: u8,
    W: u8, w: u8,
    X: u8, x: u8,
    Y: u8, y: u8,
    Z: u8, z: u8
}

impl ASCII {
    pub fn init() -> ASCII {
        let ascii = ASCII {
            A: 0x41, a: 0x61, exclamation: 0x21,
            B: 0x42, b: 0x62, quotation: 0x22,
            C: 0x43, c: 0x63, hashtag: 0x23,
            D: 0x44, d: 0x64, dollar: 0x24,
            E: 0x45, e: 0x65, bracket_1: 0x5B,
            F: 0x46, f: 0x66, bracket_2: 0x5D,
            G: 0x47, g: 0x67, slash: 0x5C,
            H: 0x48, h: 0x68, less_than: 0x3C,
            I: 0x49, i: 0x69, greater_than: 0x3E,
            J: 0x4A, j: 0x6A, equal: 0x3D,
            K: 0x4B, k: 0x6B, hyphen: 0x2D,
            L: 0x4C, l: 0x6C, wave: 0x7E,
            M: 0x4D, m: 0x6D, colon: 0x3A,
            N: 0x4E, n: 0x6E, underscore: 0x5F,
            O: 0x4F, o: 0x6F, vertical_bar: 0x7C,
            P: 0x50, p: 0x70, ampersand: 0x26,
            Q: 0x51, q: 0x71, question: 0x3F,
            R: 0x52, r: 0x72, at: 0x40,
            S: 0x53, s: 0x73,
            T: 0x54, t: 0x74,
            U: 0x55, u: 0x75,
            V: 0x56, v: 0x76,
            W: 0x57, w: 0x77,
            X: 0x58, x: 0x78,
            Y: 0x59, y: 0x79,
            Z: 0x5A, z: 0x7A
        };
        return ascii
    }
}
