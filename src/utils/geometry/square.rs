use eframe::{
    egui::{Pos2, Rect, Vec2},
    emath::Rot2,
};

/// Returns the point where a ray from `center` in the given `direction`
/// intersects the edge of a square centered at `center` with half-side `half_size`.
///
/// The returned point lies on the perimeter of the square.
pub fn intersect_rect_edge(center: Pos2, half_size: f32, direction: Vec2) -> Pos2 {
    let rect = Rect::from_center_size(center, Vec2::splat(2.0 * half_size));
    let dir = direction.normalized();

    let tx = if dir.x != 0.0 {
        let dx = if dir.x > 0.0 {
            rect.max.x - center.x
        } else {
            rect.min.x - center.x
        };

        Some(dir.x / dx)
    } else {
        None
    };

    let ty = if dir.y != 0.0 {
        let dy = if dir.y > 0.0 {
            rect.max.y - center.y
        } else {
            rect.min.y - center.y
        };

        Some(dir.y / dy)
    } else {
        None
    };

    let t = match (tx, ty) {
        (Some(tx), Some(ty)) => tx.min(ty),
        (Some(tx), None) => tx,
        (None, Some(ty)) => ty,
        (None, None) => 0.0,
    };

    center + dir * t
}

/// Rotate a point on the border of a square around its center by a given angle (alpha, in radians).
/// The resulting point is snapped back to the square's perimeter.
pub fn rotate_square_border_point(
    border_pos: Pos2,
    center: Pos2,
    alpha: f32,
    half_size: f32,
) -> Pos2 {
    let rotation = Rot2::from_angle(alpha);
    let translated = border_pos - center;
    let rotated = rotation * translated;
    let rotated_pos = center + rotated;

    project_to_square_edge(rotated_pos, center, half_size)
}

fn project_to_square_edge(pos: Pos2, center: Pos2, half_size: f32) -> Pos2 {
    let dx = pos.x - center.x;
    let dy = pos.y - center.y;

    let abs_dx = dx.abs();
    let abs_dy = dy.abs();

    if abs_dx > abs_dy {
        Pos2::new(
            center.x + dx.signum() * half_size,
            center.y + dy * half_size / abs_dx,
        )
    } else {
        Pos2::new(
            center.x + dx * half_size / abs_dy,
            center.y + dy.signum() * half_size,
        )
    }
}
