use starbase_styles::color::{create_style, Color, OwoStyle};

pub fn start_checkpoint<T: AsRef<str>>(label: T) {
    println!(
        "{} {}",
        create_style(Color::Yellow as u8).bold().style("===>"),
        OwoStyle::new().bold().style(label.as_ref()),
    );
}
