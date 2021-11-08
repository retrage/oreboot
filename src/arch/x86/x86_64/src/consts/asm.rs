pub const EFER: usize = 0xC0000080;
pub const LME: usize = 1 << 8;

pub const PSE: usize = 0x10;
pub const PGE: usize = 0x40;
pub const PAE: usize = 0x20;
pub const PGE_PAE: usize = 0x60;

pub const PE: usize = 1;
pub const MP: usize = 2;
pub const EM: usize = 4;
pub const TS: usize = 8;
pub const ET: usize = 0x10;
pub const NE: usize = 0x20;
pub const WP: usize = 0x10000;
pub const AM: usize = 0x40000;
pub const NW: usize = 0x20000000;
pub const CD: usize = 0x40000000;
pub const PG: usize = 0x80000000;

pub const CR0_PG: usize = 1 << 31;
