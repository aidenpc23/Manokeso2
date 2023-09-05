use crate::util::point::Point;

const DEFAULT_ASPECT_RATIO: f32 = 16. / 9.;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Camera {
    pub pos: Point<f32>,
    pub aspect: f32,
    pub scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Point::zero(),
            aspect: DEFAULT_ASPECT_RATIO,
            scale: 2.0,
        }
    }
}
