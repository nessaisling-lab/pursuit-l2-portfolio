//! The four project themes, as data rather than as repeated CSS.
//!
//! Every colour here was sampled from that project's own screenshots in this repo by
//! region-mode pixel scan. The point of moving them into Rust is the bottom of this
//! file: `assert_readable!` runs WCAG contrast at compile time, so a theme whose text
//! cannot be read on its own surface is a build error rather than something a person
//! has to notice. One of these pairings really was 1.2:1 before it was caught by hand.

/// A colour, kept as three channels so contrast can be computed from it.
#[derive(Clone, Copy, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
    /// `rgba()` at the given alpha percentage, for the muted inks and hairline rules.
    pub fn alpha(self, pct: u8) -> String {
        format!("rgba({},{},{},.{:02})", self.0, self.1, self.2, pct)
    }
}

// ── compile-time contrast ────────────────────────────────────────────────────
//
// WCAG needs sRGB channels linearised by `((c+0.055)/1.055)^2.4`, and `powf` is not
// available in a const fn. So the curve is precomputed once as fixed-point integers
// (linear value x 1_000_000) and everything downstream is integer arithmetic, which
// const evaluation is perfectly happy with.

#[rustfmt::skip]
const SRGB_LINEAR: [u32; 256] = [
          0,     304,     607,     911,    1214,    1518,    1821,    2125,
       2428,    2732,    3035,    3347,    3677,    4025,    4391,    4777,
       5182,    5605,    6049,    6512,    6995,    7499,    8023,    8568,
       9134,    9721,   10330,   10960,   11612,   12286,   12983,   13702,
      14444,   15209,   15996,   16807,   17642,   18500,   19382,   20289,
      21219,   22174,   23153,   24158,   25187,   26241,   27321,   28426,
      29557,   30713,   31896,   33105,   34340,   35601,   36889,   38204,
      39546,   40915,   42311,   43735,   45186,   46665,   48172,   49707,
      51269,   52861,   54480,   56128,   57805,   59511,   61246,   63010,
      64803,   66626,   68478,   70360,   72272,   74214,   76185,   78187,
      80220,   82283,   84376,   86500,   88656,   90842,   93059,   95307,
      97587,   99899,  102242,  104616,  107023,  109462,  111932,  114435,
     116971,  119538,  122139,  124772,  127438,  130136,  132868,  135633,
     138432,  141263,  144128,  147027,  149960,  152926,  155926,  158961,
     162029,  165132,  168269,  171441,  174647,  177888,  181164,  184475,
     187821,  191202,  194618,  198069,  201556,  205079,  208637,  212231,
     215861,  219526,  223228,  226966,  230740,  234551,  238398,  242281,
     246201,  250158,  254152,  258183,  262251,  266356,  270498,  274677,
     278894,  283149,  287441,  291771,  296138,  300544,  304987,  309469,
     313989,  318547,  323143,  327778,  332452,  337164,  341914,  346704,
     351533,  356400,  361307,  366253,  371238,  376262,  381326,  386429,
     391572,  396755,  401978,  407240,  412543,  417885,  423268,  428690,
     434154,  439657,  445201,  450786,  456411,  462077,  467784,  473531,
     479320,  485150,  491021,  496933,  502886,  508881,  514918,  520996,
     527115,  533276,  539479,  545724,  552011,  558340,  564712,  571125,
     577580,  584078,  590619,  597202,  603827,  610496,  617207,  623960,
     630757,  637597,  644480,  651406,  658375,  665387,  672443,  679542,
     686685,  693872,  701102,  708376,  715694,  723055,  730461,  737910,
     745404,  752942,  760525,  768151,  775822,  783538,  791298,  799103,
     806952,  814847,  822786,  830770,  838799,  846873,  854993,  863157,
     871367,  879622,  887923,  896269,  904661,  913099,  921582,  930111,
     938686,  947307,  955973,  964686,  973445,  982251,  991102, 1000000,
];

/// Relative luminance, fixed point (x 1_000_000).
const fn luminance(c: Rgb) -> u64 {
    let r = SRGB_LINEAR[c.0 as usize] as u64;
    let g = SRGB_LINEAR[c.1 as usize] as u64;
    let b = SRGB_LINEAR[c.2 as usize] as u64;
    (2126 * r + 7152 * g + 722 * b) / 10_000
}

/// WCAG contrast ratio x 100, so 4.5:1 reads as 450. Integer only, so it const-evaluates.
pub const fn contrast_x100(a: Rgb, b: Rgb) -> u64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    ((hi + 50_000) * 100) / (lo + 50_000)
}

/// Fails the build if `$fg` on `$bg` drops below `$min` x100. The message names the
/// pairing, because a bare "assertion failed" would send you hunting through four themes.
macro_rules! assert_readable {
    ($fg:expr, $bg:expr, $min:expr, $what:literal) => {
        const _: () = assert!(
            contrast_x100($fg, $bg) >= $min,
            concat!("theme contrast below WCAG minimum: ", $what),
        );
    };
}

// ── the themes ───────────────────────────────────────────────────────────────

#[derive(PartialEq)]
pub struct Theme {
    pub slug: &'static str,
    pub surface: Rgb,
    pub ink: Rgb,
    pub ink_alpha: u8,
    pub accent: Rgb,
    /// Chip background. HouseCheck's is opaque because the real app puts its mint
    /// inside a dark pill rather than on the light ground -- copying the component
    /// is what makes an otherwise unreadable brand colour legible.
    pub chip: &'static str,
    pub chip_ink: Rgb,
    /// The `Live` tag only. HouseCheck is the one build with a deployed URL, and its app
    /// marks that state in mint -- but mint is unreadable on the card's light ground, so
    /// it is legible only because the chip behind it is dark. Kept separate from
    /// `chip_ink` because collapsing the two turns every tag mint, which is not the
    /// design and did happen once.
    pub live_ink: Option<Rgb>,
    pub edge_alpha: u8,
    pub face: &'static str,
    pub face_size: &'static str,
    pub face_track: &'static str,
    /// Window/browser chrome height of that project's workflow GIF, as a percentage of
    /// the GIF's *width*. Percentage margins resolve against width, so this crops the
    /// same source pixels at every viewport.
    pub chrome_pct: &'static str,
    /// That app's own page background, sampled below the chrome, so a card whose text
    /// column runs taller than its screenshot has no visible seam.
    pub shot_bg: Rgb,
    /// Width divided by height of that project's recording, measured from the asset.
    /// Used to widen the bezel so every portrait video lands at the same height.
    pub video_aspect: f64,
    pub hover: Option<Rgb>,
}

pub const HOUSECHECK: Theme = Theme {
    slug: "housecheck",
    surface: Rgb(0xD6, 0xCD, 0xCD),
    ink: Rgb(0x27, 0x26, 0x28),
    ink_alpha: 72,
    accent: Rgb(0x17, 0x54, 0x43),
    chip: "#3B3B3D",
    chip_ink: Rgb(0xD6, 0xCD, 0xCD),
    live_ink: Some(Rgb(0x4B, 0xCD, 0xA7)),
    edge_alpha: 16,
    face: "Archivo,Inter,sans-serif",
    face_size: ".94",
    face_track: "-.025em",
    chrome_pct: "8.026%",
    shot_bg: Rgb(0xD9, 0xD8, 0xDA),
    video_aspect: 440.0 / 714.0,
    hover: None,
};

pub const ZIQPU: Theme = Theme {
    slug: "ziqpu",
    surface: Rgb(0xEE, 0xEA, 0xDD),
    ink: Rgb(0x2E, 0x25, 0x19),
    ink_alpha: 74,
    accent: Rgb(0x8A, 0x61, 0x09),
    chip: "rgba(138,97,9,.13)",
    chip_ink: Rgb(0x6E, 0x4E, 0x07),
    live_ink: None,
    edge_alpha: 16,
    face: "Fraunces,Georgia,serif",
    face_size: "1",
    face_track: "-.012em",
    chrome_pct: "4.605%",
    shot_bg: Rgb(0xEB, 0xE0, 0xCA),
    video_aspect: 1100.0 / 798.0,
    hover: Some(Rgb(0xB1, 0x45, 0x2C)),
};

pub const SITEASSURE: Theme = Theme {
    slug: "siteassure",
    surface: Rgb(0x15, 0x19, 0x1D),
    ink: Rgb(0xF2, 0xF5, 0xF7),
    ink_alpha: 68,
    accent: Rgb(0xF4, 0xA4, 0x1E),
    chip: "rgba(244,164,30,.13)",
    chip_ink: Rgb(0xF4, 0xA4, 0x1E),
    live_ink: None,
    edge_alpha: 14,
    face: "\"Source Serif 4\",Georgia,serif",
    face_size: "1",
    face_track: "-.008em",
    chrome_pct: "7.143%",
    shot_bg: Rgb(0x02, 0x04, 0x07),
    video_aspect: 436.0 / 918.0,
    hover: None,
};

pub const RESONA: Theme = Theme {
    slug: "resona",
    surface: Rgb(0x07, 0x1A, 0x18),
    ink: Rgb(0xEA, 0xF6, 0xF1),
    ink_alpha: 66,
    accent: Rgb(0x4F, 0xC7, 0x9A),
    chip: "rgba(45,106,79,.38)",
    chip_ink: Rgb(0x9F, 0xE8, 0xCB),
    live_ink: None,
    edge_alpha: 13,
    face: "Manrope,Inter,sans-serif",
    face_size: ".94",
    face_track: "-.03em",
    chrome_pct: "3.088%",
    shot_bg: Rgb(0x04, 0x1E, 0x21),
    video_aspect: 500.0 / 940.0,
    hover: None,
};

pub const THEMES: [&Theme; 4] = [&HOUSECHECK, &ZIQPU, &SITEASSURE, &RESONA];

// ── the invariants ───────────────────────────────────────────────────────────
//
// Body ink must clear 4.5:1 on its own surface. The accent carries small bold mono
// (the cycle number), which WCAG would let through at 3:1 as large text -- it is not
// large, so it is held to 4.5:1 as well. Chip ink is checked against the chip it
// actually sits in, not against the card, which is the mistake that hid HouseCheck's
// 1.2:1 mint: measured against the wrong background it looked fine.

assert_readable!(HOUSECHECK.ink, HOUSECHECK.surface, 450, "HouseCheck ink on surface");
assert_readable!(HOUSECHECK.accent, HOUSECHECK.surface, 450, "HouseCheck accent on surface");
assert_readable!(HOUSECHECK.chip_ink, Rgb(0x3B, 0x3B, 0x3D), 450, "HouseCheck chip ink on chip");
const HC_CHIP: Rgb = Rgb(0x3B, 0x3B, 0x3D);
const _: () = assert!(
    match HOUSECHECK.live_ink {
        Some(c) => contrast_x100(c, HC_CHIP) >= 450,
        None => false,
    },
    "HouseCheck Live tag must be readable inside its dark chip",
);

assert_readable!(ZIQPU.ink, ZIQPU.surface, 450, "Ziqpu ink on surface");
assert_readable!(ZIQPU.accent, ZIQPU.surface, 450, "Ziqpu accent on surface");
assert_readable!(ZIQPU.chip_ink, ZIQPU.surface, 450, "Ziqpu chip ink on surface");

assert_readable!(SITEASSURE.ink, SITEASSURE.surface, 450, "SiteAssure ink on surface");
assert_readable!(SITEASSURE.accent, SITEASSURE.surface, 450, "SiteAssure accent on surface");
assert_readable!(SITEASSURE.chip_ink, SITEASSURE.surface, 450, "SiteAssure chip ink on surface");

assert_readable!(RESONA.ink, RESONA.surface, 450, "Resona ink on surface");
assert_readable!(RESONA.accent, RESONA.surface, 450, "Resona accent on surface");
assert_readable!(RESONA.chip_ink, RESONA.surface, 450, "Resona chip ink on surface");

#[cfg(test)]
mod tests {
    use super::*;

    /// Anchors the fixed-point maths against ratios anyone can check by hand: black on
    /// white is exactly 21:1, and any colour against itself is exactly 1:1.
    #[test]
    fn known_ratios() {
        let black = Rgb(0, 0, 0);
        let white = Rgb(255, 255, 255);
        assert_eq!(contrast_x100(black, white), 2100);
        assert_eq!(contrast_x100(white, white), 100);
        assert_eq!(contrast_x100(black, black), 100);
    }

    /// The bug this whole module exists to prevent: HouseCheck's real app mint on
    /// HouseCheck's real app ground. If this ever climbs above 4.5:1 the sampled
    /// values have drifted and the const assertions are guarding nothing.
    #[test]
    fn the_pairing_that_was_wrong() {
        let mint = Rgb(0x4B, 0xCD, 0xA7);
        assert!(
            contrast_x100(mint, HOUSECHECK.surface) < 200,
            "mint on the light ground should still be unreadable -- that is why it lives in a chip",
        );
        assert!(contrast_x100(mint, Rgb(0x3B, 0x3B, 0x3D)) >= 450);
    }

    #[test]
    fn every_theme_is_distinct() {
        for (i, a) in THEMES.iter().enumerate() {
            for b in THEMES.iter().skip(i + 1) {
                assert_ne!(a.surface.hex(), b.surface.hex(), "two themes share a surface");
                assert_ne!(a.face, b.face, "two themes share a display face");
            }
        }
    }
}
