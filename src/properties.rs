pub const AI: u8 = 1;
pub const AL: u8 = 2;
pub const B2: u8 = 3;
pub const BA: u8 = 4;
pub const BB: u8 = 5;
pub const BK: u8 = 6;
pub const CB: u8 = 7;
pub const CJ: u8 = 8;
pub const CL: u8 = 9;
pub const CM: u8 = 10;
pub const CP: u8 = 11;
pub const CR: u8 = 12;
pub const EB: u8 = 13;
pub const EM: u8 = 14;
pub const EX: u8 = 15;
pub const GL: u8 = 16;
pub const H2: u8 = 17;
pub const H3: u8 = 18;
pub const HL: u8 = 19;
pub const HY: u8 = 20;
pub const ID: u8 = 21;
pub const IN: u8 = 22;
pub const IS: u8 = 23;
pub const JL: u8 = 24;
pub const JT: u8 = 25;
pub const JV: u8 = 26;
pub const LF: u8 = 27;
pub const NL: u8 = 28;
pub const NS: u8 = 29;
pub const NU: u8 = 30;
pub const OP: u8 = 31;
pub const PO: u8 = 32;
pub const PR: u8 = 33;
pub const QU: u8 = 34;
pub const RI: u8 = 35;
pub const SA: u8 = 36;
pub const SG: u8 = 37;
pub const SP: u8 = 38;
pub const SY: u8 = 39;
pub const WJ: u8 = 40;
pub const XX: u8 = 41;
pub const ZW: u8 = 42;
pub const ZWJ: u8 = 43;
pub const B2_SP: u8 = 44;
pub const CL_CP_SP: u8 = 45;
pub const CM_CM: u8 = 46;
pub const HL_HY: u8 = 47;
pub const OP_SP: u8 = 48;
pub const QU_SP: u8 = 49;
pub const ZW_SP: u8 = 50;
pub const ZWJ_ZWJ: u8 = 51;
pub const PROP_COUNT: usize = 51;

pub const UAX14_PROPERTIES_0: [u8; 1024] = [
    CM, CM, CM, CM, CM, CM, CM, CM, CM, BA, LF, BK, BK, CR, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, SP, EX, QU, AL, PR, PO, AL, QU, OP, CP, AL, PR, IS, HY, IS, SY,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, IS, IS, AL, AL, AL, EX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, OP, PR, CP, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, OP, BA, CL, AL, CM, CM, CM, CM, CM, CM, NL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, GL, OP, PO, PR, PR, PR, AL, AI,
    AI, AL, AI, QU, AL, BA, AL, AL, PO, PR, AI, AI, BB, AL, AI, AI, AI, AI, AI, QU, AI, AI, AI, OP,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, BB, AI, AI, AI, BB, AI, AL, AL,
    AI, AL, AL, AL, AL, AL, AL, AL, AI, AI, AI, AI, AL, AI, AL, BB, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, GL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, GL, GL, GL, GL,
    GL, GL, GL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, AL, AL, AL, AL, IS, AL, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, XX, AL, XX, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
];

pub const UAX14_PROPERTIES_1: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, IS, BA, XX, XX, AL, AL, PR, XX, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, BA, CM, AL, CM, CM, AL, CM, CM, EX, CM,
    XX, XX, XX, XX, XX, XX, XX, XX, HL, HL, HL, HL, HL, HL, HL, HL, HL, HL, HL, HL, HL, HL, HL, HL,
    HL, HL, HL, HL, HL, HL, HL, HL, HL, HL, HL, XX, XX, XX, XX, HL, HL, HL, HL, AL, AL, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, PO, PO, PO, IS, IS, AL, AL,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, EX, CM, XX, EX, EX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, PO, NU, NU, AL, AL, AL,
    CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, EX, AL, CM, CM, CM, CM, CM, CM, CM, AL, AL, CM, CM, CM, CM, CM, CM, AL, AL, CM,
    CM, AL, CM, CM, CM, CM, AL, AL, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, CM, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, AL, AL, AL, AL, IS, EX, AL, XX, XX, CM, PR, PR,
];

pub const UAX14_PROPERTIES_2: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM,
    CM, CM, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, CM, CM, CM, AL, CM, CM, CM, CM, CM, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, XX, XX, AL, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, CM, CM, CM, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    AL, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, BA, BA, NU, NU,
    NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, CM, CM, CM, XX, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, XX, XX, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL,
    AL, XX, AL, XX, XX, XX, AL, AL, AL, AL, XX, XX, CM, AL, CM, CM, CM, CM, CM, CM, CM, XX, XX, CM,
    CM, XX, XX, CM, CM, CM, AL, XX, XX, XX, XX, XX, XX, XX, XX, CM, XX, XX, XX, XX, AL, AL, XX, AL,
    AL, AL, CM, CM, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, PO, PO, AL, AL, AL, AL,
    AL, PO, AL, PR, AL, AL, CM, XX, XX, CM, CM, CM, XX, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, AL,
    AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, XX, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, XX, AL, AL, XX, AL, AL, XX, XX, CM, XX, CM, CM,
    CM, CM, CM, XX, XX, XX, XX, CM, CM, XX, XX, CM, CM, CM, XX, XX, XX, CM, XX, XX, XX, XX, XX, XX,
    XX, AL, AL, AL, AL, XX, AL, XX, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU,
    CM, CM, AL, AL, AL, CM, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, CM, CM, CM, XX, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, XX, AL, AL, AL,
    AL, AL, XX, XX, CM, AL, CM, CM, CM, CM, CM, CM, CM, CM, XX, CM, CM, CM, XX, CM, CM, CM, XX, XX,
    AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, CM, CM, XX, XX, NU, NU,
    NU, NU, NU, NU, NU, NU, NU, NU, AL, PR, XX, XX, XX, XX, XX, XX, XX, AL, CM, CM, CM, CM, CM, CM,
    XX, CM, CM, CM, XX, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, XX, XX, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL,
    AL, XX, AL, AL, XX, AL, AL, AL, AL, AL, XX, XX, CM, AL, CM, CM, CM, CM, CM, CM, CM, XX, XX, CM,
    CM, XX, XX, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, CM, CM, CM, XX, XX, XX, XX, AL, AL, XX, AL,
    AL, AL, CM, CM, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, CM, AL, XX, AL, AL, AL, AL, AL, AL, XX, XX, XX, AL, AL,
    AL, XX, AL, AL, AL, AL, XX, XX, XX, AL, AL, XX, AL, XX, AL, AL, XX, XX, XX, AL, AL, XX, XX, XX,
    AL, AL, AL, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, CM, CM,
    CM, CM, CM, XX, XX, XX, CM, CM, CM, XX, CM, CM, CM, CM, XX, XX, AL, XX, XX, XX, XX, XX, XX, CM,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, PR, AL, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_3: [u8; 1024] = [
    CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, AL, CM, CM, CM, CM, CM, CM, CM, XX, CM, CM,
    CM, XX, CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, CM, CM, XX, AL, AL, AL, XX, XX, XX, XX, XX,
    AL, AL, CM, CM, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX, XX, BB,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, BB, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL,
    AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, XX, XX, CM, AL, CM, CM,
    CM, CM, CM, CM, CM, XX, CM, CM, CM, XX, CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, CM, CM, XX,
    XX, XX, XX, XX, XX, XX, AL, XX, AL, AL, CM, CM, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU,
    XX, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, CM, CM, CM, CM, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, XX, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, CM, CM, AL, CM, CM, CM, CM, CM, CM, CM, XX, CM, CM, CM, XX, CM, CM, CM, CM, AL, AL,
    XX, XX, XX, XX, AL, AL, AL, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, XX, XX, NU, NU,
    NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL, AL, AL, AL, PO, AL, AL, AL, AL, AL, AL,
    XX, CM, CM, CM, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX,
    XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, XX,
    XX, XX, CM, XX, XX, XX, XX, CM, CM, CM, CM, CM, CM, XX, CM, XX, CM, CM, CM, CM, CM, CM, CM, CM,
    XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, CM, CM, AL, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, XX, XX, XX, XX, PR,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, AL, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, BA, BA, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, SA, SA, XX, SA, XX, SA, SA,
    SA, SA, SA, XX, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, XX, SA, XX, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, XX, XX, SA, SA, SA, SA, SA, XX, SA, XX, SA, SA, SA, SA, SA, SA, XX, XX,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, SA, SA, SA, SA, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, BB, BB, BB, BB, AL, BB, BB, GL, BB, BB, BA, GL, EX, EX, EX, EX, EX, GL, AL, EX, AL, AL, AL,
    CM, CM, AL, AL, AL, AL, AL, AL, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, BA, CM, AL, CM, AL, CM, OP, CL, OP, CL, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, BA, CM, CM, CM, CM, CM, BA, CM, CM, AL, AL, AL, AL, AL, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, XX, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, XX, BA, BA,
    AL, AL, AL, AL, AL, AL, CM, AL, AL, AL, AL, AL, AL, XX, AL, AL, BB, BB, BA, BB, AL, AL, AL, AL,
    AL, GL, GL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_4: [u8; 1024] = [
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, BA, BA, AL, AL, AL, AL, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, SA, SA, SA, SA, SA, SA, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, AL, XX, XX, XX, XX, XX, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, JL, JL, JL, JL, JL, JL, JL, JL,
    JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL,
    JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL,
    JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL,
    JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JV, JV, JV, JV, JV, JV, JV, JV,
    JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV,
    JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV,
    JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JT, JT, JT, JT, JT, JT, JT, JT,
    JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT,
    JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT,
    JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT,
    JT, JT, JT, JT, JT, JT, JT, JT, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, XX,
    AL, XX, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, XX, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, XX, AL, XX, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, CM, CM, CM,
    AL, BA, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, XX, XX,
];

pub const UAX14_PROPERTIES_5: [u8; 1024] = [
    BA, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, BA, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, OP, CL, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, BA, BA, BA, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, CM, CM, CM, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, CM, CM, CM, BA, BA, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, XX, CM, CM, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, BA, BA, NS, SA,
    BA, AL, BA, PR, SA, SA, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_6: [u8; 1024] = [
    AL, AL, EX, EX, BA, BA, BB, AL, EX, EX, AL, CM, CM, CM, GL, XX, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, CM, AL, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, XX, XX, XX, XX, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, XX, XX, XX, XX, AL, XX, XX, XX, EX, EX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, XX, XX, SA, SA, SA, SA, SA, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, XX, XX, XX, XX,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, SA, XX, XX, XX, SA, SA,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, XX, XX, AL, AL, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, XX, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, XX, XX, CM, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, XX, XX, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL,
    AL, AL, AL, AL, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, BA, BA, AL, BA, BA, BA,
    BA, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, XX, XX, XX, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, AL, AL, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL,
];

pub const UAX14_PROPERTIES_7: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, XX, XX, XX, BA, BA, BA, BA, BA, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, XX, XX, XX, AL, AL, AL, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, BA, BA, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, CM, CM, CM, AL, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, CM, AL, AL,
    AL, AL, AL, AL, CM, AL, AL, CM, CM, CM, AL, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, XX, CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX,
    AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX,
    AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, XX, AL, XX, AL, XX, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL,
    AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, BB, AL, XX,
];

pub const UAX14_PROPERTIES_8: [u8; 1024] = [
    BA, BA, BA, BA, BA, BA, BA, GL, BA, BA, BA, ZW, CM, ZWJ, CM, CM, BA, GL, BA, BA, B2, AI, AI,
    AL, QU, QU, OP, QU, QU, QU, OP, QU, AI, AI, AL, AL, IN, IN, IN, BA, BK, BK, CM, CM, CM, CM, CM,
    GL, PO, PO, PO, PO, PO, PO, PO, PO, AL, QU, QU, AI, NS, NS, AL, AL, AL, AL, AL, AL, IS, OP, CL,
    NS, NS, NS, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, BA, AL, BA, BA, BA, BA, AL, BA, BA,
    BA, WJ, AL, AL, AL, AL, XX, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, XX, XX, AI, AL, AL,
    AL, AL, AL, AL, AL, AL, OP, CL, AI, AL, AI, AI, AI, AI, AL, AL, AL, AL, AL, AL, AL, AL, OP, CL,
    XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, PR, PR, PR, PR, PR, PR, PR,
    PO, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PO, PR, PR, PR, PR, PO, PR, PR, PO,
    PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, PR, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, PO, AL, AI, AL,
    AL, AL, PO, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AL, PR, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AI, AI, AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AI, AI, AL, AL, AL, AL, AL, AI, AL, AL, AI, AL, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AL, AL, AL, AL, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AL, XX, XX, XX, XX, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AI, AI, AL, AL, AL, AI, AI, AL, AL, AI, AL, AL, AL,
    AI, AL, AI, PR, PR, AL, AI, AL, AL, AL, AL, AI, AL, AL, AI, AI, AI, AI, AL, AL, AI, AL, AI, AL,
    AI, AI, AI, AI, AI, AI, AL, AI, AL, AL, AL, AL, AL, AI, AI, AI, AI, AL, AL, AL, AL, AI, AI, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AL, AL, AI, AL, AL, AL, AL, AL, AI, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AI, AL, AL, AI, AI, AI, AI, AL, AL, AI, AI, AL, AL, AI,
    AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AI, AL, AL, AI,
    AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AL, AL, AI, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, IN, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, OP, CL, OP, CL, AL, AL, AL, AL, AL, AL, AI, AL, AL, AL, AL,
    AL, AL, AL, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, OP, CL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
];

pub const UAX14_PROPERTIES_9: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AL, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AL, AL, AL, AL,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AL, AL, AI, AI, AI, AI, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AI, AI, AL, AI, AI, AI, AI, AI, AI, AI, AL, AL, AL, AL, AL, AL,
    AL, AL, AI, AI, AL, AL, AI, AI, AL, AL, AL, AL, AI, AI, AL, AL, AI, AI, AL, AL, AL, AL, AI, AI,
    AI, AL, AL, AI, AL, AL, AI, AI, AI, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AI, AI, AI, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, ID, AL, AI, AI, AL, AL, AI, AL, AL, AL, AL, AI, AI,
    AL, AL, AL, AL, ID, ID, AI, AI, ID, AL, ID, ID, ID, EB, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, AL, AL, AL, AL,
    AI, AL, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AI, AI, AL, AI, AI, AI, AL, AI, ID, AI, AI, AL, AI, AI, AL, AI,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, ID, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AI,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AI, AI, AI, AI, ID, AL, ID,
    ID, ID, AI, ID, ID, AI, AI, AI, ID, ID, AI, AI, ID, AI, AI, ID, ID, ID, AL, AI, AL, AL, AL, AL,
    AI, AI, ID, AI, AI, AI, AI, AI, AI, ID, ID, ID, ID, ID, AI, ID, ID, EB, ID, AI, AI, ID, ID, ID,
    ID, ID, ID, ID, ID, AL, AL, AL, ID, ID, EB, EB, EB, EB, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AL, AL, AL, QU, QU, QU, QU, QU,
    QU, AL, EX, EX, ID, AL, AL, AL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, OP, CL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
];

pub const UAX14_PROPERTIES_10: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, OP,
    CL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, OP, CL, OP, CL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, OP, CL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AI, AI, AI, AI, AI, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
];

pub const UAX14_PROPERTIES_11: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM,
    CM, CM, AL, AL, XX, XX, XX, XX, XX, EX, BA, BA, BA, AL, EX, BA, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, AL, XX, XX, XX, XX, XX, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, XX, XX, XX, XX, XX, AL, BA, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, CM,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, XX,
    AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, XX,
    AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, XX,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, QU, QU, QU, QU, QU, QU, QU, QU, QU, QU, QU, QU, QU, QU, BA, BA,
    BA, BA, BA, BA, BA, BA, AL, BA, OP, BA, AL, AL, QU, QU, AL, AL, QU, QU, OP, CL, OP, CL, OP, CL,
    OP, CL, BA, BA, BA, BA, EX, AL, BA, BA, AL, BA, BA, AL, AL, AL, AL, AL, B2, B2, BA, BA, BA, AL,
    BA, BA, OP, BA, BA, BA, BA, BA, BA, BA, BA, AL, BA, AL, BA, BA, AL, AL, AL, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_12: [u8; 1024] = [
    BA, CL, CL, ID, ID, NS, ID, ID, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, ID, ID, OP, CL, OP, CL,
    OP, CL, OP, CL, NS, OP, CL, CL, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, CM, CM, CM, CM, CM, CM,
    ID, ID, ID, ID, ID, CM, ID, ID, ID, ID, ID, NS, NS, ID, ID, ID, XX, CJ, ID, CJ, ID, CJ, ID, CJ,
    ID, CJ, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, CJ, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, CJ, ID, CJ, ID, CJ, ID, ID, ID, ID, ID, ID, CJ, ID,
    ID, ID, ID, ID, ID, CJ, CJ, XX, XX, CM, CM, NS, NS, NS, NS, ID, NS, CJ, ID, CJ, ID, CJ, ID, CJ,
    ID, CJ, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, CJ, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, CJ, ID, CJ, ID, CJ, ID, ID, ID, ID, ID, ID, CJ, ID,
    ID, ID, ID, ID, ID, CJ, CJ, ID, ID, ID, ID, NS, CJ, NS, NS, ID, XX, XX, XX, XX, XX, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, CJ, CJ, CJ, CJ, CJ, CJ, CJ, CJ,
    CJ, CJ, CJ, CJ, CJ, CJ, CJ, CJ, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, AI, AI, AI, AI, AI, AI, AI, AI, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
];

pub const UAX14_PROPERTIES_19: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
];

pub const UAX14_PROPERTIES_40: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, NS, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
];

pub const UAX14_PROPERTIES_41: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, XX, XX,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, BA, BA, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, BA, EX, BA,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM,
    CM, CM, CM, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, AL, BA, BA, BA, BA, BA, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
];

pub const UAX14_PROPERTIES_42: [u8; 1024] = [
    AL, AL, CM, AL, AL, AL, CM, AL, AL, AL, AL, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, AL, AL, AL, AL, CM, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, PO, AL, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, BB, BB, EX, EX,
    XX, XX, XX, XX, XX, XX, XX, XX, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, BA, BA, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, XX, XX, XX, XX, XX, XX, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, BB, AL, AL, CM, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, BA, BA, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, JL, JL, JL, JL, JL, JL, JL, JL,
    JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, JL, XX, XX, XX,
    CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, BA,
    BA, BA, AL, AL, AL, AL, XX, AL, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, AL, AL,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, SA, SA, SA, SA, SA, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, CM, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, XX, XX, AL, BA, BA, BA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, SA, SA, SA, SA, SA, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, CM, CM, CM, CM, CM, BA, BA, AL, AL, AL, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, BA, CM, CM, XX, XX,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_43: [u8; 1024] = [
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
];

pub const UAX14_PROPERTIES_44: [u8; 1024] = [
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
];

pub const UAX14_PROPERTIES_45: [u8; 1024] = [
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
];

pub const UAX14_PROPERTIES_46: [u8; 1024] = [
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
];

pub const UAX14_PROPERTIES_47: [u8; 1024] = [
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
];

pub const UAX14_PROPERTIES_48: [u8; 1024] = [
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
];

pub const UAX14_PROPERTIES_49: [u8; 1024] = [
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
];

pub const UAX14_PROPERTIES_50: [u8; 1024] = [
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
];

pub const UAX14_PROPERTIES_51: [u8; 1024] = [
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
];

pub const UAX14_PROPERTIES_52: [u8; 1024] = [
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
];

pub const UAX14_PROPERTIES_53: [u8; 1024] = [
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H2, H3, H3, H3, H3, H3, H3, H3,
    H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, H3, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV, JV,
    JV, JV, JV, JV, JV, JV, JV, XX, XX, XX, XX, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT,
    JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT,
    JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, JT, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_62: [u8; 1024] = [
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL,
    XX, XX, XX, XX, XX, HL, CM, HL, HL, HL, HL, HL, HL, HL, HL, HL, HL, AL, HL, HL, HL, HL, HL, HL,
    HL, HL, HL, HL, HL, HL, HL, XX, HL, HL, HL, HL, HL, XX, HL, XX, HL, HL, XX, HL, HL, XX, HL, HL,
    HL, HL, HL, HL, HL, HL, HL, HL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
];

pub const UAX14_PROPERTIES_63: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, CL, OP, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, PO, AL, XX, XX, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    IS, CL, CL, IS, IS, EX, EX, OP, CL, IN, XX, XX, XX, XX, XX, XX, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, ID, ID, ID, ID, ID, OP, CL, OP, CL, OP, CL, OP, CL, OP, CL, OP,
    CL, OP, CL, OP, CL, ID, ID, OP, CL, ID, ID, ID, ID, ID, ID, ID, CL, ID, CL, XX, NS, NS, EX, EX,
    ID, OP, CL, OP, CL, OP, CL, ID, ID, ID, ID, ID, ID, ID, ID, XX, ID, PR, PO, ID, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, WJ,
    XX, EX, ID, ID, PR, PO, ID, ID, OP, CL, ID, ID, CL, ID, CL, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, NS, NS, ID, ID, ID, EX, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, OP, ID, CL, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, OP, ID, CL, ID, OP,
    CL, CL, OP, CL, CL, NS, ID, CJ, CJ, CJ, CJ, CJ, CJ, CJ, CJ, CJ, CJ, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, NS, NS, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX,
    XX, XX, ID, ID, ID, ID, ID, ID, XX, XX, ID, ID, ID, ID, ID, ID, XX, XX, ID, ID, ID, ID, ID, ID,
    XX, XX, ID, ID, ID, XX, XX, XX, PO, PR, ID, ID, ID, PR, PR, XX, AL, AL, AL, AL, AL, AL, AL, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, CM, CM, CM, CB, AI, XX, XX,
];

pub const UAX14_PROPERTIES_64: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, BA, BA, BA, XX, XX, XX, XX, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, XX, XX, XX, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, CM, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM,
    CM, CM, CM, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, BA, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, BA, AL, AL, AL, AL, AL, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_65: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_66: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, XX, XX, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, AL, AL, XX, XX, XX, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, BA, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, XX, AL, AL, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, BA,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, XX, XX, XX, XX, XX, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, XX, CM, CM, XX, XX, XX, XX, XX, CM, CM, CM, CM,
    AL, AL, AL, AL, XX, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, CM, CM, CM, XX, XX, XX, XX, CM,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, BA, BA, BA, BA, BA, BA, BA, BA,
    AL, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, XX,
    XX, XX, XX, AL, AL, AL, AL, AL, BA, BA, BA, BA, BA, BA, IN, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, XX, BA, BA, BA, BA, BA, BA, BA, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_67: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, CM, CM, BA, XX, XX, AL, AL, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_68: [u8; 1024] = [
    CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, BA,
    BA, AL, AL, AL, AL, AL, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, BA, BA,
    BA, BA, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX, CM, CM, CM, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, XX, NU, NU,
    NU, NU, NU, NU, NU, NU, NU, NU, BA, BA, BA, BA, AL, CM, CM, AL, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, AL, BB, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, BA, BA, AL,
    BA, CM, CM, CM, CM, AL, CM, CM, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, BB, AL, BA, BA, BA,
    XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, BA, BA, AL, BA, BA, AL, CM, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, XX,
    AL, XX, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, BA, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX,
    CM, CM, CM, CM, XX, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, XX, XX, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL,
    AL, XX, AL, AL, XX, AL, AL, AL, AL, AL, XX, CM, CM, AL, CM, CM, CM, CM, CM, CM, CM, XX, XX, CM,
    CM, XX, XX, CM, CM, CM, XX, XX, AL, XX, XX, XX, XX, XX, XX, CM, XX, XX, XX, XX, XX, AL, AL, AL,
    AL, AL, CM, CM, XX, XX, CM, CM, CM, CM, CM, CM, CM, XX, XX, XX, CM, CM, CM, CM, CM, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_69: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL,
    AL, AL, AL, BA, BA, BA, BA, AL, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, BA, BA, XX, AL, CM, AL,
    AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM,
    CM, CM, CM, CM, CM, CM, XX, XX, CM, CM, CM, CM, CM, CM, CM, CM, CM, BB, BA, BA, EX, EX, AL, AL,
    AL, BA, BA, BA, BA, BA, BA, BA, BA, BA, BA, BA, BA, BA, BA, BA, AL, AL, AL, AL, CM, CM, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, BA, BA, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, XX, XX, XX, XX, XX, XX, BB, BB, BB, BB, BB, BB, BB, BB, BB, BB, BB, BB, BB, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    AL, XX, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA,
    SA, SA, SA, XX, XX, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, SA, XX, XX, XX, XX,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, SA, SA, BA, BA, BA, SA, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_70: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, XX,
    XX, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, XX, CM,
    CM, XX, XX, CM, CM, CM, CM, AL, CM, AL, CM, CM, BA, BA, BA, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, XX, XX, CM, CM, CM, CM, CM, CM,
    CM, AL, BB, AL, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, AL, CM, CM, CM, CM, BB,
    AL, BA, BA, BA, BA, BB, AL, CM, XX, XX, XX, XX, XX, XX, XX, XX, AL, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, BA, BA, BA, AL, BB, BB,
    BB, BA, BA, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_71: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM,
    CM, CM, CM, CM, CM, CM, CM, XX, CM, CM, CM, CM, CM, CM, CM, CM, AL, BA, BA, BA, BA, BA, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, BB, EX, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    XX, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, XX,
    AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, XX,
    XX, XX, CM, XX, CM, CM, XX, CM, CM, CM, CM, CM, CM, CM, AL, CM, XX, XX, XX, XX, XX, XX, XX, XX,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, XX, AL,
    AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, XX, CM, CM, XX, CM, CM, CM, CM, CM,
    AL, XX, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, AL, AL, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, PO, PO, PO, PO, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, BA,
];

pub const UAX14_PROPERTIES_72: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_73: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, BA, BA, BA, BA, BA, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_76: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    OP, OP, OP, CL, CL, CL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CL, AL, AL, AL, OP, CL,
    OP, CL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, OP, CL, CL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
];

pub const UAX14_PROPERTIES_77: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX,
    GL, GL, GL, GL, GL, GL, GL, OP, CL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_81: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, OP, CL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_90: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, BA, BA,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, CM, CM, CM, CM, CM, BA, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    CM, CM, CM, CM, CM, CM, CM, BA, BA, BA, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, BA, AL, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, AL, AL, AL, AL, AL,
    AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_91: [u8; 1024] = [
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, BA, BA, AL, AL, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, XX, XX, XX, XX, CM, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, CM,
    CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, NS, NS, NS, NS, GL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    CM, CM, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_97: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_98: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
];

pub const UAX14_PROPERTIES_99: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_108: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    CJ, CJ, CJ, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, CJ, CJ, CJ, CJ,
    XX, XX, XX, XX, XX, XX, XX, XX, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_111: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, CM, CM, BA, CM, CM, CM, CM, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_116: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM,
    CM, CM, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, AL, AL, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, CM, CM, CM, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_117: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, XX, XX, AL, XX, XX, AL, AL, XX,
    XX, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, XX, AL, AL, AL,
    AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL,
    AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, XX, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, XX, AL, XX, XX, XX, AL, AL, AL, AL, AL, AL,
    AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU,
];

pub const UAX14_PROPERTIES_118: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL, AL, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, AL, AL, AL,
    AL, AL, AL, AL, AL, CM, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, AL, AL, BA,
    BA, BA, BA, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, CM, CM, CM, CM, CM,
    XX, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_120: [u8; 1024] = [
    CM, CM, CM, CM, CM, CM, CM, XX, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM, CM,
    CM, XX, XX, CM, CM, CM, CM, CM, CM, CM, XX, CM, CM, XX, CM, CM, CM, CM, CM, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, CM, CM, CM, CM, CM, CM, CM, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, AL, AL,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, CM, CM, CM, CM, NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, PR,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_122: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, CM, CM, CM, CM, CM, CM, CM, AL, XX, XX, XX, XX,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, OP, OP, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_123: [u8; 1024] = [
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, PO, AL, AL, AL, PO, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, XX, AL, XX, XX, AL,
    XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, XX, AL, XX, AL, XX, XX, XX, XX,
    XX, XX, AL, XX, XX, XX, XX, AL, XX, AL, XX, AL, XX, AL, AL, AL, XX, AL, AL, XX, AL, XX, XX, AL,
    XX, AL, XX, AL, XX, AL, XX, AL, XX, AL, AL, XX, AL, XX, XX, AL, AL, AL, AL, XX, AL, AL, AL, AL,
    AL, AL, AL, XX, AL, AL, AL, AL, XX, AL, AL, AL, AL, XX, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX,
    XX, AL, AL, AL, XX, AL, AL, AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_124: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, ID, ID, ID, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AL, AL, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AL, AL, AL, ID, ID, ID, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI,
    AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, AI, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, RI, RI, RI, RI, RI, RI, RI, RI, RI, RI, RI, RI, RI, RI, RI, RI, RI, RI,
    RI, RI, RI, RI, RI, RI, RI, RI, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, EB, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, ID, ID, ID, ID, ID, AL, ID, ID, ID,
    ID, ID, EB, EB, EB, ID, ID, EB, ID, ID, EB, EB, EB, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, EM, EM, EM, EM, EM,
];

pub const UAX14_PROPERTIES_125: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, EB, EB, ID, ID, EB, EB,
    EB, EB, EB, EB, EB, EB, EB, EB, EB, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB,
    EB, ID, ID, ID, EB, ID, ID, ID, ID, EB, EB, EB, ID, EB, EB, EB, ID, ID, ID, ID, ID, ID, ID, EB,
    ID, EB, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, ID, AL, ID, AL, ID, ID, ID,
    ID, ID, EB, ID, ID, ID, ID, AL, ID, AL, AL, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, EB, EB, ID, ID, ID, ID, EB, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, EB, ID, ID, ID, ID, EB, EB, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL,
    AL, AL, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, EB, EB, EB, ID, ID, ID, EB, EB, EB, EB, EB, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, QU, QU, QU, NS, NS, NS, AL, AL, AL, AL, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, EB, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, EB, EB, EB, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, EB, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, EB, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
];

pub const UAX14_PROPERTIES_126: [u8; 1024] = [
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, ID, ID, ID,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, ID, ID, ID, ID, ID,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, EB, ID, ID, EB, ID, ID, ID, ID, ID, ID, ID, ID, EB, EB, EB, EB, EB, EB, EB, EB,
    ID, ID, ID, ID, ID, ID, EB, ID, ID, ID, ID, ID, ID, ID, ID, ID, EB, EB, EB, EB, EB, EB, EB, EB,
    EB, EB, ID, ID, EB, EB, EB, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, EB, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, EB, EB, ID, EB, EB, ID, EB, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, EB, EB, EB, ID, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, EB, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, XX, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL,
    AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    NU, NU, NU, NU, NU, NU, NU, NU, NU, NU, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTIES_127: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, XX, XX,
];

pub const UAX14_PROPERTIES_ID: [u8; 1024] = [
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID,
];

pub const UAX14_PROPERTIES_SG: [u8; 1024] = [
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
    SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG, SG,
];

pub const UAX14_PROPERTIES_XX: [u8; 1024] = [
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX,
];

pub const UAX14_PROPERTY_TABLE: [&[u8; 1024]; 128] = [
    &UAX14_PROPERTIES_0,
    &UAX14_PROPERTIES_1,
    &UAX14_PROPERTIES_2,
    &UAX14_PROPERTIES_3,
    &UAX14_PROPERTIES_4,
    &UAX14_PROPERTIES_5,
    &UAX14_PROPERTIES_6,
    &UAX14_PROPERTIES_7,
    &UAX14_PROPERTIES_8,
    &UAX14_PROPERTIES_9,
    &UAX14_PROPERTIES_10,
    &UAX14_PROPERTIES_11,
    &UAX14_PROPERTIES_12,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_19,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_40,
    &UAX14_PROPERTIES_41,
    &UAX14_PROPERTIES_42,
    &UAX14_PROPERTIES_43,
    &UAX14_PROPERTIES_44,
    &UAX14_PROPERTIES_45,
    &UAX14_PROPERTIES_46,
    &UAX14_PROPERTIES_47,
    &UAX14_PROPERTIES_48,
    &UAX14_PROPERTIES_49,
    &UAX14_PROPERTIES_50,
    &UAX14_PROPERTIES_51,
    &UAX14_PROPERTIES_52,
    &UAX14_PROPERTIES_53,
    &UAX14_PROPERTIES_SG,
    &UAX14_PROPERTIES_SG,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_62,
    &UAX14_PROPERTIES_63,
    &UAX14_PROPERTIES_64,
    &UAX14_PROPERTIES_65,
    &UAX14_PROPERTIES_66,
    &UAX14_PROPERTIES_67,
    &UAX14_PROPERTIES_68,
    &UAX14_PROPERTIES_69,
    &UAX14_PROPERTIES_70,
    &UAX14_PROPERTIES_71,
    &UAX14_PROPERTIES_72,
    &UAX14_PROPERTIES_73,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_76,
    &UAX14_PROPERTIES_77,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_81,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_90,
    &UAX14_PROPERTIES_91,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_ID,
    &UAX14_PROPERTIES_97,
    &UAX14_PROPERTIES_98,
    &UAX14_PROPERTIES_99,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_108,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_111,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_116,
    &UAX14_PROPERTIES_117,
    &UAX14_PROPERTIES_118,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_120,
    &UAX14_PROPERTIES_XX,
    &UAX14_PROPERTIES_122,
    &UAX14_PROPERTIES_123,
    &UAX14_PROPERTIES_124,
    &UAX14_PROPERTIES_125,
    &UAX14_PROPERTIES_126,
    &UAX14_PROPERTIES_127,
];
pub const UAX14_RULE_TABLE: [i8; 2601] = [
    // AI
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // AL
    -128,          /* AI */
    -1,            /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // B2
    -128,          /* AI */
    -128,          /* AL */
    -1,            /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    B2_SP as i8,   /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // BA
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -128,          /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // BB
    -1,            /* AI */
    -1,            /* AL */
    -1,            /* B2 */
    -1,            /* BA */
    -1,            /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -1,            /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -1,            /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -1,            /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -1,            /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -1,            /* RI */
    -1,            /* SA */
    -1,            /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -1,            /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -1,            /* B2_SP */
    -1,            /* CL_CP_SP */
    -1,            /* CM_CM */
    -1,            /* HL_HY */
    -1,            /* OP_SP */
    -1,            /* QU_SP */
    -1,            /* ZW_SP */
    -1,            /* ZWJ_ZWJ */
    // BK
    -128, /* AI */
    -128, /* AL */
    -128, /* B2 */
    -128, /* BA */
    -128, /* BB */
    -128, /* BK */
    -128, /* CB */
    -128, /* CJ */
    -128, /* CL */
    -128, /* CM */
    -128, /* CP */
    -128, /* CR */
    -128, /* EB */
    -128, /* EM */
    -128, /* EX */
    -128, /* GL */
    -128, /* H2 */
    -128, /* H3 */
    -128, /* HL */
    -128, /* HY */
    -128, /* ID */
    -128, /* IN */
    -128, /* IS */
    -128, /* JL */
    -128, /* JT */
    -128, /* JV */
    -128, /* LF */
    -128, /* NL */
    -128, /* NS */
    -128, /* NU */
    -128, /* OP */
    -128, /* PO */
    -128, /* PR */
    -128, /* QU */
    -128, /* RI */
    -128, /* SA */
    -128, /* SG */
    -128, /* SP */
    -128, /* SY */
    -128, /* WJ */
    -128, /* XX */
    -128, /* ZW */
    -128, /* ZWJ */
    -128, /* B2_SP */
    -128, /* CL_CP_SP */
    -128, /* CM_CM */
    -128, /* HL_HY */
    -128, /* OP_SP */
    -128, /* QU_SP */
    -128, /* ZW_SP */
    -128, /* ZWJ_ZWJ */
    // CB
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -128,          /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -128,          /* HY */
    -128,          /* ID */
    -128,          /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -128,          /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // CJ
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // CL
    -128,           /* AI */
    -128,           /* AL */
    -128,           /* B2 */
    -1,             /* BA */
    -128,           /* BB */
    -1,             /* BK */
    -128,           /* CB */
    -128,           /* CJ */
    -1,             /* CL */
    CM_CM as i8,    /* CM */
    -1,             /* CP */
    -1,             /* CR */
    -128,           /* EB */
    -128,           /* EM */
    -1,             /* EX */
    -1,             /* GL */
    -128,           /* H2 */
    -128,           /* H3 */
    -128,           /* HL */
    -1,             /* HY */
    -128,           /* ID */
    -1,             /* IN */
    -1,             /* IS */
    -128,           /* JL */
    -128,           /* JT */
    -128,           /* JV */
    -1,             /* LF */
    -1,             /* NL */
    -1,             /* NS */
    -128,           /* NU */
    -128,           /* OP */
    -1,             /* PO */
    -1,             /* PR */
    -1,             /* QU */
    -128,           /* RI */
    -128,           /* SA */
    -128,           /* SG */
    CL_CP_SP as i8, /* SP */
    -1,             /* SY */
    -1,             /* WJ */
    -128,           /* XX */
    -1,             /* ZW */
    ZWJ_ZWJ as i8,  /* ZWJ */
    -128,           /* B2_SP */
    -128,           /* CL_CP_SP */
    -128,           /* CM_CM */
    -128,           /* HL_HY */
    -128,           /* OP_SP */
    -128,           /* QU_SP */
    -128,           /* ZW_SP */
    -128,           /* ZWJ_ZWJ */
    // CM
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // CP
    -128,          /* AI */
    -1,            /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -128,          /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // CR
    -128, /* AI */
    -128, /* AL */
    -128, /* B2 */
    -128, /* BA */
    -128, /* BB */
    -128, /* BK */
    -128, /* CB */
    -128, /* CJ */
    -128, /* CL */
    -128, /* CM */
    -128, /* CP */
    -128, /* CR */
    -128, /* EB */
    -128, /* EM */
    -128, /* EX */
    -128, /* GL */
    -128, /* H2 */
    -128, /* H3 */
    -128, /* HL */
    -128, /* HY */
    -128, /* ID */
    -128, /* IN */
    -128, /* IS */
    -128, /* JL */
    -128, /* JT */
    -128, /* JV */
    -1,   /* LF */
    -128, /* NL */
    -128, /* NS */
    -128, /* NU */
    -128, /* OP */
    -128, /* PO */
    -128, /* PR */
    -128, /* QU */
    -128, /* RI */
    -128, /* SA */
    -128, /* SG */
    -128, /* SP */
    -128, /* SY */
    -128, /* WJ */
    -128, /* XX */
    -128, /* ZW */
    -128, /* ZWJ */
    -128, /* B2_SP */
    -128, /* CL_CP_SP */
    -128, /* CM_CM */
    -128, /* HL_HY */
    -128, /* OP_SP */
    -128, /* QU_SP */
    -128, /* ZW_SP */
    -128, /* ZWJ_ZWJ */
    // EB
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // EM
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // EX
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // GL
    -1,            /* AI */
    -1,            /* AL */
    -1,            /* B2 */
    -1,            /* BA */
    -1,            /* BB */
    -1,            /* BK */
    -1,            /* CB */
    -1,            /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -1,            /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -1,            /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -1,            /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -1,            /* RI */
    -1,            /* SA */
    -1,            /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -1,            /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -1,            /* B2_SP */
    -1,            /* CL_CP_SP */
    -1,            /* CM_CM */
    -1,            /* HL_HY */
    -1,            /* OP_SP */
    -1,            /* QU_SP */
    -1,            /* ZW_SP */
    -1,            /* ZWJ_ZWJ */
    // H2
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -128,          /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -1,            /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // H3
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -1,            /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -1,            /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // HL
    -128,          /* AI */
    -1,            /* AL */
    -128,          /* B2 */
    HL_HY as i8,   /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -1,            /* HL */
    HL_HY as i8,   /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // HY
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -128,          /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // ID
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // IN
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // IS
    -128,          /* AI */
    -1,            /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // JL
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -128,          /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -1,            /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // JT
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -1,            /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -1,            /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // JV
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -128,          /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -1,            /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // LF
    -128, /* AI */
    -128, /* AL */
    -128, /* B2 */
    -128, /* BA */
    -128, /* BB */
    -128, /* BK */
    -128, /* CB */
    -128, /* CJ */
    -128, /* CL */
    -128, /* CM */
    -128, /* CP */
    -128, /* CR */
    -128, /* EB */
    -128, /* EM */
    -128, /* EX */
    -128, /* GL */
    -128, /* H2 */
    -128, /* H3 */
    -128, /* HL */
    -128, /* HY */
    -128, /* ID */
    -128, /* IN */
    -128, /* IS */
    -128, /* JL */
    -128, /* JT */
    -128, /* JV */
    -128, /* LF */
    -128, /* NL */
    -128, /* NS */
    -128, /* NU */
    -128, /* OP */
    -128, /* PO */
    -128, /* PR */
    -128, /* QU */
    -128, /* RI */
    -128, /* SA */
    -128, /* SG */
    -128, /* SP */
    -128, /* SY */
    -128, /* WJ */
    -128, /* XX */
    -128, /* ZW */
    -128, /* ZWJ */
    -128, /* B2_SP */
    -128, /* CL_CP_SP */
    -128, /* CM_CM */
    -128, /* HL_HY */
    -128, /* OP_SP */
    -128, /* QU_SP */
    -128, /* ZW_SP */
    -128, /* ZWJ_ZWJ */
    // NL
    -128, /* AI */
    -128, /* AL */
    -128, /* B2 */
    -128, /* BA */
    -128, /* BB */
    -128, /* BK */
    -128, /* CB */
    -128, /* CJ */
    -128, /* CL */
    -128, /* CM */
    -128, /* CP */
    -128, /* CR */
    -128, /* EB */
    -128, /* EM */
    -128, /* EX */
    -128, /* GL */
    -128, /* H2 */
    -128, /* H3 */
    -128, /* HL */
    -128, /* HY */
    -128, /* ID */
    -128, /* IN */
    -128, /* IS */
    -128, /* JL */
    -128, /* JT */
    -128, /* JV */
    -128, /* LF */
    -128, /* NL */
    -128, /* NS */
    -128, /* NU */
    -128, /* OP */
    -128, /* PO */
    -128, /* PR */
    -128, /* QU */
    -128, /* RI */
    -128, /* SA */
    -128, /* SG */
    -128, /* SP */
    -128, /* SY */
    -128, /* WJ */
    -128, /* XX */
    -128, /* ZW */
    -128, /* ZWJ */
    -128, /* B2_SP */
    -128, /* CL_CP_SP */
    -128, /* CM_CM */
    -128, /* HL_HY */
    -128, /* OP_SP */
    -128, /* QU_SP */
    -128, /* ZW_SP */
    -128, /* ZWJ_ZWJ */
    // NS
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // NU
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // OP
    -1,            /* AI */
    -1,            /* AL */
    -1,            /* B2 */
    -1,            /* BA */
    -1,            /* BB */
    -1,            /* BK */
    -1,            /* CB */
    -1,            /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -1,            /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -1,            /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -1,            /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -1,            /* RI */
    -1,            /* SA */
    -1,            /* SG */
    OP_SP as i8,   /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -1,            /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -1,            /* B2_SP */
    -1,            /* CL_CP_SP */
    -1,            /* CM_CM */
    -1,            /* HL_HY */
    -1,            /* OP_SP */
    -1,            /* QU_SP */
    -1,            /* ZW_SP */
    -1,            /* ZWJ_ZWJ */
    // PO
    -128,          /* AI */
    -1,            /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // PR
    -128,          /* AI */
    -1,            /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -1,            /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -1,            /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -1,            /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // QU
    -1,            /* AI */
    -1,            /* AL */
    -1,            /* B2 */
    -1,            /* BA */
    -1,            /* BB */
    -1,            /* BK */
    -1,            /* CB */
    -1,            /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -1,            /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -1,            /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -1,            /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -1,            /* RI */
    -1,            /* SA */
    -1,            /* SG */
    QU_SP as i8,   /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -1,            /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -1,            /* B2_SP */
    -1,            /* CL_CP_SP */
    -1,            /* CM_CM */
    -1,            /* HL_HY */
    -1,            /* OP_SP */
    -1,            /* QU_SP */
    -1,            /* ZW_SP */
    -1,            /* ZWJ_ZWJ */
    // RI
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // SA
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // SG
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // SP
    -128,           /* AI */
    -128,           /* AL */
    -128,           /* B2 */
    -128,           /* BA */
    -128,           /* BB */
    -1,             /* BK */
    -128,           /* CB */
    -128,           /* CJ */
    -1,             /* CL */
    -128,           /* CM */
    -1,             /* CP */
    -1,             /* CR */
    -128,           /* EB */
    -128,           /* EM */
    -1,             /* EX */
    -128,           /* GL */
    -128,           /* H2 */
    -128,           /* H3 */
    -128,           /* HL */
    -128,           /* HY */
    -128,           /* ID */
    -128,           /* IN */
    -1,             /* IS */
    -128,           /* JL */
    -128,           /* JT */
    -128,           /* JV */
    -1,             /* LF */
    -1,             /* NL */
    -128,           /* NS */
    -128,           /* NU */
    -128,           /* OP */
    -128,           /* PO */
    -128,           /* PR */
    -128,           /* QU */
    -128,           /* RI */
    -128,           /* SA */
    -128,           /* SG */
    CL_CP_SP as i8, /* SP */
    -1,             /* SY */
    -1,             /* WJ */
    -128,           /* XX */
    -1,             /* ZW */
    -128,           /* ZWJ */
    -128,           /* B2_SP */
    -128,           /* CL_CP_SP */
    -128,           /* CM_CM */
    -128,           /* HL_HY */
    -128,           /* OP_SP */
    -128,           /* QU_SP */
    -128,           /* ZW_SP */
    -128,           /* ZWJ_ZWJ */
    // SY
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // WJ
    -1,            /* AI */
    -1,            /* AL */
    -1,            /* B2 */
    -1,            /* BA */
    -1,            /* BB */
    -1,            /* BK */
    -1,            /* CB */
    -1,            /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -1,            /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -1,            /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -1,            /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -1,            /* RI */
    -1,            /* SA */
    -1,            /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -1,            /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -1,            /* B2_SP */
    -1,            /* CL_CP_SP */
    -1,            /* CM_CM */
    -1,            /* HL_HY */
    -1,            /* OP_SP */
    -1,            /* QU_SP */
    -1,            /* ZW_SP */
    -1,            /* ZWJ_ZWJ */
    // XX
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -1,            /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -1,            /* HY */
    -128,          /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -1,            /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // ZW
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -128,          /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -128,          /* CL */
    CM_CM as i8,   /* CM */
    -128,          /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -128,          /* EX */
    -128,          /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -128,          /* HY */
    -128,          /* ID */
    -128,          /* IN */
    -128,          /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -128,          /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -128,          /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    ZW_SP as i8,   /* SP */
    -128,          /* SY */
    -128,          /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // ZWJ
    -1,            /* AI */
    -1,            /* AL */
    -1,            /* B2 */
    -1,            /* BA */
    -1,            /* BB */
    -1,            /* BK */
    -1,            /* CB */
    -1,            /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -1,            /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -1,            /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -1,            /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -1,            /* RI */
    -1,            /* SA */
    -1,            /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -1,            /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -1,            /* B2_SP */
    -1,            /* CL_CP_SP */
    -1,            /* CM_CM */
    -1,            /* HL_HY */
    -1,            /* OP_SP */
    -1,            /* QU_SP */
    -1,            /* ZW_SP */
    -1,            /* ZWJ_ZWJ */
    // B2_SP
    -128,          /* AI */
    -128,          /* AL */
    -1,            /* B2 */
    -128,          /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -128,          /* HY */
    -128,          /* ID */
    -128,          /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -128,          /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -128,          /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    B2_SP as i8,   /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // CL_CP_SP
    -128,           /* AI */
    -128,           /* AL */
    -128,           /* B2 */
    -128,           /* BA */
    -128,           /* BB */
    -1,             /* BK */
    -128,           /* CB */
    -128,           /* CJ */
    -1,             /* CL */
    CM_CM as i8,    /* CM */
    -1,             /* CP */
    -1,             /* CR */
    -128,           /* EB */
    -128,           /* EM */
    -1,             /* EX */
    -1,             /* GL */
    -128,           /* H2 */
    -128,           /* H3 */
    -128,           /* HL */
    -128,           /* HY */
    -128,           /* ID */
    -128,           /* IN */
    -1,             /* IS */
    -128,           /* JL */
    -128,           /* JT */
    -128,           /* JV */
    -1,             /* LF */
    -1,             /* NL */
    -1,             /* NS */
    -128,           /* NU */
    -128,           /* OP */
    -128,           /* PO */
    -128,           /* PR */
    -128,           /* QU */
    -128,           /* RI */
    -128,           /* SA */
    -128,           /* SG */
    CL_CP_SP as i8, /* SP */
    -1,             /* SY */
    -1,             /* WJ */
    -128,           /* XX */
    -1,             /* ZW */
    ZWJ_ZWJ as i8,  /* ZWJ */
    -128,           /* B2_SP */
    -128,           /* CL_CP_SP */
    -128,           /* CM_CM */
    -128,           /* HL_HY */
    -128,           /* OP_SP */
    -128,           /* QU_SP */
    -128,           /* ZW_SP */
    -128,           /* ZWJ_ZWJ */
    // CM_CM
    CM as i8,      /* AI */
    CM as i8,      /* AL */
    CM as i8,      /* B2 */
    CM as i8,      /* BA */
    CM as i8,      /* BB */
    -1,            /* BK */
    CM as i8,      /* CB */
    CM as i8,      /* CJ */
    CM as i8,      /* CL */
    CM_CM as i8,   /* CM */
    CM as i8,      /* CP */
    -1,            /* CR */
    CM as i8,      /* EB */
    CM as i8,      /* EM */
    CM as i8,      /* EX */
    CM as i8,      /* GL */
    CM as i8,      /* H2 */
    CM as i8,      /* H3 */
    CM as i8,      /* HL */
    CM as i8,      /* HY */
    CM as i8,      /* ID */
    CM as i8,      /* IN */
    CM as i8,      /* IS */
    CM as i8,      /* JL */
    CM as i8,      /* JT */
    CM as i8,      /* JV */
    -1,            /* LF */
    -1,            /* NL */
    CM as i8,      /* NS */
    CM as i8,      /* NU */
    CM as i8,      /* OP */
    CM as i8,      /* PO */
    CM as i8,      /* PR */
    CM as i8,      /* QU */
    CM as i8,      /* RI */
    CM as i8,      /* SA */
    CM as i8,      /* SG */
    -1,            /* SP */
    CM as i8,      /* SY */
    CM as i8,      /* WJ */
    CM as i8,      /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    CM as i8,      /* B2_SP */
    CM as i8,      /* CL_CP_SP */
    CM as i8,      /* CM_CM */
    CM as i8,      /* HL_HY */
    CM as i8,      /* OP_SP */
    CM as i8,      /* QU_SP */
    CM as i8,      /* ZW_SP */
    CM as i8,      /* ZWJ_ZWJ */
    // HL_HY
    -1,            /* AI */
    -1,            /* AL */
    -1,            /* B2 */
    -1,            /* BA */
    -1,            /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -1,            /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -1,            /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -1,            /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -1,            /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -1,            /* RI */
    -1,            /* SA */
    -1,            /* SG */
    -1,            /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -1,            /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -1,            /* B2_SP */
    -1,            /* CL_CP_SP */
    -1,            /* CM_CM */
    -1,            /* HL_HY */
    -1,            /* OP_SP */
    -1,            /* QU_SP */
    -1,            /* ZW_SP */
    -1,            /* ZWJ_ZWJ */
    // OP_SP
    -1,            /* AI */
    -1,            /* AL */
    -1,            /* B2 */
    -1,            /* BA */
    -1,            /* BB */
    -1,            /* BK */
    -1,            /* CB */
    -1,            /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -1,            /* EB */
    -1,            /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -1,            /* H2 */
    -1,            /* H3 */
    -1,            /* HL */
    -1,            /* HY */
    -1,            /* ID */
    -1,            /* IN */
    -1,            /* IS */
    -1,            /* JL */
    -1,            /* JT */
    -1,            /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -1,            /* NS */
    -1,            /* NU */
    -1,            /* OP */
    -1,            /* PO */
    -1,            /* PR */
    -1,            /* QU */
    -1,            /* RI */
    -1,            /* SA */
    -1,            /* SG */
    OP_SP as i8,   /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -1,            /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -1,            /* B2_SP */
    -1,            /* CL_CP_SP */
    -1,            /* CM_CM */
    -1,            /* HL_HY */
    -1,            /* OP_SP */
    -1,            /* QU_SP */
    -1,            /* ZW_SP */
    -1,            /* ZWJ_ZWJ */
    // QU_SP
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -128,          /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -1,            /* CL */
    CM_CM as i8,   /* CM */
    -1,            /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -1,            /* EX */
    -1,            /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -128,          /* HY */
    -128,          /* ID */
    -128,          /* IN */
    -1,            /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -128,          /* NS */
    -128,          /* NU */
    -1,            /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -128,          /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    QU_SP as i8,   /* SP */
    -1,            /* SY */
    -1,            /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // ZW_SP
    -128,          /* AI */
    -128,          /* AL */
    -128,          /* B2 */
    -128,          /* BA */
    -128,          /* BB */
    -1,            /* BK */
    -128,          /* CB */
    -128,          /* CJ */
    -128,          /* CL */
    CM_CM as i8,   /* CM */
    -128,          /* CP */
    -1,            /* CR */
    -128,          /* EB */
    -128,          /* EM */
    -128,          /* EX */
    -128,          /* GL */
    -128,          /* H2 */
    -128,          /* H3 */
    -128,          /* HL */
    -128,          /* HY */
    -128,          /* ID */
    -128,          /* IN */
    -128,          /* IS */
    -128,          /* JL */
    -128,          /* JT */
    -128,          /* JV */
    -1,            /* LF */
    -1,            /* NL */
    -128,          /* NS */
    -128,          /* NU */
    -128,          /* OP */
    -128,          /* PO */
    -128,          /* PR */
    -128,          /* QU */
    -128,          /* RI */
    -128,          /* SA */
    -128,          /* SG */
    ZW_SP as i8,   /* SP */
    -128,          /* SY */
    -128,          /* WJ */
    -128,          /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    -128,          /* B2_SP */
    -128,          /* CL_CP_SP */
    -128,          /* CM_CM */
    -128,          /* HL_HY */
    -128,          /* OP_SP */
    -128,          /* QU_SP */
    -128,          /* ZW_SP */
    -128,          /* ZWJ_ZWJ */
    // ZWJ_ZWJ
    ZWJ as i8,     /* AI */
    ZWJ as i8,     /* AL */
    ZWJ as i8,     /* B2 */
    ZWJ as i8,     /* BA */
    ZWJ as i8,     /* BB */
    -1,            /* BK */
    ZWJ as i8,     /* CB */
    ZWJ as i8,     /* CJ */
    ZWJ as i8,     /* CL */
    CM_CM as i8,   /* CM */
    ZWJ as i8,     /* CP */
    -1,            /* CR */
    ZWJ as i8,     /* EB */
    ZWJ as i8,     /* EM */
    ZWJ as i8,     /* EX */
    ZWJ as i8,     /* GL */
    ZWJ as i8,     /* H2 */
    ZWJ as i8,     /* H3 */
    ZWJ as i8,     /* HL */
    ZWJ as i8,     /* HY */
    ZWJ as i8,     /* ID */
    ZWJ as i8,     /* IN */
    ZWJ as i8,     /* IS */
    ZWJ as i8,     /* JL */
    ZWJ as i8,     /* JT */
    ZWJ as i8,     /* JV */
    -1,            /* LF */
    -1,            /* NL */
    ZWJ as i8,     /* NS */
    ZWJ as i8,     /* NU */
    ZWJ as i8,     /* OP */
    ZWJ as i8,     /* PO */
    ZWJ as i8,     /* PR */
    ZWJ as i8,     /* QU */
    ZWJ as i8,     /* RI */
    ZWJ as i8,     /* SA */
    ZWJ as i8,     /* SG */
    -1,            /* SP */
    ZWJ as i8,     /* SY */
    ZWJ as i8,     /* WJ */
    ZWJ as i8,     /* XX */
    -1,            /* ZW */
    ZWJ_ZWJ as i8, /* ZWJ */
    ZWJ as i8,     /* B2_SP */
    ZWJ as i8,     /* CL_CP_SP */
    ZWJ as i8,     /* CM_CM */
    ZWJ as i8,     /* HL_HY */
    ZWJ as i8,     /* OP_SP */
    ZWJ as i8,     /* QU_SP */
    ZWJ as i8,     /* ZW_SP */
    ZWJ as i8,     /* ZWJ_ZWJ */
];
