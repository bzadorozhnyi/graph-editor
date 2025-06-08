use eframe::{egui::Pos2, emath::Rot2};

/// Rotate a point on the border of a circle around its center by a given angle (alpha, in radians).
/// The resulting point is snapped back to the circle's perimeter.
pub fn rotate_cirlce_border_point(border_pos: Pos2, center_pos: Pos2, alpha: f32) -> Pos2 {
    let rotation = Rot2::from_angle(alpha);
    let translated = border_pos - center_pos;
    let rotated = rotation * translated;

    center_pos + rotated
}
