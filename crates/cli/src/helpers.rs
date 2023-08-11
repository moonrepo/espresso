use starbase_styles::color::{create_style, Color};

pub fn start_checkpoint<T: AsRef<str>>(label: T) {
    println!(
        "{} {}",
        create_style(Color::Cyan as u8).bold().style("===>"),
        label.as_ref()
    );
}
