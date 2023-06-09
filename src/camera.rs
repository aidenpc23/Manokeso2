const DEFAULT_ASPECT_RATIO: f32 = 16. / 9.;

pub struct Camera {
    pub pos: [f32; 2],
    pub aspect: f32,
    pub scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: [0., 0.],
            aspect: DEFAULT_ASPECT_RATIO,
            scale: 1.0,
        }
    }
}
