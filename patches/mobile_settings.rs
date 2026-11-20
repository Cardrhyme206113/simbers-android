// Android view of the original full SECTIONS settings data.
// All original settings fields remain available; desktop is untouched.
pub(super) fn settings_mobile(app: &mut App, ui: &mut Ui, rect: Rect) {
    let ctx = ui.ctx().clone();
    let p = ui.painter();
    p.rect_filled(rect, 0.0, Color32::from_rgb(21, 21, 23));
    // NativeActivity status-bar inset: keep controls out of Android system chrome.
    let body = Rect::from_min_max(rect.min + vec2(0.0, 39.0), rect.max - vec2(0.0, 14.0));
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body)
        .layout(egui::Layout::top_down(egui::Align::Min)));
    egui::ScrollArea::vertical().id_salt("android-advanced-settings")
        .auto_shrink([false,false]).show(&mut child, |ui| {
        let w = (body.width() - 24.0).max(240.0);
        ui.set_max_width(body.width());
        ui.add_space(9.0);
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("SIMPLIFIED SABERS").font(theme::font(W::MonoBold, 9.0)).color(theme::PRIMARY));
                ui.label(egui::RichText::new("SETTINGS").font(theme::font(W::Black, 25.0)).color(Color32::WHITE));
            });
            ui.add_space((w - 215.0).max(1.0));
            if ui.add_sized(vec2(60.0, 36.0), egui::Button::new("CLOSE")).clicked() {
                app.overlay = Overlay::None;
            }
        });
        ui.add_space(10.0);
        for (si, section) in SECTIONS.iter().enumerate() {
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(section.title.to_uppercase())
                        .font(theme::font(W::Bold, 18.0)).color(Color32::WHITE));
                    ui.add(egui::Label::new(egui::RichText::new(section.desc)
                        .font(theme::font(W::Regular, 11.0)).color(theme::MUTED)).wrap());
                });
            });
            ui.add_space(8.0);
            for (fi, field) in section.fields.iter().enumerate() {
                if let Some((dep, yes)) = field.depends {
                    if settings::boolean(dep, false) != yes { continue; }
                }
                let key = field.key;
                let mut changed = false;
                let mut new_value: Option<Value> = None;
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    egui::Frame::new()
                        .fill(Color32::from_rgb(26,26,28))
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(52,52,54)))
                        .corner_radius(4.0).inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.set_max_width((w - 22.0).max(190.0));
                            ui.label(egui::RichText::new(field.label)
                                .font(theme::font(W::Black, 13.0)).color(Color32::WHITE));
                            // Descriptions are the same source data as the desktop UI.
                            let desc = field.desc.iter().map(|(s,_)| *s).collect::<String>();
                            ui.add(egui::Label::new(egui::RichText::new(desc)
                                .font(theme::font(W::Regular, 11.0)).color(theme::MUTED)).wrap());
                            ui.add_space(6.0);
                            match &field.control {
                                Control::Number { min, max, step } => {
                                    let mut value = settings::number(key,*min);
                                    if ui.add(egui::DragValue::new(&mut value).range(*min..=*max)
                                        .speed(*step).fixed_decimals(2)).changed() {
                                        new_value = Some(Value::from(value.clamp(*min,*max))); changed = true;
                                    }
                                }
                                Control::Range { min, max, step, percent, unit } => {
                                    let mut value = settings::number(key,*min);
                                    let label = if *percent { "%".to_string() } else { unit.to_string() };
                                    if ui.add(egui::Slider::new(&mut value,*min..=*max)
                                        .step_by(*step).text(label)).changed() {
                                        new_value = Some(Value::from(value)); changed = true;
                                    }
                                }
                                Control::BoolSelect(options) => {
                                    let mut current = settings::boolean(key,false).to_string();
                                    egui::ComboBox::from_id_salt((si,fi,"android"))
                                        .selected_text(options.iter().find(|(v,_)| *v == current).map(|(_,l)|*l).unwrap_or(options[0].1))
                                        .width((w-56.0).max(150.0)).show_ui(ui, |ui| {
                                            for (v,l) in *options {
                                                if ui.selectable_value(&mut current,(*v).to_string(),*l).changed() {
                                                    changed=true; new_value=Some(Value::Bool(*v == "true"));
                                                }
                                            }
                                        });
                                }
                                Control::Select(options) => {
                                    let mut current = settings::get_str(key,options[0].0);
                                    egui::ComboBox::from_id_salt((si,fi,"android"))
                                        .selected_text(options.iter().find(|(v,_)| *v == current).map(|(_,l)|*l).unwrap_or(options[0].1))
                                        .width((w-56.0).max(150.0)).show_ui(ui, |ui| {
                                            for (v,l) in *options {
                                                if ui.selectable_value(&mut current,(*v).to_string(),*l).changed() {
                                                    changed=true; new_value=Some(Value::String((*v).to_string()));
                                                }
                                            }
                                        });
                                }
                                Control::NumSelect(options) => {
                                    let mut current = settings::number(key,options[0].0);
                                    egui::ComboBox::from_id_salt((si,fi,"android"))
                                        .selected_text(options.iter().find(|(v,_)| (*v-current).abs() < f64::EPSILON)
                                            .map(|(_,l)|*l).unwrap_or(options[0].1))
                                        .width((w-56.0).max(150.0)).show_ui(ui, |ui| {
                                            for (v,l) in *options {
                                                if ui.selectable_value(&mut current,*v,*l).changed() {
                                                    changed=true; new_value=Some(Value::from(*v));
                                                }
                                            }
                                        });
                                }
                                Control::Switch => {
                                    let mut on = settings::boolean(key,key == "output.autoPreview");
                                    if ui.checkbox(&mut on,"Enabled").changed() {
                                        changed=true; new_value=Some(Value::Bool(on));
                                    }
                                }
                                Control::ReplayMode => {
                                    let mut replay = app.viewer.replay_visible;
                                    ui.horizontal(|ui| {
                                        if ui.selectable_label(!replay,"Map view").clicked() { replay=false; app.viewer.set_replay_visible(false); }
                                        if ui.selectable_label(replay,"HMD view").clicked() { app.viewer.set_replay_visible(true); }
                                    });
                                }
                                Control::ReplayActions => {
                                    ui.horizontal_wrapped(|ui| {
                                        if ui.button("Import BSOR").clicked(){ app.import_replay(None); }
                                        if ui.button("Export replay").clicked(){ app.export_replay(); }
                                    });
                                }
                            }
                        });
                });
                if changed {
                    if let Some(v)=new_value { settings::set(key,v); }
                    settings::save();
                    settings::apply_runtime();
                }
                ui.add_space(5.0);
            }
            if let Some(note)=section.note {
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    ui.add(egui::Label::new(egui::RichText::new(note)
                        .font(theme::font(W::Regular,11.0)).color(theme::MUTED)).wrap());
                });
            }
        }
        ui.add_space(14.0);
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.label(egui::RichText::new("YOUTUBE / YT-DLP DISABLED ON ANDROID")
                .font(theme::font(W::MonoBold,10.0)).color(theme::MUTED));
        });
        ui.add_space(40.0);
    });
    ctx.request_repaint_after(std::time::Duration::from_millis(200));
}
