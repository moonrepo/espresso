use espresso_package::Package;
use starbase_styles::color::{create_style, Color, OwoStyle};
use std::future::Future;

pub fn start_checkpoint<T: AsRef<str>>(label: T) {
    println!(
        "{} {}",
        create_style(Color::Yellow as u8).bold().style("===>"),
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
