use std::fs::File;
use std::io::BufWriter;
use once_cell::sync::OnceCell;
use resvg::{tiny_skia, tiny_skia::Pixmap, usvg};
use snafu::{OptionExt, ResultExt, Snafu};
use std::path::Path;
use std::sync::Arc;
use image::{ImageFormat, RgbImage};
use usvg::{fontdb, Options, Tree};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Io {file}: {source}"))]
    Io {
        file: String,
        source: std::io::Error,
    },
    #[snafu(display("Error to parse: {source}"))]
    Parse { source: usvg::Error},
    #[snafu(display("Encode fail: {source}"))]
    Image { source: image::ImageError },
    #[snafu(display("Error while create a Pixmap"))]
    Pixmap { },
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) fn get_or_init_fontdb(fonts: Option<Vec<&[u8]>>) -> Arc<fontdb::Database> {
    static GLOBAL_FONT_DB: OnceCell<Arc<fontdb::Database>> = OnceCell::new();
    GLOBAL_FONT_DB
        .get_or_init(|| {
            let mut fontdb = fontdb::Database::new();
            if let Some(value) = fonts {
                for item in value.iter() {
                    fontdb.load_font_data((*item).to_vec());
                }
            } else {
                fontdb.load_system_fonts();
            }
            Arc::new(fontdb)
        })
        .clone()
}

/// Converts svg to png.
pub fn svg_to_png(svg: &str, dest_path:&str, file_name:&str) -> Result<()> {
    let fontdb = get_or_init_fontdb(None);

    // Carica l'SVG in un usvg::Tree
    let tree = Tree::from_str(
        svg,
        &Options {
            fontdb,
            ..Default::default()
        },
    ).context(ParseSnafu {})?;

    let scale_factor = 2.0;
    let mut pixmap = Pixmap::new((tree.size().width() * 2.0) as u32,
                                 (tree.size().height() * 2.0) as u32).context(PixmapSnafu)?;
    resvg::render(&tree, tiny_skia::Transform::from_scale(scale_factor, scale_factor), &mut pixmap.as_mut());

    // Converte i dati RGBA in RGB
    let rgba_data = pixmap.data();
    let mut rgb_data = Vec::with_capacity((pixmap.width() * pixmap.height() * 3) as usize);
    for chunk in rgba_data.chunks(4) {
        rgb_data.push(chunk[0]); // R
        rgb_data.push(chunk[1]); // G
        rgb_data.push(chunk[2]); // B
    }

    // Crea un'immagine RGB utilizzando la crate image
    let rgb_image = RgbImage::from_raw(pixmap.width(), pixmap.height(), rgb_data)
        .ok_or(Error::Pixmap {})?;

    // Salva l'immagine come PNG RGB
    let path = format!("{}{}", dest_path, file_name);
    let file = File::create(Path::new(&path)).context(IoSnafu { file: path.clone() })?;
    let mut w = BufWriter::new(file);
    rgb_image.write_to(&mut w, ImageFormat::Png).context(ImageSnafu)?;
    Ok(())
}

