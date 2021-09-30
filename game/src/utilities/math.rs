use std::ops::Range;

use num_traits::{real::Real, Float};
use raylib::math::Vector2;

/// Rotate a vector by an angle
pub fn rotate_vector(vector: Vector2, angle_rad: f32) -> Vector2 {
    return Vector2 {
        x: (vector.x * angle_rad.cos()) - (vector.y * angle_rad.sin()),
        y: (vector.y * angle_rad.cos()) + (vector.x * angle_rad.sin()),
    };
}

/// Interpolate a value from an input range to an output range while being modified by an exponential curve. **Input value is not checked**
pub fn interpolate_exp_unchecked<T>(
    value: T,
    input_range: Range<T>,
    output_range: Range<T>,
    exp: T,
) -> T
where
    T: Float,
{
    // Normalize the value as a percentage of the input range
    let normalized_value = (value - input_range.start) / (input_range.end - input_range.start);

    // Map the value along an exponential curve as defined by the exponent
    let mapped_value = ((normalized_value - T::one()).powf(exp) * -T::one()) + T::one();

    // Return the value mapped to the output range
    (mapped_value * (output_range.end - output_range.start)) + output_range.start
}

/// Interpolate a value from an input range to an output range while being modified by an exponential curve. **Input value is clamped**
pub fn interpolate_exp<T>(value: T, input_range: Range<T>, output_range: Range<T>, exp: T) -> T
where
    T: Float,
{
    // Clamp the value to the input range
    let clamped_value = value.max(input_range.start).min(input_range.end);

    // Interpolate the value
    interpolate_exp_unchecked(clamped_value, input_range, output_range, exp)
}
