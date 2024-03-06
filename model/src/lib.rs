use anyhow::{bail, Result};
use log::error;

pub mod db;
pub mod resource;
pub mod view;


#[inline]
pub fn resource_path() -> &'static str {
    "listening\\resource"
}

fn get_mime_type(ty: &str) -> Result<String> {
    Ok(if ty.ends_with(".jpg") || ty.ends_with(".JPG") || ty.ends_with(".jpeg") {
        "image/jpeg".to_string()
    } else if ty.ends_with(".png") {
        "image/png".to_string()
    } else {
        error!("unknow extension {}", ty);
        bail!("unknow extension {}", ty)
    })
}