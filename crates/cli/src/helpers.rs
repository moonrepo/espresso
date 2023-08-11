use starbase_styles::color::{self, Color};

pub fn start_checkpoint<T: AsRef<str>>(label: T) {
    println!(
        "{} {}",
        color::create_style(Color::Cyan as u8).bold().style("===>"),
        label.as_ref()
    );
}
