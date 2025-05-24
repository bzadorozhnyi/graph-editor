use std::time::{Duration, Instant};

use eframe::egui::{self, Color32, Shadow, Stroke, Ui};

pub struct Toast {
    pub message: String,
    created_at: Instant,
    duration: Duration,
    pub variant: ToastVariant,
}

pub enum ToastVariant {
    Error,
    Success,
}

impl Toast {
    pub fn new<S: Into<String>>(message: S, r#type: ToastVariant) -> Self {
        Self {
            message: message.into(),
            created_at: Instant::now(),
            duration: Duration::from_secs(3),
            variant: r#type,
        }
    }

    pub fn error<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            created_at: Instant::now(),
            duration: Duration::from_secs(3),
            variant: ToastVariant::Error,
        }
    }

    pub fn success<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            created_at: Instant::now(),
            duration: Duration::from_secs(3),
            variant: ToastVariant::Success,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.duration
    }

    pub fn show(&self, ui: &mut Ui) {
        egui::Area::new("toast_area".into())
            .anchor(egui::Align2::CENTER_TOP, [0.0, 40.0])
            .show(ui.ctx(), |ui| {
                ui.visuals_mut().override_text_color = Some(Color32::WHITE);

                let fill_color = match self.variant {
                    ToastVariant::Error => Color32::DARK_RED,
                    ToastVariant::Success => Color32::DARK_GREEN,
                };

                egui::Frame::default()
                    .fill(fill_color)
                    .inner_margin(5.0)
                    .corner_radius(2.0)
                    .stroke(Stroke::new(2.0, Color32::BLACK))
                    .shadow(Shadow { offset: [1, 2], blur: 2, spread: 0, color: Color32::BLACK })
                    .show(ui, |ui| {
                        ui.set_width(300.0);

                        ui.label(&self.message)
                    })
            });
    }
}
