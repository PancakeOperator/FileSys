mod ui_sys;
use druid::Value::String;
use ui_sys::ui::*;
use druid::{AppLauncher,  LocalizedString, WindowDesc};

use ui_sys::*;

fn main() {
    let meh = AppState {
        text: "edit me!".into()
    };
    let window = WindowDesc::new(run_app)
        .title(LocalizedString::new("FileSys").with_placeholder("Very flexible"));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(meh)
        .expect("launch failed");
}
