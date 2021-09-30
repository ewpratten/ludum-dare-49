use std::ops::Range;

use num_traits::Float;
use raylib::math::Vector2;

/// Rotate a vector by an angle
#[allow(dead_code)]
pub fn rotate_vector(vector: Vector2, angle_rad: f32) -> Vector2 {
    Vector2 {
        x: (vector.x * angle_rad.cos()) - (vector.y * angle_rad.sin()),
        y: (vector.y * angle_rad.cos()) + (vector.x * angle_rad.sin()),
    }
}

/// Interpolate a value from an input range to an output range while being modified by an exponential curve. **Input value is not checked**
#[allow(dead_code)]
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
    let mapped_value = (-T::one())
        .mul((normalized_value.mul(T::one().add(T::one())) - T::one()).powf(exp))
        .add(T::one());

    // Return the value mapped to the output range
    (mapped_value * (output_range.end - output_range.start)) + output_range.start
}

/// Interpolate a value from an input range to an output range while being modified by an exponential curve. **Input value is clamped**
#[allow(dead_code)]
pub fn interpolate_exp<T>(value: T, input_range: Range<T>, output_range: Range<T>, exp: T) -> T
where
    T: Float,
{
    // Clamp the value to the input range
    let clamped_value = value.max(input_range.start).min(input_range.end);

    // Interpolate the value
    interpolate_exp_unchecked(clamped_value, input_range, output_range, exp)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_vector() {
        let vector = Vector2 { x: 1.0, y: 0.0 };
        let angle_rad = 90.0.to_radians();
        let expected_vector = Vector2 { x: 0.0, y: 1.0 };
        let actual_vector = rotate_vector(vector, angle_rad);
        assert!(relative_eq!(
            actual_vector.x,
            expected_vector.x,
            epsilon = f32::EPSILON
        ));
        assert!(relative_eq!(
            actual_vector.y,
            expected_vector.y,
            epsilon = f32::EPSILON
        ));
    }

    #[test]
    fn test_interpolate_exp_head() {
        let input_range = 0.0..1.0;
        let output_range = 0.0..1.0;
        let exp = 8.0;
        let value = 0.043;
        let expected_value = 0.513;
        let actual_value = interpolate_exp(value, input_range, output_range, exp);
        assert!(relative_eq!(actual_value, expected_value, epsilon = 0.001));
    }

    #[test]
    fn test_interpolate_exp_tail() {
        let input_range = 0.0..1.0;
        let output_range = 0.0..1.0;
        let exp = 8.0;
        let value = 0.957;
        let expected_value = 0.513;
        let actual_value = interpolate_exp(value, input_range, output_range, exp);
        assert!(relative_eq!(actual_value, expected_value, epsilon = 0.001));
    }
}
