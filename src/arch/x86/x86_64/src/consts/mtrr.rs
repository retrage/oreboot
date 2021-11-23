/* SPDX-License-Identifier: GPL-2.0-only */

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
