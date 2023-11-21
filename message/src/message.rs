use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct U8Data {
    pub offset: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PixelData {
    pub offset: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PixelIntensity {
    pub zn: f32,
    pub count: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IteratedSinZ {
    pub c: Complex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mandelbrot {}

#[derive(Debug, Serialize, Deserialize)]
pub enum FractalDescriptor {
    IteratedSinZ(IteratedSinZ),
    JuliaDescriptor(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentTask {
    pub id: U8Data,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub pixel: PixelData,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    FragmentTask(FragmentTask),
    FragmentResult(FragmentResult),
    FragmentRequest(FragmentRequest),
}

