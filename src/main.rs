slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = UI::new()?;

    ui.run()
}