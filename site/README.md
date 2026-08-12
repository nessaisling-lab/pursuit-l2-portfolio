# site — the landing page, rendered from Rust

`../index.html` is **generated**. Edit this crate, not that file.

```bash
cargo run --manifest-path site/Cargo.toml   # rewrites ../index.html
cargo test --manifest-path site/Cargo.toml
```

CI re-renders on every push and fails if the committed `index.html` differs from what
this crate produces, so the two cannot drift apart.

## Why not `dx bundle --ssg`

Dioxus has a static-site-generation mode, and it is the wrong tool here.

GitHub Pages serves this repo under `/pursuit-l2-portfolio/`, a subdirectory. Setting
`base_path` and running `dx bundle` is [DioxusLabs/dioxus#3942][3942] — open, unassigned,
no linked fix. Leptos has the same hole from the other side ([cargo-leptos#581][581],
also open). Both fail on exactly the deployment shape this repo has.

That mode would also ship a WASM bundle and hydrate a page that has nothing to hydrate.
The landing page has no state, no routes and no interactivity beyond a background shader
that is already plain WebGL.

So this binary uses `dioxus_ssr::render_element` to render a `VirtualDom` to a `String`
and writes the file. Real Dioxus RSX, no router, no hydration, no WASM, no base path to
get wrong — the bug is unreachable rather than worked around. The browser receives the
same self-contained HTML file it always did.

[3942]: https://github.com/DioxusLabs/dioxus/issues/3942
[581]: https://github.com/leptos-rs/cargo-leptos/issues/581

## What the Rust actually buys

Not much on layout — RSX and hand-written HTML are about equally readable. The value is
in `src/theme.rs`, where the four per-project card themes stop being repeated CSS and
become typed values with an invariant attached:

```rust
assert_readable!(HOUSECHECK.accent, HOUSECHECK.surface, 450, "HouseCheck accent on surface");
```

`contrast_x100` is a `const fn`, so **a theme whose text falls below WCAG 4.5:1 on its own
background is a compile error.** WCAG needs `((c+0.055)/1.055)^2.4` and `powf` is not
const, so the sRGB curve is a precomputed fixed-point table and the rest is integer maths.

This is not hypothetical. HouseCheck's brand mint is **1.2:1** on HouseCheck's own light
ground — unreadable, and caught by hand rather than by anything systematic. It is legible
on the card only because the real app puts it inside a dark chip, which is what the theme
now encodes. Ziqpu's brass clears the bar by nine hundredths. A test pins the mint case so
that if the sampled values ever drift, the guard fails loudly instead of quietly passing.

The contrast ratios written into the generated CSS comments are computed from the same
values the assertions check, so those comments cannot go stale. That failure mode is the
one this whole portfolio is about: HouseCheck's export shipped a comment saying *"compare
with the published one"* while nothing published it.

## Layout

| file | holds |
|---|---|
| `src/theme.rs` | the four themes, contrast maths, and the const assertions |
| `src/content.rs` | copy and per-build data — adding a cycle is one entry in `BUILDS` |
| `src/main.rs` | RSX components, theme-CSS generation, and the HTML shell |
| `assets/style.css` | all styling; `/*@THEMES@*/` marks where generated themes are spliced |
| `assets/field.js` | the cursor-reactive WebGL shader, unchanged and not compiled |

The `/*@THEMES@*/` marker sits after the responsive block on purpose. `.build--lead` and
`.build` have identical specificity and media queries add none, so source order decides
which wins — putting the themes earlier silently cost the lead card its two-column layout
once already.
