use raylib::math::Vector2;

/// Rotate a vector by an angle
pub fn rotate_vector(vector: Vector2, angle_rad: f32) -> Vector2 {
    return Vector2 {
        x: (vector.x * angle_rad.cos()) - (vector.y * angle_rad.sin()),
        y: (vector.y * angle_rad.cos()) + (vector.x * angle_rad.sin()),
    };
}
