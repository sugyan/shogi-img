# Development

How the files under `shogi-img/src/data/` are made. None of it runs during a
build — they are committed, and `generator.rs` reads them with `include_bytes!`.

## Two directories, one line between them

| | |
|---|---|
| `assets/` | Inputs. SVG sources, the full font. **Not published** — it sits outside the package directory, so `cargo package` never sees it. |
| `shogi-img/src/data/` | What the crate compiles in. **Published**, and therefore in every download. |

Anything large that is only an input belongs on the left.

## Board and piece images

From the repository root — the paths inside `create-data` are relative to it:

```sh
cargo run -p create-data
```

Boards: each `assets/board/*_458x500.svg` rendered with resvg, resized to
527×572 with a Lanczos3 filter, then oxipng. Pieces: one `piece.svg` sheet per
style, rendered, cropped into an 8×4 grid, each cell resized to 53×56, then
oxipng.

**Check `git diff` afterwards.** The boards reproduce; the gothic pieces do not.

### Boards

Reproducible as long as `Cargo.lock` is unchanged, which is why it is committed.

Bumping `resvg` or `tiny-skia` moves `light.png` and `warm.png` by a pixel or
two and leaves `resin.png` alone: those two embed a 458×500 photograph placed at
439.215 × 479.792, so tiny-skia has to resample it, while `resin` is pure
vector. Expected rather than a defect — regenerate them in the same commit as
the bump.

### Gothic pieces

`assets/hitomoji_gothic/piece.svg` is the only source with `<text>` left in it.
It asks for `BIZ UDPGothic` Bold, falls back to `sans-serif`, and `create-data`
resolves that with `fontdb.load_system_fonts()` — a lock does not pin fonts.
Without that face, all 28 gothic pieces come out in a different typeface, and
nothing warns you. Regenerate them only on a machine that has it; revert them
otherwise.

`assets/hitomoji/piece.svg` has its text already converted to paths and
reproduces anywhere. Doing the same to the gothic sheet would close this.

## The font subset

`assets/fonts/MoralerspaceNeon-Regular.ttf` is 7.6 MB and is not published. What
the crate compiles in is the 12 KB subset under `src/data/fonts/`, with
`OFL.txt` beside it — the license has to travel with the font.

```sh
pip install fonttools     # or: brew install fonttools

pyftsubset assets/fonts/MoralerspaceNeon-Regular.ttf \
  --text='一二三四五六七八九1234567890' \
  --output-file=shogi-img/src/data/fonts/MoralerspaceNeon-Regular.subset.ttf
```

This reproduces the committed file byte for byte.

**The 19 characters are exactly what `generator.rs` hands to `draw_text_mut`,
and nothing else:**

| | |
|---|---|
| `1`–`9` | file numbers along the top |
| 一 二 三 四 五 六 七 八 九 | `RANK_TO_KANJI`, down the right |
| `0`–`9` | a hand's count when it is above one — up to 18, which is the only reason `0` is in there |

Adding a character to any of those three without rebuilding the subset draws
`.notdef` — a box, or nothing at all. Nothing checks this.
