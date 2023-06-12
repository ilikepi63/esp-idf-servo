use esp_idf_sys::{
    esp,
    gpio_num_t,
    ledc_channel_config,
    ledc_channel_config_t,
    ledc_channel_t,
    ledc_clk_cfg_t_LEDC_AUTO_CLK,
    ledc_intr_type_t_LEDC_INTR_DISABLE,
    ledc_mode_t,
    ledc_set_duty,
    ledc_stop,
    ledc_timer_bit_t,
    ledc_timer_config,
    ledc_timer_config_t,
    ledc_timer_config_t__bindgen_ty_1,
    ledc_timer_rst,
    ledc_timer_t,
    ledc_update_duty,
    ledc_timer_bit_t_LEDC_TIMER_10_BIT
};

static SERVO_LEDC_INIT_BITS: ledc_timer_bit_t = ledc_timer_bit_t_LEDC_TIMER_10_BIT;

pub struct ServoChannel {
    pub servo_pin: gpio_num_t,
    pub channel: ledc_channel_t,
}

pub struct ServoConfig {
    pub max_angle: u16,
    pub min_width_us: u16,
    pub max_width_us: u16,
    pub frequency: u32,
    pub timer_number: ledc_timer_t,
    pub channels: Vec<ServoChannel>,
}

fn calculate_duty(config: &ServoConfig, full_duty: u32, angle: f64) -> u32 {
    let angle_us = angle / (config.max_angle as f64) * ((config.max_width_us - config.min_width_us) as f64)
        + (config.min_width_us as f64);

    println!("Angle in us: {:?}", angle_us);

    let duty: u32 = ((full_duty as f64) * angle_us * (config.frequency as f64) / 1000000.0 ) as u32;

    println!("Setting duty: {:?}", duty);

    duty
}

pub struct Servo {
    pub full_duty: u32,
    pub config: ServoConfig,
    pub speed_mode: ledc_mode_t,
}

impl Servo {
    pub fn init(speed_mode: ledc_mode_t, config: ServoConfig) -> Self {

        let ledc_timer_cfg = ledc_timer_config_t {
            clk_cfg: ledc_clk_cfg_t_LEDC_AUTO_CLK,
            __bindgen_anon_1: ledc_timer_config_t__bindgen_ty_1 {
                duty_resolution: SERVO_LEDC_INIT_BITS,
            },
            freq_hz: config.frequency,
            speed_mode,
            timer_num: config.timer_number,
        };

        esp!(unsafe { ledc_timer_config(&ledc_timer_cfg) }).unwrap();

        for channel in &config.channels {
            let ledc_ch = ledc_channel_config_t {
                intr_type: ledc_intr_type_t_LEDC_INTR_DISABLE,
                channel: channel.channel,
                duty: calculate_duty(&config, 0, 0.0),
                gpio_num: channel.servo_pin,
                speed_mode,
                timer_sel: config.timer_number,
                hpoint: 0,
                ..Default::default()
            };

            let _ = esp!(unsafe {
                ledc_channel_config(&ledc_ch)
            });
        }

        let full_duty = (1 << SERVO_LEDC_INIT_BITS) - 1;

        Servo { full_duty, config, speed_mode }
    }

    pub fn write_angle(&self, channel: u8, angle: f64) {
        let duty = calculate_duty(&self.config, self.full_duty, angle);
        
        esp!(unsafe { ledc_set_duty(self.speed_mode, channel.into(), duty) }).unwrap();

        esp!(unsafe {
            ledc_update_duty(self.speed_mode, channel.into())
        }).unwrap();

    }
}

impl Drop for Servo {
    fn drop(&mut self) {
        for channel in &self.config.channels {
            esp!(unsafe {
                //TODO: hardcoded channel here
                ledc_stop(self.speed_mode, channel.channel, 0)
            }).unwrap();
        }
        esp!(unsafe {
            ledc_timer_rst(self.speed_mode, self.config.timer_number)
        }).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
