#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct PointVariableD {
    pub components: Vec<f32>,
}
