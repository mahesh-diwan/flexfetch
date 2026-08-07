//! Shared ANSI color tables: SGR color code → RGB. Single source of truth for
//! the palette used by ANSI parsing (export) and the colors module collector.

pub const ANSI_COLORS: &[[u8; 3]] = &[
    [0, 0, 0],       // 30 black
    [170, 0, 0],     // 31 red
    [0, 170, 0],     // 32 green
    [170, 85, 0],    // 33 yellow
    [0, 0, 170],     // 34 blue
    [170, 0, 170],   // 35 magenta
    [0, 170, 170],   // 36 cyan
    [170, 170, 170], // 37 white
];

pub const ANSI_BRIGHT_COLORS: &[[u8; 3]] = &[
    [85, 85, 85],    // 90 bright black
    [255, 85, 85],   // 91 bright red
    [85, 255, 85],   // 92 bright green
    [255, 255, 85],  // 93 bright yellow
    [85, 85, 255],   // 94 bright blue
    [255, 85, 255],  // 95 bright magenta
    [85, 255, 255],  // 96 bright cyan
    [255, 255, 255], // 97 bright white
];
