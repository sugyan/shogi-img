use image::codecs::png::PngEncoder;
use image::imageops::{self, FilterType};
use image::RgbaImage;
use oxipng::{Options, StripChunks};
use resvg::usvg::{TreeParsing, TreeTextToPath};
use resvg::{tiny_skia, usvg, Tree};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Cursor, Read, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process_board(&["light_458x500.svg", "warm_458x500.svg", "resin_458x500.svg"])?;
    process_pieces(&["hitomoji", "hitomoji_gothic"])?;
    Ok(())
}

fn process_board(filenames: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let outdir = Path::new("shogi-img/src/data/board");
    create_dir_all(outdir)?;
    for filename in filenames {
        // load svg to resvg::Tree
        let tree = {
            let mut file = File::open(Path::new("assets/board").join(filename))?;
            let mut text = String::new();
            file.read_to_string(&mut text)?;
            Tree::from_usvg(&usvg::Tree::from_str(&text, &Default::default())?)
        };
        let size = tree.size.to_int_size();
        // render to tiny_skia::Pixmap with the original size
        if let Some(mut pixmap) = tiny_skia::Pixmap::new(size.width(), size.height()) {
            tree.render(Default::default(), &mut pixmap.as_mut());
            // convert to image::ImageBuffer
            if let Some(image) = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
            {
                // resize to 527:572 with Lanczos3 filter
                // and write to png file
                let path = outdir.join(format!(
                    "{}.png",
                    filename
                        .split('_')
                        .next()
                        .expect("filename should have `_`")
                ));
                let mut cursor = Cursor::new(Vec::new());
                imageops::resize(&image, 527, 572, FilterType::Lanczos3)
                    .write_with_encoder(PngEncoder::new(BufWriter::new(&mut cursor)))?;
                let mut file = File::create(path)?;
                file.write_all(&oxipng::optimize_from_memory(
                    cursor.get_ref(),
                    &oxipng_options(),
                )?)?;
            }
        }
    }
    Ok(())
}

fn process_pieces(names: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let outdir = Path::new("shogi-img/src/data/pieces");
    create_dir_all(outdir)?;
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    for name in names {
        create_dir_all(outdir.join(name))?;
        let tree = {
            let mut file = File::open(Path::new("assets").join(name).join("piece.svg"))?;
            let mut text = String::new();
            file.read_to_string(&mut text)?;
            let mut tree = usvg::Tree::from_str(&text, &Default::default())?;
            tree.convert_text(&fontdb);
            Tree::from_usvg(&tree)
        };
        let size = tree.size.to_int_size();
        if let Some(mut pixmap) = tiny_skia::Pixmap::new(size.width(), size.height()) {
            tree.render(Default::default(), &mut pixmap.as_mut());
            // convert to image::ImageBuffer
            if let Some(image) = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
            {
                let (width, height) = image.dimensions();
                // crop to width/8:height/4 and
                // resize to 53:56 with Lanczos3 filter
                for i in 0..4 {
                    for j in 0..8 {
                        if (i % 2 == 0 && j == 0) || (i % 2 == 1 && j == 3) {
                            continue;
                        }
                        let mut cursor = Cursor::new(Vec::new());
                        imageops::resize(
                            &imageops::crop_imm(
                                &image,
                                width * j / 8,
                                height * i / 4,
                                width / 8,
                                height / 4,
                            )
                            .to_image(),
                            53,
                            56,
                            FilterType::Lanczos3,
                        )
                        .write_with_encoder(PngEncoder::new(BufWriter::new(&mut cursor)))?;
                        let mut file = File::create(outdir.join(name).join(format!("{i}{j}.png")))?;
                        file.write_all(&oxipng::optimize_from_memory(
                            cursor.get_ref(),
                            &oxipng_options(),
                        )?)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn oxipng_options() -> Options {
    let mut opt = Options::from_preset(4);
    opt.strip = StripChunks::Safe;
    opt.optimize_alpha = true;
    opt
}
