"""Apply Android-only UI changes to the compact source archive before Cargo build.
The original zipped Rust workspace remains untouched and reproducible.
"""
from pathlib import Path
import shutil
import sys

root = Path(sys.argv[1]) if len(sys.argv)>1 else Path("source")
repo = Path(__file__).resolve().parent.parent
ui = root / "src" / "ui"
shutil.copy2(repo / "patches" / "mobile.rs", ui / "mobile.rs")
shutil.copytree(repo / "patches", root / "patches", dirs_exist_ok=True)

main = ui / "main_view.rs"
text = main.read_text(encoding="utf-8")
text = text.replace("fn intensity_slider(app: &mut App", "pub(super) fn intensity_slider(app: &mut App")
text = text.replace("fn status_bar(app: &mut App", "pub(super) fn status_bar(app: &mut App")
text = text.replace("fn past_jobs(app: &mut App", "pub(super) fn past_jobs(app: &mut App")
text = text.replace("pos2(ox + 313.0, oy + 8.0)", "pos2(ox + (area.width() - 87.0).max(0.0), oy + 8.0)")
text = text.replace("Txt::new(W::Semibold, 11.0, if err", "Txt::new(W::Semibold, 11.0, if err")
text = text.replace(" &msg, t, 401.0)", " &msg, t, area.width())")
text = text.replace("Color32::from_rgb(0x99, 0x9d, 0xa6)), 380.0)", "Color32::from_rgb(0x99, 0x9d, 0xa6)), (w - 20.0).max(160.0))")
main.write_text(text,encoding="utf-8")

mobile = ui / "mobile.rs"
text = mobile.read_text(encoding="utf-8")
text = text.replace("super::main_view::intensity_slider(app,ui,Rect::from_min_size(", "let busy = app.busy();\n        super::main_view::intensity_slider(app,ui,Rect::from_min_size(")
text = text.replace("vec2(bar.width()-22.0,15.0)),app.busy());", "vec2(bar.width()-22.0,15.0)),busy);")
mobile.write_text(text,encoding="utf-8")

pages = ui / "pages.rs"
text = pages.read_text(encoding="utf-8")
line = '\n#[cfg(target_os = "android")]\ninclude!("../../patches/mobile_settings.rs");\n'
if "include!(\"../../patches/mobile_settings.rs\")" not in text:
    pages.write_text(text+line,encoding="utf-8")

# Ensure OS chrome is not drawn on top of the application surface.
activity = root / "android" / "app" / "src" / "main" / "java" / "io" / "simbers" / "app" / "SimbersActivity.java"
text = activity.read_text(encoding="utf-8")
anchor = "    /** Called via JNI by the Rust UI. Never blocks the native UI thread. */"
add = """    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        getWindow().clearFlags(android.view.WindowManager.LayoutParams.FLAG_FULLSCREEN);
        getWindow().getDecorView().setSystemUiVisibility(0);
        getWindow().setStatusBarColor(android.graphics.Color.rgb(18, 18, 18));
        getWindow().setNavigationBarColor(android.graphics.Color.rgb(18, 18, 18));
    }

"""
if "protected void onCreate(Bundle savedInstanceState)" not in text:
    assert anchor in text
    text = text.replace(anchor,add+anchor)
activity.write_text(text,encoding="utf-8")

assert "pub(super) fn past_jobs" in main.read_text()
assert "pub(super) fn intensity_slider" in main.read_text()
assert "settings_mobile" in mobile.read_text()
print("Applied full Simbers mobile UI restoration patch")
