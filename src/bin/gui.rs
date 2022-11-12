use klask::Settings;
use rfb_ui::process;

fn main() -> anyhow::Result<()> {
    klask::run_derived(Settings::default(), |a| {
        process(a).expect("Processing error");
    });
    Ok(())
}
