use anyhow::{anyhow, Result};
use base64::prelude::*;
use imghdr::{from_bytes, Type};

/// Basically a to_string for the `imghdr::Type` enum
pub fn image_type_to_string(image_type: Type) -> &'static str {
    match image_type {
        Type::Bgp => "bgp",
        Type::Bmp => "bmp",
        Type::Exr => "exr",
        Type::Flif => "flif",
        Type::Gif => "gif",
        Type::Ico => "ico",
        Type::Jpeg => "jpeg",
        Type::Pbm => "pbm",
        Type::Pgm => "pgm",
        Type::Png => "png",
        Type::Ppm => "ppm",
        Type::Rast => "rast",
        Type::Rgb => "rgb",
        Type::Rgbe => "rgbe",
        Type::Tiff => "tiff",
        Type::Webp => "webp",
        Type::Xbm => "xbm",
    }
}

/// Encodes image bytes into their base64 representation
///
/// # Arguments
///
/// * `image` - The image bytes
pub fn encode_image(image: &[u8]) -> Result<String> {
    let image_type = from_bytes(image);

    if let Some(image_type) = image_type {
        let encoded_image = BASE64_STANDARD.encode(image);

        Ok(format!(
            "data:image/{};base64,{}",
            image_type_to_string(image_type),
            encoded_image
        ))
    } else {
        Err(anyhow!("Unable to get image type"))
    }
}
