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

pub const CR0_CD: usize = 1 << 30;
pub const CR0_PG: usize = 1 << 31;

pub const PTE_P: usize = 0x0000000000000001;
pub const PTE_RW: usize = 0x0000000000000002;
pub const PTE_PS: usize = 0x0000000000000080;
pub const PTE2_MPAT: usize = 0x0000000000001000;

pub const NUM_FIXED_RANGES: usize = 88;
pub const RANGES_PER_FIXED_MTRR: usize = 8;
pub const MTRR_FIX_64K_00000: usize = 0x250;
pub const MTRR_FIX_16K_80000: usize = 0x258;
pub const MTRR_FIX_16K_A0000: usize = 0x259;
pub const MTRR_FIX_4K_C0000: usize = 0x268;
pub const MTRR_FIX_4K_C8000: usize = 0x269;
pub const MTRR_FIX_4K_D0000: usize = 0x26a;
pub const MTRR_FIX_4K_D8000: usize = 0x26b;
pub const MTRR_FIX_4K_E0000: usize = 0x26c;
pub const MTRR_FIX_4K_E8000: usize = 0x26d;
pub const MTRR_FIX_4K_F0000: usize = 0x26e;
pub const MTRR_FIX_4K_F8000: usize = 0x26f;
pub const MTRR_CAP_MSR: usize = 0x0fe;
//#define MTRR_PHYS_BASE(reg)       (0x200 + 2 * (reg))
//#define MTRR_PHYS_MASK(reg)       (MTRR_PHYS_BASE(reg) + 1)
pub const MTRR_DEF_TYPE_MSR: usize = 0x2ff;
pub const MTRR_DEF_TYPE_MASK: usize = 0xff;
pub const MTRR_DEF_TYPE_EN: usize = 1 << 11;
pub const MTRR_DEF_TYPE_FIX_EN: usize = 1 << 10;

pub const MTRR_PHYS_MASK_VALID: usize = 1 << 11;

pub const MTRR_TYPE_WRPROT: usize = 5;
pub const MTRR_TYPE_WRBACK: usize = 6;

#[macro_export]
macro_rules! const_asm {
    ($file : expr $(,)?) => {
        global_asm!(
            include_str!($file),
            pse = const asm::PSE,
            pge = const asm::PGE,
            pae = const asm::PAE,
            efer = const asm::EFER,
            lme = const asm::LME,
            cd = const asm::CD,
            nw = const asm::NW,
            ts = const asm::TS,
            mp = const asm::MP,
            pg = const asm::PG,
            wp = const asm::WP,
            pe = const asm::PE,
            options(att_syntax),
        );
    };
}
