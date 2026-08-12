//! The page's content, as data. Kept apart from layout so that adding a fifth cycle
//! is one entry in `BUILDS` rather than a hand-edit of four parallel places.

use crate::theme::{self, Theme};

#[derive(PartialEq)]
pub struct Tag {
    pub label: &'static str,
    pub live: bool,
}

/// Shape of a build's recording, declared rather than inferred.
///
/// Three of the four apps are portrait: SiteAssure is phone-shaped, Resona is a
/// responsive Tauri UI recorded at 500x940, and HouseCheck is mobile-first so its
/// content is a narrow column — the original 1320px capture was roughly 70% dead page
/// gutter and has been cropped to the column. Ziqpu is the only true desktop layout.
#[derive(PartialEq, Clone, Copy)]
pub enum Media {
    Portrait,
    Landscape,
}

/// One panel of a card's fact pager.
///
/// Slide 1 is load-bearing: a portfolio gets about fifteen seconds and most visitors
/// never click, so it carries the whole claim on its own and its `points` stay empty.
/// Slides 2-4 are depth for a reader who chooses to go further. A test enforces both
/// halves of that rule.
#[derive(PartialEq)]
pub struct Slide {
    pub title: &'static str,
    pub lead: &'static str,
    pub points: &'static [&'static str],
}

#[derive(PartialEq)]
pub struct Build {
    pub name: &'static str,
    pub cycle: &'static str,
    pub theme: &'static Theme,
    /// Path under this repo's tree, joined to `REPO` at render time.
    pub path: &'static str,
    /// The single hardest measured number, shown beside the title on every slide.
    pub stat: &'static str,
    pub tags: &'static [Tag],
    pub media: Media,
    pub video: &'static str,
    pub poster: &'static str,
    pub alt: &'static str,
    pub slides: &'static [Slide],
    /// Only the lead card carries one. Exactly one card says where to start.
    pub flag: Option<&'static str>,
    pub lead: bool,
}

const REPO: &str = "https://github.com/nessaisling-lab/pursuit-l2-portfolio/tree/main";

pub fn href(b: &Build) -> String {
    format!("{REPO}{}", b.path)
}

pub const BUILDS: &[Build] = &[
    Build {
        name: "HouseCheck",
        cycle: "CYCLE 04",
        theme: &theme::HOUSECHECK,
        path: "/cycle-4-housecheck",
        stat: "250 buildings · 26,343 violations · 149 tests",
        tags: &[
            Tag { label: "Live", live: true },
            Tag { label: "Rust · axum", live: false },
            Tag { label: "Ed25519", live: false },
        ],
        media: Media::Portrait,
        video: "./cycle-4-housecheck/screenshots/workflow.mp4",
        poster: "./cycle-4-housecheck/screenshots/workflow-poster.jpg",
        alt: "HouseCheck: searching an address through to a Building Health Card",
        slides: &[
            Slide {
                title: "What it is",
                lead: "Carfax for NYC apartments. Type an address, get a Building Health Card \
                       scored from public city data — and export a record a stranger can \
                       cryptographically verify.",
                points: &[],
            },
            Slide {
                title: "How it works",
                lead: "Six public datasets, one artifact, no database server.",
                points: &[
                    "HPD violations, 311, PLUTO, DOB, DOHMH and Census ACS, ingested with \
                     completeness checks and block-compressed per building — measured 9.89×.",
                    "A read-only SQLite artifact baked into the Docker image: no database server, \
                     no connection pool, no cold-start query.",
                    "Search on the curated path answers in 137–157 ms warm.",
                ],
            },
            Slide {
                title: "The hard part",
                lead: "Signing the document turned out not to be enough.",
                points: &[
                    "A forger who rewrites a row, recomputes the whole chain and signs it with \
                     their own keypair produces a document that verifies as intact — every check \
                     inside it passes, because it is internally consistent.",
                    "A row I rewrote to “NO VIOLATIONS OF ANY KIND AT THIS ADDRESS” passed cleanly.",
                    "Fixed by publishing the public key at a stable endpoint, so a reader has \
                     something independent to compare against. The same forgery is now rejected.",
                ],
            },
            Slide {
                title: "What it doesn't do",
                lead: "Stated here rather than discovered later.",
                points: &[
                    "250 buildings — about 0.1% of the city.",
                    "Class I violations are excluded; 753 records were skipped at ingest and the \
                     card says so.",
                    "A signal, not a legal ruling. It does not give legal advice.",
                ],
            },
        ],
        flag: Some("Start here"),
        lead: true,
    },
    Build {
        name: "Ziqpu",
        cycle: "CYCLE 03",
        theme: &theme::ZIQPU,
        path: "/cycle-3-ziqpu",
        stat: "Two agents · offline ephemeris · no keys, no network",
        tags: &[
            Tag { label: "Rust · Dioxus", live: false },
            Tag { label: "Tool-calling", live: false },
            Tag { label: "Local models", live: false },
        ],
        media: Media::Landscape,
        video: "./cycle-3-ziqpu/screenshots/workflow.mp4",
        poster: "./cycle-3-ziqpu/screenshots/workflow-poster.jpg",
        alt: "Ziqpu: benchmarking the machine and recommending a local model",
        slides: &[
            Slide {
                title: "What it is",
                lead: "A consumer astrology decision tool with two visible agents — one measures, \
                       one interprets. The separation is the integrity guarantee, enforced by \
                       architecture rather than by a disclaimer.",
                points: &[],
            },
            Slide {
                title: "How it works",
                lead: "Measurement and meaning are different programs.",
                points: &[
                    "Hamun-ana computes positions and aspects and returns structured JSON only. \
                     It never interprets.",
                    "Ungasaga turns those measures into a reading in three beats — measured, \
                     meaning, reminder — and refuses to give advice.",
                    "The measurement sequence is fixed in code rather than chosen by the model, so \
                     a language model cannot influence the arithmetic.",
                ],
            },
            Slide {
                title: "The hard part",
                lead: "An unknown birth time is honest.",
                points: &[
                    "If the hour is unknown the app withholds the Ascendant and Midheaven rather \
                     than guessing them — an absent answer stated, never invented.",
                    "It benchmarks the machine and recommends the largest quantisation that \
                     actually fits: on this laptop, Tier Medium → Qwen3 14B at Q6_K.",
                    "Nothing is downloaded until you say so.",
                ],
            },
            Slide {
                title: "What it doesn't do",
                lead: "For reflection and entertainment.",
                points: &[
                    "Astrological interpretation is traditional and symbolic — not statements of \
                     fact, not predictions, not guarantees.",
                    "Not financial, medical, legal or psychological guidance. The interpreter is \
                     built to decline advice rather than to disclaim it in a footer.",
                ],
            },
        ],
        flag: None,
        lead: false,
    },
    Build {
        name: "SiteAssure",
        cycle: "CYCLE 02",
        theme: &theme::SITEASSURE,
        path: "/cycle-2-siteassure",
        stat: "18 commands, zero stubs · 100% audit integrity · 3-OS CI",
        tags: &[
            Tag { label: "Rust", live: false },
            Tag { label: "Hash chain", live: false },
            Tag { label: "Offline first", live: false },
        ],
        media: Media::Portrait,
        video: "./cycle-2-siteassure/screenshots/workflow.mp4",
        poster: "./cycle-2-siteassure/screenshots/workflow-poster.jpg",
        alt: "SiteAssure: daily log, records, trade-risk dashboard and audit trail",
        slides: &[
            Slide {
                title: "What it is",
                lead: "A tamper-evident, offline-first OSHA compliance logger driven by voice. \
                       Every create, amend and delete is chained and hash-verified — nothing is \
                       silently altered or removed.",
                points: &[],
            },
            Slide {
                title: "How it works",
                lead: "A safety record is a legal document before it is anything else.",
                points: &[
                    "An append-only log where each entry's hash includes the previous entry's \
                     hash, so one altered character invalidates everything after it.",
                    "Tampering is detectable by anyone holding the file — not only by whoever runs \
                     the server.",
                    "Egress hardened to an explicit allowlist rather than a kill-switch, so a new \
                     call site cannot quietly reach the internet by default.",
                ],
            },
            Slide {
                title: "The hard part",
                lead: "The audit log, read a second way, is a second product.",
                points: &[
                    "Because every capture is already timestamped and chained, the trade-risk \
                     dashboard needs no additional data entry.",
                    "3 open flags, 100% audit integrity, 1m 48s average capture time, benchmarked \
                     against OSHA enforcement weights for NAICS 23 in the NY–Newark market.",
                    "This is the same hash chain HouseCheck's export uses two cycles later. It was \
                     proven here first, on OSHA records.",
                ],
            },
            Slide {
                title: "What it doesn't do",
                lead: "Feature-complete and uninstallable.",
                points: &[
                    "There is no signed installer yet, so running it means building from source. \
                     That is the honest status and it is tracked as the next task.",
                    "Its upstream repo still describes only the voice-transcription clone it grew \
                     out of, so a visitor landing there never learns SiteAssure exists.",
                ],
            },
        ],
        flag: None,
        lead: false,
    },
    Build {
        name: "Resona",
        cycle: "CYCLE 01",
        theme: &theme::RESONA,
        path: "/cycle-1-resona",
        stat: "100% on-device · one core, four GPU backends",
        tags: &[
            Tag { label: "Rust · Tauri 2", live: false },
            Tag { label: "whisper.cpp", live: false },
            Tag { label: "On-device", live: false },
        ],
        media: Media::Portrait,
        video: "./cycle-1-resona/screenshots/workflow.mp4",
        poster: "./cycle-1-resona/screenshots/workflow-poster.jpg",
        alt: "Resona: checking the machine and choosing a speech model size",
        slides: &[
            Slide {
                title: "What it is",
                lead: "Privacy-first voice-to-text. 100% on-device via whisper.cpp — a property you \
                       can verify by unplugging the network and watching it keep working.",
                points: &[],
            },
            Slide {
                title: "How it works",
                lead: "Dictation is the one input that is inherently sensitive.",
                points: &[
                    "Microphone capture → voice-activity detection → incremental transcription, \
                     with results appearing as you speak rather than after you stop.",
                    "Tier gating is enforced in the Rust core rather than in the UI, so an \
                     entitlement check is not a frontend suggestion.",
                    "One codebase, four GPU backends — Tauri 2 builds every target from the same \
                     source.",
                ],
            },
            Slide {
                title: "The hard part",
                lead: "Making the privacy claim testable rather than promised.",
                points: &[
                    "Every mainstream option streams your audio to someone else's server, and the \
                     privacy policy is the only thing standing between the recording and a \
                     training corpus.",
                    "Here the guarantee is checkable by anyone in ten seconds: disconnect the \
                     network and it still transcribes.",
                    "On first run it profiles the machine and recommends a model size rather than \
                     making the user guess between tiny and large.",
                ],
            },
            Slide {
                title: "What it doesn't do",
                lead: "One defect kept visible on purpose.",
                points: &[
                    "The hardware probe reports 32041 GB RAM on a 32 GB machine — megabytes \
                     labelled as gigabytes.",
                    "It is documented in the cycle README rather than cropped out of the \
                     screenshot, because it is exactly the class of bug that survives by looking \
                     like a big impressive number rather than a wrong one.",
                ],
            },
        ],
        flag: None,
        lead: false,
    },
];

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
    Row { dt: "Backends", dd: "axum · rusqlite / SQLx · Tauri 2 · Dioxus · whisper-rs · cpal" },
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The pager renders one dot per slide and assumes a fixed count; a build with three
    /// would render a dead dot that pages to nothing.
    #[test]
    fn every_build_has_four_slides() {
        for b in BUILDS {
            assert_eq!(b.slides.len(), 4, "{} has the wrong slide count", b.name);
        }
    }

    /// Slide 1 must stand alone, because most visitors never press next. If it ever grows
    /// bullets it has started depending on slides that begin hidden.
    #[test]
    fn slide_one_is_self_contained() {
        for b in BUILDS {
            let first = &b.slides[0];
            assert!(!first.lead.is_empty(), "{} slide 1 has no lead", b.name);
            assert!(
                first.points.is_empty(),
                "{} slide 1 has bullets -- it must carry the claim in its lead alone",
                b.name
            );
            assert!(!b.stat.is_empty(), "{} has no headline stat", b.name);
        }
    }

    /// Exactly one card says where to start. Two would be no hierarchy at all.
    #[test]
    fn exactly_one_lead_card() {
        assert_eq!(BUILDS.iter().filter(|b| b.lead).count(), 1);
        assert_eq!(BUILDS.iter().filter(|b| b.flag.is_some()).count(), 1);
    }

    /// Every slide after the first earns its place by carrying points.
    #[test]
    fn depth_slides_have_points() {
        for b in BUILDS {
            for s in &b.slides[1..] {
                assert!(!s.points.is_empty(), "{} slide '{}' is empty", b.name, s.title);
            }
        }
    }
}
