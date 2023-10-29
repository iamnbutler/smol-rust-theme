use crate::utils::hsla::Hsla;

/// A ColorScale represents the range of values a color can have.
/// It is an array with exactly 12 Hsla values.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorScale([Hsla; 12]);

impl ColorScale {
    /// Creates a new ColorScale with the provided Hsla values.
    /// The input slice must contain exactly 12 elements.
    pub fn new(values: &[Hsla; 12]) -> Self {
        ColorScale(*values)
    }

    /// Returns a reference to the Hsla value at the specified index.
    /// Returns None if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&Hsla> {
        self.0.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::hsla::hsla;

    #[test]
    fn test_color_scale() {
        let values = [
            hsla(0.0, 0.0, 0.0, 0.0),
            hsla(0.1, 0.1, 0.1, 0.1),
            hsla(0.2, 0.2, 0.2, 0.2),
            hsla(0.3, 0.3, 0.3, 0.3),
            hsla(0.4, 0.4, 0.4, 0.4),
            hsla(0.5, 0.5, 0.5, 0.5),
            hsla(0.6, 0.6, 0.6, 0.6),
            hsla(0.7, 0.7, 0.7, 0.7),
            hsla(0.8, 0.8, 0.8, 0.8),
            hsla(0.9, 0.9, 0.9, 0.9),
            hsla(1.0, 1.0, 1.0, 1.0),
            hsla(1.0, 1.0, 1.0, 1.0),
        ];
        let scale = ColorScale::new(&values);
        assert_eq!(scale.get(0), Some(&hsla(0.0, 0.0, 0.0, 0.0)));
        assert_eq!(scale.get(11), Some(&hsla(1.0, 1.0, 1.0, 1.0)));
        assert_eq!(scale.get(12), None);
    }
}
