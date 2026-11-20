//! Android portrait layout using the same custom-painted Simbers controls as desktop.
//! No stock egui buttons/checkboxes; shared map engine, settings, history, viewer.
use super::{App, Overlay};
use super::icons::Icon;
use super::theme::{self, Txt, W, rgba};
use super::widgets::{self, ButtonStyle};
use eframe::egui::{self, Align2, Color32, Rect, Sense, Ui, pos2, vec2};

const PAD: f32 = 12.0;
const GAP: f32 = 7.0;

pub fn show(app: &mut App, ui: &mut Ui, rect: Rect) {
    // Android NativeActivity may report an edge-to-edge surface. Keep controls
    // below system status icons even on ROMs that ignore decor-fit flags.
    if app.overlay == Overlay::Settings {
        super::pages::settings_mobile(app, ui, rect);
        return;
    }
    if app.viewer_fullscreen && app.viewer.has_map() {
        super::viewer_ui::show(app, ui, rect.shrink2(vec2(0.0, 32.0)));
        let back = Rect::from_min_size(rect.min + vec2(12.0, 46.0), vec2(74.0, 38.0));
        if widgets::button(ui, &mut app.icons, "android-viewer-back", back, "Back", None, ButtonStyle::outline(11.0), true).clicked() {
            app.viewer_fullscreen = false;
        }
        return;
    }
    let w = (rect.width() - PAD * 2.0).max(260.0);
    let mut surface = ui.new_child(egui::UiBuilder::new()
        .max_rect(rect)
        .layout(egui::Layout::top_down(egui::Align::Min)));
    egui::ScrollArea::vertical().id_salt("android-main-scroll")
        .auto_shrink([false, false]).show(&mut surface, |ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        // Explicit inset for status bar; no controls occupy the Android clock area.
        ui.add_space(43.0);

        let (row, _) = ui.allocate_exact_size(vec2(rect.width(), 51.0), Sense::hover());
        let p = ui.painter().clone();
        let ctx = ui.ctx().clone();
        let left = row.min.x + PAD;
        let logo = Rect::from_min_size(pos2(left, row.min.y + 5.0), vec2(28.0, 33.0));
        app.icons.paint(&ctx, &p, Icon::Brand, logo, Color32::WHITE);
        let text_x = left + 36.0;
        let version_width = 53.0;
        let title_width = (w - 36.0 - version_width - 4.0).max(100.0);
        let title_style = Txt::new(W::Black, 20.0, Color32::WHITE).ls(-0.4);
        let title = theme::ellipsize(&p, "SIMPLIFIED SABERS", title_style, title_width);
        theme::text(&p, pos2(text_x, row.min.y + 23.0), Align2::LEFT_CENTER, &title, title_style);
        let ver = Rect::from_min_size(pos2(left + w - version_width, row.min.y + 12.0), vec2(version_width, 23.0));
        theme::boxed(&p, ver, 4.0, Color32::from_rgb(22,22,22), 1.0, rgba(255,255,255,0.17));
        theme::text(&p, ver.center(), Align2::CENTER_CENTER, super::VERSION, Txt::new(W::MonoBold, 10.0, theme::MUTED));

        let (row, _) = ui.allocate_exact_size(vec2(rect.width(), 52.0), Sense::hover());
        let x = row.min.x + PAD;
        let y = row.min.y + 2.0;
        let available = w - 3.0 * GAP;
        let widths = [available * 0.34, available * 0.32, available * 0.22, available * 0.12];
        let labels = ["Upload File", app.analyze_label, "Export", "×"];
        let styles = [
            ButtonStyle::outline(11.0),
            ButtonStyle::primary(11.0),
            ButtonStyle::secondary(11.0),
            ButtonStyle::secondary(20.0),
        ];
        let mut x = x;
        for i in 0..4 {
            let area = Rect::from_min_size(pos2(x,y), vec2(widths[i],43.0));
            let enabled = i != 1 || (app.analyze_enabled && !app.busy());
            if widgets::button(ui, &mut app.icons, &format!("android-action-{i}"), area, labels[i], None, styles[i], enabled).clicked() {
                match i {
                    0 => app.pick_file(),
                    1 => app.run_analysis(),
                    2 => app.export_open = !app.export_open,
                    _ => app.back(),
                }
            }
            x += widths[i] + GAP;
        }
        // Match the original export dropdown: two actions, never a replacement
        // generic desktop dialog.
        if app.export_open {
            let (row, _) = ui.allocate_exact_size(vec2(rect.width(), 43.0), Sense::hover());
            let x = row.min.x + PAD;
            let bw = (w - GAP) / 2.0;
            let b1 = Rect::from_min_size(pos2(x, row.min.y), vec2(bw, 36.0));
            let b2 = Rect::from_min_size(pos2(x + bw + GAP, row.min.y), vec2(bw, 36.0));
            if widgets::button(ui, &mut app.icons, "android-exp-map", b1, "Export Map", Some((Icon::Download, 13.0)), ButtonStyle::header(10.0), app.map_zip.is_some()).clicked() {
                app.export_map();
                app.export_open = false;
            }
            if widgets::button(ui, &mut app.icons, "android-exp-replay", b2, "Export Replay", Some((Icon::Open, 13.0)), ButtonStyle::header(10.0), app.map_zip.is_some()).clicked() {
                app.export_replay();
                app.export_open = false;
            }
        }
        ui.add_space(6.0);
        let (row, _) = ui.allocate_exact_size(vec2(rect.width(), 47.0), Sense::hover());
        let toggle_icons = [Icon::Bomb, Icon::Walls, Icon::Arcs, Icon::Bot];
        let toggle_text = ["Bombs", "Walls", "Arcs", "8-bot"];
        let tw = (w - GAP * 3.0) / 4.0;
        let p = ui.painter().clone();
        for i in 0..4 {
            let button = Rect::from_min_size(pos2(row.min.x + PAD + (tw+GAP)*i as f32,row.min.y),vec2(tw,42.0));
            let on = match i {0=>app.features.bombs,1=>app.features.walls,2=>app.features.arcs,_=>app.features.bot_optimizer};
            let response = widgets::interact(ui, ("android-feature",i), button, !app.busy());
            let op = if app.busy() {0.4} else {1.0};
            theme::boxed(&p, button, 4.0, theme::fade(if on {theme::PRIMARY} else {Color32::TRANSPARENT}, op), 2.0,
                theme::fade(if on {theme::PRIMARY} else {theme::BORDER}, op));
            let c = theme::fade(if on {Color32::WHITE} else {theme::MUTED},op);
            let icon = Rect::from_center_size(pos2(button.center().x,button.center().y - 7.0),vec2(19.0,19.0));
            app.icons.paint(ui.ctx(), &p, toggle_icons[i], icon, c);
            theme::text(&p,pos2(button.center().x,button.center().y + 12.0),Align2::CENTER_CENTER,
                toggle_text[i],Txt::new(W::Bold,9.0,c));
            if response.clicked() {
                match i {0=>app.features.bombs=!app.features.bombs,1=>app.features.walls=!app.features.walls,
                    2=>app.features.arcs=!app.features.arcs,_=>app.features.bot_optimizer=!app.features.bot_optimizer}
                if app.output.is_some() {
                    app.status = "Map objects changed. Select Create Map to rebuild.".into();
                    app.analyze_label = "Create Map";
                }
            }
        }
        ui.add_space(4.0);
        let (row, _) = ui.allocate_exact_size(vec2(rect.width(), 45.0), Sense::hover());
        let x = row.min.x + PAD;
        let gear = Rect::from_min_size(pos2(x,row.min.y),vec2(41.0,38.0));
        if widgets::button(ui,&mut app.icons,"android-settings",gear,"",Some((Icon::Gear,17.0)),ButtonStyle::outline(11.0),true).clicked() {
            app.overlay = Overlay::Settings;
        }
        let bar = Rect::from_min_size(pos2(x+48.0,row.min.y),vec2(w-48.0,38.0));
        let p = ui.painter().clone();
        theme::boxed(&p,bar,4.0,Color32::from_rgb(17,17,17),2.0,theme::BORDER);
        theme::text(&p,pos2(bar.min.x+9.0,bar.min.y+9.0),Align2::LEFT_TOP,"INTENSITY",
            Txt::new(W::Black,10.0,theme::MUTED).ls(0.45));
        theme::text(&p,pos2(bar.max.x-9.0,bar.min.y+9.0),Align2::RIGHT_TOP,
            &crate::mapgen::select::intensity_display_text(app.intensity).to_uppercase(),
            Txt::new(W::Black,10.0,Color32::WHITE));
        super::main_view::intensity_slider(app,ui,Rect::from_min_size(
            pos2(bar.min.x + 11.0,bar.min.y + 21.0),vec2(bar.width()-22.0,15.0)),app.busy());

        let (row, _) = ui.allocate_exact_size(vec2(rect.width(), 56.0), Sense::hover());
        super::main_view::status_bar(app,ui,Rect::from_min_size(
            pos2(row.min.x+PAD,row.min.y+3.0),vec2(w,45.0)));

        let vh = if app.viewer.has_map() {320.0} else {155.0};
        let (row, _) = ui.allocate_exact_size(vec2(rect.width(), vh + 17.0), Sense::hover());
        let viewer = Rect::from_min_size(pos2(row.min.x+PAD,row.min.y+3.0),vec2(w,vh));
        let p=ui.painter().clone();
        theme::boxed(&p,viewer,5.0,theme::BG_DARK,1.0,theme::BORDER);
        if app.viewer.has_map() {
            super::viewer_ui::show(app,ui,viewer.shrink(1.0));
        } else {
            let c=viewer.center();
            app.icons.paint(ui.ctx(),&p,Icon::Music,Rect::from_center_size(c+vec2(0.0,-32.0),vec2(32.0,32.0)),theme::MUTED);
            theme::text(&p,c+vec2(0.0,7.0),Align2::CENTER_CENTER,"BUILT-IN VIEWER",
                Txt::new(W::Black,14.0,Color32::WHITE).ls(0.3));
            theme::text_wrapped(&p,c+vec2(0.0,29.0),Align2::CENTER_CENTER,
                "UPLOAD AUDIO OR A BEAT SABER ZIP TO PREVIEW",Txt::new(W::MonoBold,9.0,theme::MUTED),w-30.0);
        }
        // Original job cards and action buttons; use the shared implementation.
        let (row, _) = ui.allocate_exact_size(vec2(rect.width(), 360.0),Sense::hover());
        super::main_view::past_jobs(app,ui,Rect::from_min_max(
            pos2(row.min.x+PAD,row.min.y),pos2(row.min.x+PAD+w,row.max.y - 5.0)));
        ui.add_space(24.0);
    });
}
