# shogi-img

[![Rust](https://github.com/sugyan/shogi-img/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/sugyan/shogi-img/actions/workflows/rust.yml)

`shogi-img` is a library for generating images that visualize the position in Shogi (Japanese chess).

## Example

![](shogi-img/images/example.png)

```rust
use shogi_img::pos2img;
use shogi_usi_parser::FromUsi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pos = shogi_core::PartialPosition::from_usi(
        "sfen 3sks3/9/4S4/9/1+B7/9/9/9/9 b S2rb4g4n4l18p 1",
    )?;
    pos2img(&pos).save("out.png")?;
    Ok(())
}
```

## Image resources

This repository includes images sourced from [Shogi Images](https://sunfish-shogi.github.io/shogi-images), which are distributed under the [CC0 1.0 Universal](https://github.com/sunfish-shogi/shogi-images?tab=CC0-1.0-1-ov-file#readme) license.

## Font resources

This repository includes the [MonaSpace](https://github.com/githubnext/monaspace) font by GitHub, licensed under the [SIL Open Font License 1.1](https://github.com/githubnext/monaspace?tab=OFL-1.1-1-ov-file#readme).
