use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer};
use resvg::usvg::{TreeParsing, TreeTextToPath};
use resvg::{tiny_skia, usvg, Tree};
use std::fs::{create_dir_all, File};
use std::io::Read;
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
            if let Some(image) =
                ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
            {
                // resize to 527:572 with Lanczos3 filter
                // and write to png file
                DynamicImage::ImageRgba8(image)
                    .resize_exact(527, 572, FilterType::Lanczos3)
                    .save(outdir.join(format!("{}.png",
                            filename
                                .split('_')
                                .next()
                                .expect("filename should have `_`")
                        )))?;
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
            if let Some(image) =
                ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
            {
                let (width, height) = image.dimensions();
                let image = DynamicImage::ImageRgba8(image);
                // crop to width/8:height/4 and
                // resize to 53:56 with Lanczos3 filter
                for i in 0..4 {
                    for j in 0..8 {
                        image
                            .crop_imm(width * j / 8, height * i / 4, width / 8, height / 4)
                            .resize(53, 56, FilterType::Lanczos3)
                            .save(outdir.join(name).join(format!("{i}{j}.png")))?;
                    }
                }
            }
        }
    }
    Ok(())
}
