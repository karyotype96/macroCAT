use std::f32::consts::PI;

pub enum EasingType {
    Linear,
    InQuadratic, OutQuadratic, InOutQuadratic,
    InCubic, OutCubic, InOutCubic,
    InExpo, OutExpo, InOutExpo,
    InSine, OutSine, InOutSine,
    InBack, OutBack, InOutBack,
    InElastic, OutElastic, InOutElastic,
}

impl EasingType {
    pub fn ease_range(&self, val: f32, min1: f32, max1: f32, min2: f32, max2: f32) -> f32 {
        // should be a value between 0 and 1
        let x = ((val-min1) / (max1-min1)).clamp(0.0, 1.0);

        let ease_value: f32 = match *self {
            EasingType::Linear => {
                x
            }
            EasingType::InQuadratic => {
                x * x
            }
            EasingType::OutQuadratic => {
                1.0 - (1.0 - x) * (1.0 - x)
            }
            EasingType::InOutQuadratic => {
                if x < 0.5 {
                    2.0 * x * x
                } else {
                    1.0 - ((-2.0 * x + 2.0) * (-2.0 * x + 2.0)) / 2.0
                }
            }
            EasingType::InCubic => {
                x * x * x
            }
            EasingType::OutCubic => {
                1.0 - (1.0 - x)*(1.0 - x)*(1.0-x)
            }
            EasingType::InOutCubic => {
                if x < 0.5 {
                    4.0 * x * x
                } else {
                    1.0 - (-2.0 * x + 2.0)*(-2.0 * x + 2.0)*(-2.0 * x + 2.0) / 2.0
                }
            }
            EasingType::InExpo => {
                if x == 0.0 {
                    0.0
                } else {
                    2.0_f32.powf(10.0 * x - 10.0)
                }
            }
            EasingType::OutExpo => {
                if x == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f32.powf(-10.0 * x)
                }
            }
            EasingType::InOutExpo => {
                match x {
                    0.0 => 0.0,
                    1.0 => 1.0,
                    _ => {
                        if x < 0.5 {
                            2.0_f32.powf(20.0 * x - 10.0) / 2.0
                        } else {
                            (2.0 - 2.0_f32.powf(-20.0 * x + 10.0)) / 2.0
                        }
                    }
                }
            }
            EasingType::InSine => {
                1.0 - ((x * PI) / 2.0).cos()
            }
            EasingType::OutSine => {
                ((x * PI) / 2.0).sin()
            }
            EasingType::InOutSine => {
                -((x * PI).cos() - 1.0) / 2.0
            }
            EasingType::InBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;

                c3 * x * x * x - c1 * x * x
            }
            EasingType::OutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;

                return 1.0 + c3 * (x-1.0)*(x-1.0)*(x-1.0) + c1 * (x-1.0)*(x-1.0)
            }
            EasingType::InOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;

                if x < 0.5 {
                    ((2.0*x).powf(2.0) * ((c2 + 1.0) * 2.0 * x - c2)) / 2.0
                } else {
                    ((2.0*x-2.0).powf(2.0) * ((c2 + 1.0) * (x * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            EasingType::InElastic => {
                let c4 = (2.0 * PI) / 3.0;

                match x {
                    0.0 => 0.0,
                    1.0 => 1.0,
                    _ => {
                        -(2.0_f32.powf(10.0 * x - 10.0)) * ((x * 10.0 - 10.75) * c4).sin()
                    }
                }
            }
            EasingType::OutElastic => {
                let c4 = (2.0 * PI) / 3.0;

                match x {
                    0.0 => 0.0,
                    1.0 => 1.0,
                    _ => {
                        2.0_f32.powf(-10.0 * x) * ((x * 10.0 - 0.75) * c4).sin() + 1.0
                    }
                }
            }
            EasingType::InOutElastic => {
                let c5 = (2.0 * PI) / 3.0;

                match x {
                    0.0 => 0.0,
                    1.0 => 1.0,
                    _ => {
                        if x < 0.5 {
                            -(2.0_f32.powf(20.0 * x - 10.0) * ((20.0 * x - 11.125) * c5).sin())
                        } else {
                            2.0_f32.powf(-20.0 * x + 10.0) * ((20.0 * x - 11.125) * c5).sin()
                        }
                    }
                }
            }
        };

        min2 + ease_value * (max2 - min2)
    }
}