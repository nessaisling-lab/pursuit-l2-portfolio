//! The page's content, as data. Kept apart from layout so that adding a fifth cycle
//! is one entry in `BUILDS` rather than a hand-edit of four parallel places.

use crate::theme::{self, Theme};

#[derive(PartialEq)]
pub struct Tag {
    pub label: &'static str,
    pub live: bool,
}

#[derive(PartialEq)]
pub struct Build {
    pub name: &'static str,
    pub cycle: &'static str,
    pub theme: &'static Theme,
    /// Path under this repo's tree, joined to `REPO` at render time.
    pub path: &'static str,
    pub blurb: &'static str,
    pub tags: &'static [Tag],
    pub shot: &'static str,
    pub alt: &'static str,
    /// Only the lead card carries one. Fifteen seconds is the budget a portfolio gets,
    /// so exactly one card says where to start.
    pub flag: Option<&'static str>,
    pub lead: bool,
}

const REPO: &str = "https://github.com/nessaisling-lab/pursuit-l2-portfolio/tree/main";

pub const BUILDS: &[Build] = &[
    Build {
        name: "HouseCheck",
        cycle: "CYCLE 04",
        theme: &theme::HOUSECHECK,
        path: "/cycle-4-housecheck",
        blurb: "Carfax for NYC apartments. Type an address, get a Building Health Card scored from \
                public city data — and export a record a stranger can cryptographically verify.",
        tags: &[
            Tag { label: "Live", live: true },
            Tag { label: "Rust · axum", live: false },
            Tag { label: "Ed25519", live: false },
            Tag { label: "149 tests", live: false },
        ],
        shot: "./cycle-4-housecheck/screenshots/00-workflow.gif",
        alt: "HouseCheck: searching an address through to the building record",
        flag: Some("Start here"),
        lead: true,
    },
    Build {
        name: "Ziqpu",
        cycle: "CYCLE 03",
        theme: &theme::ZIQPU,
        path: "/cycle-3-ziqpu",
        blurb: "A consumer astrology decision tool with two visible agents — one measures, one \
                interprets. The separation is the integrity guarantee, enforced by architecture \
                rather than disclaimer.",
        tags: &[
            Tag { label: "Rust · Dioxus", live: false },
            Tag { label: "Tool-calling", live: false },
            Tag { label: "Offline ephemeris", live: false },
        ],
        shot: "./cycle-3-ziqpu/screenshots/00-workflow.gif",
        alt: "Ziqpu: welcome through to a resolved chart",
        flag: None,
        lead: false,
    },
    Build {
        name: "SiteAssure",
        cycle: "CYCLE 02",
        theme: &theme::SITEASSURE,
        path: "/cycle-2-siteassure",
        blurb: "A tamper-evident, offline-first OSHA compliance logger driven by voice. Every \
                create, amend and delete is chained and hash-verified — nothing is silently \
                altered or removed.",
        tags: &[
            Tag { label: "Rust", live: false },
            Tag { label: "Hash chain", live: false },
            Tag { label: "3-OS CI", live: false },
        ],
        shot: "./cycle-2-siteassure/screenshots/00-workflow.gif",
        alt: "SiteAssure: speak it, record it, prove it",
        flag: None,
        lead: false,
    },
    Build {
        name: "Resona",
        cycle: "CYCLE 01",
        theme: &theme::RESONA,
        path: "/cycle-1-resona",
        blurb: "Privacy-first voice-to-text. 100% on-device via whisper.cpp — a property you can \
                verify by unplugging the network and watching it keep working.",
        tags: &[
            Tag { label: "Rust · Tauri 2", live: false },
            Tag { label: "whisper.cpp", live: false },
            Tag { label: "On-device", live: false },
        ],
        shot: "./cycle-1-resona/screenshots/00-workflow.gif",
        alt: "Resona: launch through to model selection",
        flag: None,
        lead: false,
    },
];

pub fn href(b: &Build) -> String {
    format!("{REPO}{}", b.path)
}

#[derive(PartialEq)]
pub struct Rule {
    pub head: &'static str,
    pub body: &'static str,
}

pub const RULES: &[Rule] = &[
    Rule {
        head: "Measure, don't estimate.",
        body: "Figures in these repos are measured and say so. Anything derived is labelled \
               derived; anything unverified says unverified. HouseCheck's backlog carries the rule \
               in writing: an item with no reason is a wish, not a task.",
    },
    Rule {
        head: "Verify against production, not the repo.",
        body: "Nearly every real defect I found in the final week was invisible from inside the \
               editor — a search box that answered a Manhattan address with a Brooklyn building, a \
               failed lookup that silently kept the previous result on screen, an assistant that \
               answered a different question than the one asked.",
    },
    Rule {
        head: "An absent answer is still an answer.",
        body: "Repeatedly the honest design has three states rather than two: signed / \
               unsigned-but-intact / tampered. A repair-time median / “nothing has been closed” / \
               genuinely no data. Collapsing the middle state is how software ends up stating \
               something reassuring it has not earned.",
    },
];

#[derive(PartialEq)]
pub struct Row {
    pub dt: &'static str,
    pub dd: &'static str,
}

pub const RUST_ROWS: &[Row] = &[
    Row { dt: "Backends", dd: "axum · rusqlite / SQLx · Tauri 2 · whisper-rs · cpal" },
    Row { dt: "Crypto", dd: "ed25519-dalek · sha2 — hash chains and offline signature verification" },
    Row { dt: "Discipline", dd: "149 tests · clippy clean · compile-time assertions where a comment would have drifted" },
];

pub const STACK_ROWS: &[Row] = &[
    Row { dt: "Rust", dd: "axum · rusqlite/SQLx · Tauri 2 · Dioxus · whisper-rs · ed25519-dalek · sha2" },
    Row { dt: "Front end", dd: "TypeScript · React · Vite · Tailwind" },
    Row { dt: "Data", dd: "SQLite · DuckDB · NYC Open Data (Socrata) · US Census ACS" },
    Row { dt: "AI", dd: "LLM tool-calling with enforced grounding · structured outputs · agent guardrails" },
    Row { dt: "Ops", dd: "Docker · GitHub Actions (3-OS matrix) · Fly.io · Vercel" },
];
