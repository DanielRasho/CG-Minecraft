use nalgebra_glm::Vec3;
use super::color::Color;
use std::f32::consts::PI;

pub trait Light : Sync {
   fn get_position(&self) -> Vec3 ;
   fn get_color(&self) -> Color;
   fn get_intensity(&self) -> f32;
}

pub struct PointLight {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
}

impl PointLight {
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        PointLight {
            position,
            color,
            intensity,
        }
    }
}

const DAY_START: f32 = 0.0;
const DAY_MID: f32 = PI / 3.0;
const DAY_END: f32 = 2.0 * PI / 3.0;
const NIGHT_START: f32 = 3.0 * PI / 2.0;

pub struct DayLight {
    pub position: Vec3,
    pub center: Vec3,
    pub radius: f32,
    pub day_angle: f32,
    pub color: Color,
    pub intensity: f32,
}


pub struct AmbientLight{
    pub color: Color,
    pub intensity: f32,
}

impl AmbientLight {
    pub fn new(color: Color, intensity: f32) -> Self {
        AmbientLight {
            color,
            intensity,
        }
    }
}
impl Light for PointLight {
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_intensity(&self) -> f32 {
        self.intensity
    }
}

impl Light for AmbientLight {
    fn get_position(&self) -> Vec3 {
        // Ambient light does not have a position, so we can return a zero vector or a default.
        Vec3::new(0.0, 0.0, 0.0)
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_intensity(&self) -> f32 {
        self.intensity
    }
}

impl Light for DayLight {
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_intensity(&self) -> f32 {
        self.intensity
    }
}

impl DayLight {
    pub fn new(position: Vec3, center: Vec3, radius: f32, day_angle:f32, color: Color, intensity: f32) -> Self {
        DayLight {
            position,
            center,
            radius,
            day_angle,
            color,
            intensity,
        }
    }
    pub fn translate_day_light(&mut self, delta_angle: f32) {
        // Update day_angle within the range [0, 2*PI] to keep it bounded to one day cycle
        self.day_angle = (self.day_angle + delta_angle) % (2.0 * PI);

        // Calculate the new position on the circular path around `center`
        self.position.x = self.center.x + self.radius * self.day_angle.cos();
        self.position.y = self.center.y + self.radius * self.day_angle.sin();

        // Adjust intensity based on day_angle
        if self.day_angle >= DAY_START && self.day_angle <= DAY_END {
            let ratio = if self.day_angle < DAY_MID {
                self.day_angle / DAY_MID
            } else {
                1.0 - ((self.day_angle - DAY_MID) / (DAY_END - DAY_MID))
            };
            self.intensity = ratio; // Intensity varies from 0 to 1 between DAY_START and DAY_END

        } else if self.day_angle >= DAY_END && self.day_angle <= NIGHT_START {
            // Dimming toward night
            self.intensity = 0.0;
        }
    }
}