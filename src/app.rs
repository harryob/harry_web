use egui::{Align2, Color32, OpenUrl, Vec2};

pub struct HarryWeb {}

impl Default for HarryWeb {
    fn default() -> Self {
        Self {}
    }
}

impl HarryWeb {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        setup_font(&cc.egui_ctx);

        Default::default()
    }
}

fn setup_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "JetBrains Mono".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/JetBrainsMono-Regular.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "JetBrains Mono".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "JetBrains Mono".to_owned());

    ctx.set_fonts(fonts);
}

impl eframe::App for HarryWeb {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {});
        egui::Window::new("harry's site")
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, -100.0))
            .pivot(Align2::CENTER_CENTER)
            .movable(false)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.heading(format!("welcome to my little corner of the web"));
            });

        egui::Window::new("discord")
            .anchor(Align2::CENTER_CENTER, [-200.0, -250.0])
            .pivot(Align2::CENTER_CENTER)
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("../assets/discord-mark-white.png"))
                            .max_width(100.0),
                    );

                    ui.label("._hry (98130771507695616)")
                })
            });

        egui::Window::new("email")
            .anchor(Align2::CENTER_CENTER, [200.0, -250.0])
            .pivot(Align2::CENTER_CENTER)
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("../assets/envelope-solid.svg"))
                            .max_width(100.0),
                    );

                    ui.hyperlink_to("me@harry.live", "mailto:me@harry.live")
                })
            });

        egui::Window::new("github")
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 100.0))
            .pivot(Align2::CENTER_CENTER)
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let github = ui.add(
                        egui::Image::new(egui::include_image!("../assets/github-mark-white.png"))
                            .fit_to_exact_size(Vec2 { x: 75.0, y: 75.0 })
                            .sense(egui::Sense {
                                click: true,
                                drag: false,
                                focusable: false,
                            }),
                    );

                    if github.hovered() {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }

                    if github.clicked() {
                        ctx.open_url(egui::OpenUrl {
                            url: "https://github.com/harryob".to_owned(),
                            new_tab: true,
                        })
                    }

                    egui::CollapsingHeader::new("my projects:")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label("cmss13 - sysadmin and maintainer");
                            ui.label("cm-api - backend api to interact with the cmss13 database");
                            ui.label("cmdb - frontend for cm-api");
                        })
                });
            });

        egui::Window::new("source")
            .anchor(Align2::RIGHT_BOTTOM, [-50.0, -50.0])
            .title_bar(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let github = ui.add(
                        egui::Image::new(egui::include_image!("../assets/pencil-solid.svg"))
                            .max_width(25.0)
                            .sense(egui::Sense {
                                click: true,
                                drag: false,
                                focusable: false,
                            }),
                    );

                    let label: egui::Response = ui.label("built with open source!");

                    if github.hovered() || label.hovered() {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }

                    if github.clicked() || label.clicked() {
                        ctx.open_url(egui::OpenUrl {
                            url: "https://github.com/harryob".to_owned(),
                            new_tab: true,
                        })
                    }
                })
            });
    }
}
