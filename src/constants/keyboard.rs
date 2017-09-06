pub const KEYBOARD_INTERRUPT: u8 = 0x21;
pub const PORT: u16 = 0x60;

#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub lower: char,
    pub upper: char,
    pub scancode: u8,
}

pub const ZERO_KEY: Key = Key {
    lower: '0',
    upper: ')',
    scancode: 0x29,
};
pub const ONE_KEY: Key = Key {
    lower: '1',
    upper: '!',
    scancode: 0x2,
};
pub const TWO_KEY: Key = Key {
    lower: '2',
    upper: '@',
    scancode: 0x3,
};
pub const THREE_KEY: Key = Key {
    lower: '3',
    upper: '#',
    scancode: 0x4,
};
pub const FOUR_KEY: Key = Key {
    lower: '4',
    upper: '$',
    scancode: 0x5,
};
pub const FIVE_KEY: Key = Key {
    lower: '5',
    upper: '%',
    scancode: 0x6,
};
pub const SIX_KEY: Key = Key {
    lower: '6',
    upper: '^',
    scancode: 0x7,
};
pub const SEVEN_KEY: Key = Key {
    lower: '7',
    upper: '&',
    scancode: 0x8,
};
pub const EIGHT_KEY: Key = Key {
    lower: '8',
    upper: '*',
    scancode: 0x9,
};
pub const NINE_KEY: Key = Key {
    lower: '9',
    upper: '(',
    scancode: 0xA,
};
pub const DASH_KEY: Key = Key {
    lower: '-',
    upper: '_',
    scancode: 0xC,
};
pub const EQUAL_KEY: Key = Key {
    lower: '=',
    upper: '+',
    scancode: 0xD,
};
pub const DELETE_KEY: Key = Key {
    lower: ' ',
    upper: ' ',
    scancode: 0xE,
};
pub const TAB_KEY: Key = Key {
    lower: '\t',
    upper: '\t',
    scancode: 0xF,
};
pub const Q_KEY: Key = Key {
    lower: 'q',
    upper: 'Q',
    scancode: 0x10,
};
pub const W_KEY: Key = Key {
    lower: 'w',
    upper: 'W',
    scancode: 0x11,
};
pub const E_KEY: Key = Key {
    lower: 'e',
    upper: 'E',
    scancode: 0x12,
};
pub const R_KEY: Key = Key {
    lower: 'r',
    upper: 'R',
    scancode: 0x13,
};
pub const T_KEY: Key = Key {
    lower: 't',
    upper: 'T',
    scancode: 0x14,
};
pub const Y_KEY: Key = Key {
    lower: 'y',
    upper: 'Y',
    scancode: 0x15,
};
pub const U_KEY: Key = Key {
    lower: 'u',
    upper: 'U',
    scancode: 0x16,
};
pub const I_KEY: Key = Key {
    lower: 'i',
    upper: 'I',
    scancode: 0x17,
};
pub const O_KEY: Key = Key {
    lower: 'o',
    upper: 'O',
    scancode: 0x18,
};
pub const P_KEY: Key = Key {
    lower: 'p',
    upper: 'P',
    scancode: 0x19,
};
pub const LB_KEY: Key = Key {
    lower: '[',
    upper: '{',
    scancode: 0x1A,
};
pub const RB_KEY: Key = Key {
    lower: ']',
    upper: '}',
    scancode: 0x1B,
};
pub const ENTER_KEY: Key = Key {
    lower: '\r',
    upper: '\r',
    scancode: 0x1C,
};
pub const A_KEY: Key = Key {
    lower: 'a',
    upper: 'A',
    scancode: 0x1E,
};
pub const S_KEY: Key = Key {
    lower: 's',
    upper: 'S',
    scancode: 0x1F,
};
pub const D_KEY: Key = Key {
    lower: 'd',
    upper: 'D',
    scancode: 0x20,
};
pub const F_KEY: Key = Key {
    lower: 'f',
    upper: 'F',
    scancode: 0x21,
};
pub const G_KEY: Key = Key {
    lower: 'g',
    upper: 'G',
    scancode: 0x22,
};
pub const H_KEY: Key = Key {
    lower: 'h',
    upper: 'H',
    scancode: 0x23,
};
pub const J_KEY: Key = Key {
    lower: 'j',
    upper: 'J',
    scancode: 0x24,
};
pub const K_KEY: Key = Key {
    lower: 'k',
    upper: 'K',
    scancode: 0x25,
};
pub const L_KEY: Key = Key {
    lower: 'l',
    upper: 'L',
    scancode: 0x26,
};
pub const TILDE_KEY: Key = Key {
    lower: '`',
    upper: '~',
    scancode: 0x29,
};
pub const BACKSLASH_KEY: Key = Key {
    lower: '\\',
    upper: '|',
    scancode: 0x2B,
};
pub const Z_KEY: Key = Key {
    lower: 'z',
    upper: 'Z',
    scancode: 0x2C,
};
pub const X_KEY: Key = Key {
    lower: 'x',
    upper: 'X',
    scancode: 0x2D,
};
pub const C_KEY: Key = Key {
    lower: 'c',
    upper: 'C',
    scancode: 0x2E,
};
pub const V_KEY: Key = Key {
    lower: 'v',
    upper: 'V',
    scancode: 0x2F,
};
pub const B_KEY: Key = Key {
    lower: 'b',
    upper: 'B',
    scancode: 0x30,
};
pub const N_KEY: Key = Key {
    lower: 'n',
    upper: 'N',
    scancode: 0x31,
};
pub const M_KEY: Key = Key {
    lower: 'm',
    upper: 'M',
    scancode: 0x32,
};
pub const COMMA_KEY: Key = Key {
    lower: ',',
    upper: '<',
    scancode: 0x33,
};
pub const DOT_KEY: Key = Key {
    lower: '.',
    upper: '>',
    scancode: 0x34,
};
pub const SLASH_KEY: Key = Key {
    lower: '/',
    upper: '?',
    scancode: 0x35,
};
pub const SPACE_KEY: Key = Key {
    lower: ' ',
    upper: ' ',
    scancode: 0x39,
};

pub const KEYS: [Option<Key>; 128] = [// 0x0
                                      None,
                                      None,
                                      Some(ONE_KEY),
                                      Some(TWO_KEY),
                                      Some(THREE_KEY),
                                      Some(FOUR_KEY),
                                      Some(FIVE_KEY),
                                      Some(SIX_KEY), // 0x7
                                      // 0x8
                                      Some(SEVEN_KEY),
                                      Some(EIGHT_KEY),
                                      Some(NINE_KEY),
                                      Some(ZERO_KEY),
                                      Some(DASH_KEY),
                                      Some(EQUAL_KEY),
                                      Some(DELETE_KEY),
                                      Some(TAB_KEY), // 0xF
                                      // 0x10
                                      Some(Q_KEY),
                                      Some(W_KEY),
                                      Some(E_KEY),
                                      Some(R_KEY),
                                      Some(T_KEY),
                                      Some(Y_KEY),
                                      Some(U_KEY),
                                      Some(I_KEY), // 0x17
                                      // 0x18
                                      Some(O_KEY),
                                      Some(P_KEY),
                                      Some(LB_KEY),
                                      Some(RB_KEY),
                                      Some(ENTER_KEY),
                                      None,
                                      Some(A_KEY),
                                      Some(S_KEY), // 0x1F
                                      // 0x20
                                      Some(D_KEY),
                                      Some(F_KEY),
                                      Some(G_KEY),
                                      Some(H_KEY),
                                      Some(J_KEY),
                                      Some(K_KEY),
                                      Some(L_KEY),
                                      None, // 0x27
                                      // 0x28
                                      None,
                                      Some(TILDE_KEY),
                                      None,
                                      Some(BACKSLASH_KEY),
                                      Some(Z_KEY),
                                      Some(X_KEY),
                                      Some(C_KEY),
                                      Some(V_KEY), // 0x2F
                                      // 0x30
                                      Some(B_KEY),
                                      Some(N_KEY),
                                      Some(M_KEY),
                                      Some(COMMA_KEY),
                                      Some(DOT_KEY),
                                      Some(SLASH_KEY),
                                      None,
                                      None, // 0x37
                                      // 0x38
                                      None,
                                      Some(SPACE_KEY),
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None, // 0x3F
                                      // 0x40
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None, // 0x47
                                      // 0x48
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None, // 0x4F
                                      // 0x50
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None, // 0x57
                                      // 0x58
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None, // 0x5F
                                      // 0x60
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None, // 0x67
                                      // 0x68
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None, // 0x6F
                                      // 0x70
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None, // 0x77
                                      // 0x78
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None /* 0x7F */];
