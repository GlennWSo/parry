use crate::math::Real;
use crate::transformation::voxelization::FillMode;
use na::Point3;

pub struct VHACDParameters {
    pub concavity: Real,
    pub alpha: Real,
    pub beta: Real,
    pub resolution: u32,
    pub plane_downsampling: u32,
    pub convex_hull_downsampling: u32,
    pub fill_mode: FillMode,
    pub convex_hull_approximation: bool,
    pub max_convex_hulls: u32,
}

impl Default for VHACDParameters {
    fn default() -> Self {
        Self {
            resolution: 64,
            concavity: 0.001,
            plane_downsampling: 4,
            convex_hull_downsampling: 4,
            alpha: 0.05,
            beta: 0.05,
            convex_hull_approximation: true,
            max_convex_hulls: 1024,
            fill_mode: FillMode::FloodFill,
        }
    }
}
