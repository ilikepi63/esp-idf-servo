use esp_idf_sys::{
    esp, gpio_num_t, ledc_channel_config, ledc_channel_config_t, ledc_channel_t,
    ledc_clk_cfg_t_LEDC_AUTO_CLK, ledc_get_duty, ledc_intr_type_t_LEDC_INTR_DISABLE, ledc_mode_t,
    ledc_set_duty, ledc_stop, ledc_timer_bit_t, ledc_timer_bit_t_LEDC_TIMER_10_BIT,
    ledc_timer_config, ledc_timer_config_t, ledc_timer_config_t__bindgen_ty_1, ledc_timer_rst,
    ledc_timer_t, ledc_update_duty, EspError,
};

static SERVO_LEDC_INIT_BITS: ledc_timer_bit_t = ledc_timer_bit_t_LEDC_TIMER_10_BIT;

pub struct ServoConfig {
    pub max_angle: u16,
    pub min_width_us: u16,
    pub max_width_us: u16,
    pub frequency: u32,
    pub timer_number: ledc_timer_t,
    pub pin: gpio_num_t,
    pub channel: ledc_channel_t,
    pub speed_mode: ledc_mode_t,
}

fn calculate_duty(config: &ServoConfig, full_duty: u32, angle: f64) -> u32 {
    let angle_us = angle / (config.max_angle as f64)
        * ((config.max_width_us - config.min_width_us) as f64)
        + (config.min_width_us as f64);

    let duty: u32 = ((full_duty as f64) * angle_us * (config.frequency as f64) / 1000000.0) as u32;

    duty
}

fn calculate_angle(config: &ServoConfig, full_duty: u32, duty: u32) -> f64 {
    let mut angle_us = (duty as f64) * 1000000.0 / (full_duty as f64) / config.frequency;

    angle_us -= config.min_width_us;

    if angle_us < 0.0 {
        angule_us = 0.0;
    }

    let angle = angle_us * config.max_angle / (config.max_width_us - config.min_width_us);

    angle
}

pub struct Servo {
    full_duty: u32,
    config: ServoConfig,
}

impl Servo {
    pub fn init(config: ServoConfig) -> Result<Self, EspError> {
        let ledc_timer_cfg = ledc_timer_config_t {
            clk_cfg: ledc_clk_cfg_t_LEDC_AUTO_CLK,
            __bindgen_anon_1: ledc_timer_config_t__bindgen_ty_1 {
                duty_resolution: SERVO_LEDC_INIT_BITS,
            },
            freq_hz: config.frequency,
            speed_mode: config.speed_mode,
            timer_num: config.timer_number,
        };

        esp!(unsafe { ledc_timer_config(&ledc_timer_cfg) })?;

        let ledc_ch = ledc_channel_config_t {
            intr_type: ledc_intr_type_t_LEDC_INTR_DISABLE,
            channel: config.channel,
            duty: calculate_duty(&config, 0, 0.0),
            gpio_num: config.pin,
            speed_mode: config.speed_mode,
            timer_sel: config.timer_number,
            hpoint: 0,
            ..Default::default()
        };

        let _ = esp!(unsafe { ledc_channel_config(&ledc_ch) });

        let full_duty = (1 << SERVO_LEDC_INIT_BITS) - 1;

        Ok(Servo { full_duty, config })
    }

    pub fn write_angle(&self, angle: f64) -> Result<(), EspError> {
        let duty = calculate_duty(&self.config, self.full_duty, angle);

        esp!(unsafe { ledc_set_duty(self.config.speed_mode, self.config.channel.into(), duty) })?;

        esp!(unsafe { ledc_update_duty(self.config.speed_mode, self.config.channel.into()) })?;

        Ok(())
    }

    pub fn read_angle(&self) -> Result<f64, EspError> {
        let duty: u32 =
            esp!(unsafe { ledc_get_duty(self.config.speed_mode, self.config.channel) })?;

        Ok(calculate_angle(&self.config, self.full_duty, duty))
    }
}

impl Drop for Servo {
    fn drop(&mut self) {
        esp!(unsafe { ledc_stop(self.config.speed_mode, self.config.channel, 0) }).unwrap();
        esp!(unsafe { ledc_timer_rst(self.config.speed_mode, self.config.timer_number) }).unwrap();
    }
}
