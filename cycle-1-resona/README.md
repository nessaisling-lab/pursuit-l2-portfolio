# Resona — Cycle 1

**Privacy-first voice-to-text that never sends your audio anywhere.** Live streaming dictation
and file transcription, entirely on-device via whisper.cpp.

**[Source](https://github.com/nessaisling-lab/L2-Project-Resona)**

![Resona](./screenshots/01-main.png)

---

## The problem

Dictation is one of the few things people do with a computer where the input is *inherently*
sensitive — therapy notes, medical dictation, legal drafting, journalism with a source on the
line. Every mainstream option streams that audio to someone else's server, and the privacy
policy is the only thing standing between the recording and a training corpus.

## The solution

Run the model locally. No cloud speech-to-text, no account required, no audio leaving the
machine — a property you can verify by unplugging the network and watching it keep working.

## Key features

- **Live streaming dictation** — microphone capture → voice-activity detection → incremental
  transcription, with results appearing as you speak rather than after you stop
- **File transcription** for existing recordings
- **Grammar review pass** — local rule-based cleanup, with an AI hook behind the paid tier
- **Tier gating enforced in the Rust core**, not in the UI, so entitlement checks are not a
  frontend suggestion
- **One codebase, four GPU backends** — the Rust core carries straight to mobile, since Tauri 2
  builds every target from the same source

## Architecture

```
src/                    React + TypeScript
  App.tsx               load model, live dictation, file upload, review, paywall
  lib/tauri.ts          invoke() wrappers, event listeners, Web Audio decode
  lib/grammar.ts        local rule-based reviewer (free) / AI hook (pro)
src-tauri/              Rust
  whisper.rs            whisper-rs wrapper — load model, transcribe buffer
  audio.rs              cpal mic capture → 16 kHz mono f32
  vad.rs                energy-based voice-activity detection
  streaming.rs          capture → VAD → incremental whisper → events
  licensing.rs          tier / entitlements / license validation
```

## Tech stack

Rust · Tauri 2 · whisper-rs (whisper.cpp) · cpal · React · TypeScript

## Status

Feature-complete and **uninstallable** — no signed release build yet, so running it means
compiling from source with a C toolchain and CMake. Same honest wall as Cycle 2.

*The screenshot above is the real interface, served from the project's built `dist/`. Because
Tauri renders its UI as a web view, the frontend runs in a browser without the Rust core
attached — which is enough to show the interface honestly, and is why this shot exists without
a 30-minute whisper.cpp compile.*
