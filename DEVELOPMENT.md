# Development

How the files under `shogi-img/src/data/` are made. None of it runs during a
build — they are committed, and `generator.rs` reads them with `include_bytes!`.

## Two directories, one line between them

| | |
|---|---|
| `assets/` | Inputs. SVG sources, the full font. **Not published** — it sits outside the package directory, so `cargo package` never sees it. |
| `shogi-img/src/data/` | What the crate compiles in. **Published**, and therefore in every download. |

Anything large that is only an input belongs on the left. The full font spent
0.4.0 and 0.5.0 on the right, which is very nearly all of what those releases
weighed.

## Board and piece images

From the repository root — the paths inside `create-data` are relative to it:

```sh
cargo run -p create-data
```

Boards: each `assets/board/*_458x500.svg` rendered with resvg, resized to
527×572 with a Lanczos3 filter, then oxipng. Pieces: one `piece.svg` sheet per
style, rendered, cropped into an 8×4 grid, each cell resized to 53×56, then
oxipng.

**Check `git diff` afterwards. One half of this reproduces and the other does
not, and the half that does not says nothing about it.**

### The boards reproduce, because `Cargo.lock` is committed

They did not always. `light.png` and `warm.png` — and only those two — moved
whenever the toolchain moved: 7 and 2 pixels of 301444, at a delta of 1.
Invisible, and enough to make every regeneration a diff.

Only those two because **only those two embed a photograph.**
`light_458x500.svg` and `warm_458x500.svg` are 415 KB and 342 KB and each holds
one 458×500 PNG of woodgrain, placed at 439.215 × 479.792 — a fractional
downscale, so it has to be resampled. The resampler is **tiny-skia**, which
`create-data` never names: it arrives under `resvg` as `^0.11`, so before the
lock was committed its patch version was whatever resolved that day.
`resin_458x500.svg` is 29 KB of pure vector, rasterises identically every time,
and never moved.

So a `resvg` or `tiny-skia` bump will move `light` and `warm` again, by a pixel
or two, and leave `resin` alone. That is expected, not a defect. Regenerate them
in the same commit as the bump.

Within one lock, output is byte-identical run to run — checked.

### The gothic pieces do not reproduce, and depend on the machine's fonts

`assets/hitomoji_gothic/piece.svg` is the only source with `<text>` left in it.
It asks for `BIZ UDPGothic` Bold and falls back to `sans-serif`, and
`create-data` resolves that with `fontdb.load_system_fonts()` — which the lock
does not pin, because fonts are not dependencies. Without that face installed,
all 28 gothic pieces come out in whatever the fallback is: measured on `01.png`,
990 of 2968 pixels at full contrast. A different typeface, not a rounding
difference, and nothing warns you.

`assets/hitomoji/piece.svg` has its text already converted to paths and
reproduces anywhere. Doing the same to the gothic sheet would close this; until
then, regenerate the pieces only on a machine with `BIZ UDPGothic`, and revert
them otherwise.

## The font subset

`assets/fonts/MoralerspaceNeon-Regular.ttf` is 7.6 MB and is not published. What
the crate compiles in is the 12 KB subset beside `generator.rs`'s data, and
`OFL.txt` next to it is the license that has to travel with it.

```sh
pip install fonttools     # or: brew install fonttools

pyftsubset assets/fonts/MoralerspaceNeon-Regular.ttf \
  --text='一二三四五六七八九1234567890' \
  --output-file=shogi-img/src/data/fonts/MoralerspaceNeon-Regular.subset.ttf
```

Unlike `create-data`, this **does** reproduce the committed file — byte for
byte, checked with fonttools 4.63.0.

The command is from the review that introduced the subset, sugyan/shogi-img#2,
with one change: that one ran in place inside `src/data/fonts/`, where the full
font used to live, and took `pyftsubset`'s default output name. The input has
moved, so the output has to be named.

**The 19 characters are exactly what `generator.rs` hands to `draw_text_mut`,
and nothing else:**

| | |
|---|---|
| `1`–`9` | file numbers along the top |
| 一 二 三 四 五 六 七 八 九 | `RANK_TO_KANJI`, down the right |
| `0`–`9` | a hand's count when it is above one — up to 18, which is the only reason `0` is in there |

Adding a character to any of those three without rebuilding the subset draws
`.notdef` — a box, or nothing at all. Nothing checks this.
