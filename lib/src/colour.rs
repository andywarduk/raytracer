use crate::vec3::Vec3;

pub type Colour = Vec3;

impl Colour {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        // Translate the [0,1] component values to the byte range [0,255].
        let rbyte = (256.0 * self.x()).clamp(0.0, 255.0) as u8;
        let gbyte = (256.0 * self.y()).clamp(0.0, 255.0) as u8;
        let bbyte = (256.0 * self.z()).clamp(0.0, 255.0) as u8;

        (rbyte, gbyte, bbyte)
    }

    pub fn to_rgb_gamma(&self) -> (u8, u8, u8) {
        // Translate the [0,1] component values to the byte range [0,255].
        let rbyte = (256.0 * Self::linear_to_gamma(self.x())).clamp(0.0, 255.0) as u8;
        let gbyte = (256.0 * Self::linear_to_gamma(self.y())).clamp(0.0, 255.0) as u8;
        let bbyte = (256.0 * Self::linear_to_gamma(self.z())).clamp(0.0, 255.0) as u8;

        (rbyte, gbyte, bbyte)
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }
}
