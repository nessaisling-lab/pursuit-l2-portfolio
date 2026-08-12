# Pursuit L2 — four cycles, four shipped products

**Stan (Aisling) Leiva-Davila** · AI-Native Builder, inaugural cohort
**[nessaisling-lab.github.io/pursuit-l2-portfolio](https://nessaisling-lab.github.io/pursuit-l2-portfolio/)**
[ness.aisling@gmail.com](mailto:ness.aisling@gmail.com) · [LinkedIn](https://linkedin.com/in/stan-leiva-davila) · [github.com/nessaisling-lab](https://github.com/nessaisling-lab)

Four builds across seven months. Every one runs. Two are live on the internet right now, two
are desktop applications you compile and launch. Most of the work is **Rust** — not because
it is fashionable, but because three of the four have a correctness property that a garbage
collector and a loose type system make harder to defend.

---

## The four builds

| Cycle | Build | What it is | Status |
|---|---|---|---|
| **4** | **[HouseCheck](./cycle-4-housecheck)** | Carfax for NYC apartments — a Building Health Card scored from public city data, with an export a stranger can cryptographically verify | **[Live](https://housecheck-wine.vercel.app)** |
| **3** | **[Ziqpu](./cycle-3-ziqpu)** | Consumer astrology decision tool. Two visible agents — one measures, one interprets — and the separation *is* the integrity guarantee | Runs locally |
| **2** | **[SiteAssure](./cycle-2-siteassure)** | Tamper-evident OSHA compliance logger. Append-only hash chain over safety records | Runs locally |
| **1** | **[Resona](./cycle-1-resona)** | Privacy-first voice-to-text. 100% on-device via whisper.cpp — audio never leaves the machine | Runs locally |

---

## What I would want you to look at

If you only open one thing, open **[HouseCheck's export design](./cycle-4-housecheck#the-export-is-the-hard-part)**.
It is the piece of work I would defend in an interview.

The short version: a tenant lawyer can read a building's violations on any city website. What
they cannot do is put that reading in front of a court, because a printout is unverifiable.
HouseCheck exports the record as a document carrying an append-only hash chain and an Ed25519
signature — so opposing counsel can re-check it offline, without trusting us and without
reaching our servers.

**And the part I am prouder of than the cryptography:** while verifying it in production, I
wrote an independent verifier in a different language and found that signing alone was not
enough. A forger who rewrites a row, recomputes the whole chain and signs it with their own
keypair produces a document that verifies as intact — because it is internally consistent. The
fix was publishing the public key at a stable endpoint so a reader has something to compare
against. That hole existed because the code's own comment said *"compare with the published
one"* while nothing published it.

---

## How I work

Three habits that show up in every repo here, and each of them came from getting something
wrong first.

**Measure, don't estimate.** Numbers in these READMEs are measured and say so. Where a figure
is arithmetic rather than observation, it is labelled derived. Where it is unverified, it says
unverified. HouseCheck's backlog carries the rule in writing: *an item with no reason is a wish,
not a task.*

**Verify against production, not against the repo.** Nearly every real defect I found in the
final week was invisible from inside the editor — a search box that answered a Manhattan
address with a Brooklyn building, a failed lookup that silently kept the previous result on
screen, an assistant that answered a different question than the one asked. All found by using
the deployed product and checking whether what it *said* matched what it *knew*.

**An absent answer is still an answer.** Repeatedly across these builds, the honest design has
three states rather than two: signed / unsigned-but-intact / tampered; a repair-time median /
"nothing has been closed" / genuinely no data. Collapsing the middle state is how software ends
up stating something reassuring that it has not earned.

---

## Stack across the four

**Rust** — axum, SQLx/rusqlite, Tauri 2, whisper-rs, ed25519-dalek, sha2
**TypeScript/React** — Vite, Tailwind
**Data** — SQLite, DuckDB, NYC Open Data (Socrata), US Census ACS
**AI** — LLM tool-calling loops with enforced grounding, structured outputs, agent guardrails
**Ops** — Docker, GitHub Actions (3-OS matrix), Fly.io, Vercel

---

*Pursuit AI-Native Builder — inaugural cohort, 600+ hours over seven months.*
