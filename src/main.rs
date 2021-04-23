mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    build_theme(&palette::Palette::default(), "sema")?;
    build_theme(&palette::Palette::chroma(), "sema chroma")?;
    build_theme(&palette::Palette::soft(), "sema soft")?;
    build_theme(&palette::Palette::soft_chroma(), "sema soft chroma")?;

    Ok(())
}

fn build_theme(palette: &palette::Palette, name: &str) -> io::Result<()> {
    let mut builder = ThemeBuilder::new(name.to_string(), Type::Dark);
    imp::add_rules(&mut builder, palette);
    builder.build().save()?;

    Ok(())
}
