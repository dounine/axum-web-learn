use image::{DynamicImage, GenericImageView, Luma, Pixel, Rgba};
use qrcode::QrCode;

use crate::error::ApiError;

fn create_rounded_mask(width: u32, height: u32, radius: u32) -> image::RgbaImage {
    const SUPERSAMPLE: u32 = 4;
    let high_width = width * SUPERSAMPLE;
    let high_height = height * SUPERSAMPLE;
    let high_radius = radius * SUPERSAMPLE;

    let mut high_mask = image::RgbaImage::new(high_width, high_height);
    let (w, h) = (high_width as f32, high_height as f32);
    let r = high_radius as f32;

    for (x, y, pixel) in high_mask.enumerate_pixels_mut() {
        let px = x as f32 + 0.5;
        let py = y as f32 + 0.5;

        let dx = if px < r { r - px } else if px > w - r { px - (w - r) } else { 0.0 };
        let dy = if py < r { r - py } else if py > h - r { py - (h - r) } else { 0.0 };

        let distance_sq = dx * dx + dy * dy;

        let alpha = if dx == 0.0 && dy == 0.0 {
            255
        } else {
            let distance = distance_sq.sqrt();
            let delta = distance - r;

            if delta <= -1.0 {
                255
            } else if delta >= 1.0 {
                0
            } else {
                ((1.0 - delta) * 255.0) as u8
            }
        };

        *pixel = Rgba([255, 255, 255, alpha]);
    }

    image::DynamicImage::ImageRgba8(high_mask)
        .resize_exact(width, height, image::imageops::FilterType::Lanczos3)
        .to_rgba8()
}

fn apply_round_corners(img: &mut image::RgbaImage, radius: u32) {
    let (width, height) = img.dimensions();
    let mask = create_rounded_mask(width, height, radius);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mask_pixel = mask.get_pixel(x, y);
        pixel[3] = (pixel[3] as f32 * (mask_pixel[3] as f32 / 255.0)) as u8;
    }
}

#[derive(Debug)]
pub struct QrcodeUtil<'u, 'm> {
    url: &'u str,
    water_mark: Option<&'m [u8]>,
    rounded: bool,
}
pub trait QrcodeRender<'m> {
    fn render(&self) -> Result<DynamicImage, ApiError>;
    fn water_mark(&mut self, water_mark: &'m [u8]) -> &mut Self;
    fn rounded(&mut self, rounded: bool) -> &mut Self;
}
impl<'u, 'm> QrcodeUtil<'u, 'm> {
    pub fn new(url: &'u str) -> Self {
        Self {
            url,
            water_mark: None,
            rounded: false,
        }
    }
}
impl<'u, 'm> QrcodeRender<'m> for QrcodeUtil<'u, 'm> {
    fn render(&self) -> Result<DynamicImage, ApiError> {
        if let Some(mark) = self.water_mark {
            create_qrcode_mark(self.url, mark, self.rounded)
        } else {
            create_qrcode(self.url)
        }
    }

    fn water_mark(&mut self, water_mark: &'m [u8]) -> &mut Self {
        self.water_mark = Some(water_mark);
        self
    }

    fn rounded(&mut self, rounded: bool) -> &mut Self {
        self.rounded = rounded;
        self
    }
}

/// 创建二维码，无水印
fn create_qrcode<T: AsRef<str>>(url: T) -> Result<DynamicImage, ApiError> {
    let code = QrCode::new(url.as_ref().as_bytes())?;
    let image = code
        .render::<Luma<u8>>()
        .min_dimensions(300, 300)
        .dark_color(Luma([0u8]))
        .light_color(Luma([255u8]))
        .build();
    // 确保最终图片固定为 300x300
    let image = DynamicImage::ImageLuma8(image)
        .resize_exact(300, 300, image::imageops::FilterType::Nearest);
    return Ok(image);
}

/// 创建二维码，水印中间
fn create_qrcode_mark<T: AsRef<str>, B: AsRef<[u8]>>(
    url: T,
    water_mark: B,
    rounded: bool,
) -> Result<DynamicImage, ApiError> {
    let code = QrCode::new(url.as_ref().as_bytes())?;
    let image = code.render::<Rgba<u8>>().min_dimensions(300, 300).build();

    let mut image = DynamicImage::ImageRgba8(image);
    let watermark = image::load_from_memory(water_mark.as_ref())?;
    let (orig_w, orig_h) = watermark.dimensions();
    let watermark = if orig_w > 50 || orig_h > 50 {
        watermark.resize_exact(50, 50, image::imageops::FilterType::Lanczos3)
    } else {
        watermark.resize_exact(50, 50, image::imageops::FilterType::Nearest)
    };

    // 先调整二维码图片到 300x300
    image = image.resize_exact(300, 300, image::imageops::FilterType::Nearest);

    let (width, height) = image.dimensions();
    let (wm_width, wm_height) = watermark.dimensions();

    let rgb_watermark = watermark.to_rgba8();

    let margin = 4;
    let bg_width = wm_width + margin * 2;
    let bg_height = wm_height + margin * 2;
    let mut bg_watermarked = DynamicImage::new_rgba8(bg_width, bg_height).to_rgba8();
    let mut watermarked = DynamicImage::new_rgba8(wm_width, wm_height).to_rgba8();

    for x in 0..bg_watermarked.width() {
        for y in 0..bg_watermarked.height() {
            bg_watermarked.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    for x in 0..wm_width {
        for y in 0..wm_height {
            watermarked.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    for x in 0..wm_width {
        for y in 0..wm_height {
            let pixel = rgb_watermark.get_pixel(x, y);
            if pixel[3] > 0 {
                watermarked.put_pixel(x, y, pixel.to_rgba());
            }
        }
    }

    if rounded {
        let bg_radius = bg_width.min(bg_height) / 4;
        let wm_radius = wm_width.min(wm_height) / 4;
        apply_round_corners(&mut bg_watermarked, bg_radius);
        apply_round_corners(&mut watermarked, wm_radius);
    }

    let x: i64 = ((width - bg_watermarked.width()) / 2).into();
    let y: i64 = ((height - bg_watermarked.height()) / 2).into();

    image::imageops::overlay(&mut image, &bg_watermarked, x, y);

    let x2: i64 = ((width - wm_width) / 2).into();
    let y2: i64 = ((height - wm_height) / 2).into();

    image::imageops::overlay(&mut image, &watermarked, x2, y2);

    Ok(image)
}
