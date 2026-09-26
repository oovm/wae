use std::fs;
use std::io::Cursor;
use std::path::Path;

use icns::{IconFamily, Image};
use ico::IconDir;
use image::imageops::FilterType;
use image::{DynamicImage, ImageEncoder, RgbaImage};
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};

use crate::error::{BuildError, Result};

/// Square master raster edge length before downscaling.
const MASTER_EDGE: u32 = 1024;
const PNG_SIZES: &[u32] = &[16, 32, 48, 64, 128, 256, 512];
const ICO_SIZES: &[u32] = &[16, 32, 48, 256];
const ICNS_SIZES: &[u32] = &[16, 32, 64, 128, 256, 512];

pub struct GeneratedIcons {
    pub ico: Vec<u8>,
    pub icns: Vec<u8>,
    pub linux_png: Vec<u8>,
    pub app_png: Vec<u8>,
    pub size_pngs: Vec<(u32, Vec<u8>)>,
}

pub fn generate_icons_from_source(source: &Path) -> Result<GeneratedIcons> {
    let master = load_master_rgba(source)?;
    let ico = encode_ico(&master)?;
    let icns = encode_icns(&master)?;
    let app_png = encode_png(resize_square(&master, 512))?;
    let linux_png = encode_png(resize_square(&master, 256))?;
    let mut size_pngs = Vec::new();
    for size in PNG_SIZES {
        let bytes = encode_png(resize_square(&master, *size))?;
        size_pngs.push((*size, bytes));
    }
    Ok(GeneratedIcons { ico, icns, linux_png, app_png, size_pngs })
}

fn load_master_rgba(path: &Path) -> Result<RgbaImage> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .ok_or_else(|| BuildError::Icon(format!("icon has no extension: {}", path.display())))?;
    match ext.as_str() {
        "png" => {
            let img = image::open(path).map_err(|e| BuildError::Icon(e.to_string()))?;
            Ok(center_square_rgba(img))
        }
        "svg" => rasterize_svg(path, MASTER_EDGE),
        other => Err(BuildError::Icon(format!("unsupported icon source `.{other}` (use .png or .svg)"))),
    }
}

fn rasterize_svg(path: &Path, edge: u32) -> Result<RgbaImage> {
    let data = fs::read(path).map_err(|e| BuildError::io(path, e))?;
    let opt = Options::default();
    let tree = Tree::from_data(&data, &opt).map_err(|e| BuildError::Icon(e.to_string()))?;
    let size = tree.size();
    if size.width() <= 0.0 || size.height() <= 0.0 {
        return Err(BuildError::Icon(format!("invalid SVG dimensions in {}", path.display())));
    }
    let scale = edge as f32 / size.width().max(size.height());
    let w = (size.width() * scale).round().max(1.0) as u32;
    let h = (size.height() * scale).round().max(1.0) as u32;
    let mut pixmap = Pixmap::new(w, h).ok_or_else(|| BuildError::Icon("failed to allocate SVG pixmap".into()))?;
    let transform = Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let img = RgbaImage::from_raw(w, h, pixmap.data().to_vec())
        .ok_or_else(|| BuildError::Icon("SVG raster buffer size mismatch".into()))?;
    Ok(pad_to_square(img, edge))
}

fn pad_to_square(img: RgbaImage, edge: u32) -> RgbaImage {
    if img.width() == edge && img.height() == edge {
        return img;
    }
    let mut canvas = RgbaImage::new(edge, edge);
    image::imageops::overlay(&mut canvas, &img, ((edge - img.width()) / 2) as i64, ((edge - img.height()) / 2) as i64);
    canvas
}

fn center_square_rgba(img: DynamicImage) -> RgbaImage {
    let (w, h) = (img.width(), img.height());
    if w == h {
        return img.to_rgba8();
    }
    let side = w.min(h);
    let x = (w - side) / 2;
    let y = (h - side) / 2;
    img.crop_imm(x, y, side, side).to_rgba8()
}

fn resize_square(master: &RgbaImage, edge: u32) -> RgbaImage {
    let squared = if master.width() == master.height() {
        master.clone()
    } else {
        center_square_rgba(DynamicImage::ImageRgba8(master.clone()))
    };
    if squared.width() == edge {
        return squared;
    }
    image::imageops::resize(&squared, edge, edge, FilterType::Lanczos3)
}

fn encode_png(rgba: RgbaImage) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    encoder
        .write_image(rgba.as_raw(), rgba.width(), rgba.height(), image::ExtendedColorType::Rgba8)
        .map_err(|e| BuildError::Icon(e.to_string()))?;
    Ok(buf)
}

fn encode_ico(master: &RgbaImage) -> Result<Vec<u8>> {
    let mut dir = IconDir::new(ico::ResourceType::Icon);
    for size in ICO_SIZES {
        let rgba = resize_square(master, *size);
        let image = ico::IconImage::from_rgba_data(*size, *size, rgba.into_raw());
        let entry = ico::IconDirEntry::encode(&image).map_err(|e| BuildError::Icon(e.to_string()))?;
        dir.add_entry(entry);
    }
    let mut buf = Vec::new();
    dir.write(&mut buf).map_err(|e| BuildError::Icon(e.to_string()))?;
    Ok(buf)
}

fn encode_icns(master: &RgbaImage) -> Result<Vec<u8>> {
    let mut family = IconFamily::new();
    for size in ICNS_SIZES {
        let png = encode_png(resize_square(master, *size))?;
        let image = Image::read_png(Cursor::new(png)).map_err(|e| BuildError::Icon(e.to_string()))?;
        family.add_icon(&image).map_err(|e| BuildError::Icon(e.to_string()))?;
    }
    let mut buf = Vec::new();
    family.write(&mut buf).map_err(|e| BuildError::Icon(e.to_string()))?;
    Ok(buf)
}
