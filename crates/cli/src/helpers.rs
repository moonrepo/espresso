use console::{style, Style};
use dialoguer::theme::ColorfulTheme;
use espresso_package::Package;
use starbase_styles::color::{create_style, Color, OwoStyle};
use std::future::Future;

pub fn create_theme() -> ColorfulTheme {
    ColorfulTheme {
        defaults_style: Style::new().for_stderr().color256(Color::Pink as u8),
        prompt_style: Style::new().for_stderr(),
        prompt_prefix: style("?".to_string())
            .for_stderr()
            .bold()
            .color256(Color::Yellow as u8),
        prompt_suffix: style("›".to_string())
            .for_stderr()
            .bold()
            .color256(Color::Gray as u8),
        success_prefix: style("✔".to_string())
            .for_stderr()
            .bold()
            .color256(Color::Green as u8),
        success_suffix: style("·".to_string())
            .for_stderr()
            .bold()
            .color256(Color::Gray as u8),
        error_prefix: style("✘".to_string())
            .for_stderr()
            .bold()
            .color256(Color::Red as u8),
        error_style: Style::new().for_stderr().color256(Color::Pink as u8),
        hint_style: Style::new().for_stderr().color256(Color::Purple as u8),
        values_style: Style::new().for_stderr().color256(Color::Purple as u8),
        active_item_style: Style::new().for_stderr().color256(Color::Teal as u8),
        inactive_item_style: Style::new().for_stderr(),
        active_item_prefix: style("❯".to_string())
            .for_stderr()
            .bold()
            .color256(Color::Teal as u8),
        inactive_item_prefix: style(" ".to_string()).for_stderr(),
        checked_item_prefix: style("✔".to_string())
            .for_stderr()
            .bold()
            .color256(Color::Teal as u8),
        unchecked_item_prefix: style("✔".to_string())
            .for_stderr()
            .bold()
            .color256(Color::GrayLight as u8),
        picked_item_prefix: style("❯".to_string())
            .for_stderr()
            .bold()
            .color256(Color::Teal as u8),
        unpicked_item_prefix: style(" ".to_string()).for_stderr(),
        ..ColorfulTheme::default()
    }
}

pub fn start_checkpoint<T: AsRef<str>>(label: T) {
    println!(
        "{} {}",
        create_style(Color::Yellow as u8).bold().style("==>"),
        OwoStyle::new().bold().style(label.as_ref()),
    );
}

pub async fn loop_packages<'pkg, F, Fut>(
    packages: Vec<&'pkg Package>,
    func: F,
) -> miette::Result<()>
where
    F: Fn(&'pkg Package) -> Fut,
    Fut: Future<Output = miette::Result<()>>,
{
    let last_index = packages.len() - 1;

    for (index, package) in packages.iter().enumerate() {
        start_checkpoint(package.name());

        func(package).await?;

        if index != last_index {
            println!();
        }
    }

    Ok(())
}

#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }};
}
