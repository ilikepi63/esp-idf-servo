# ESP IDF Servo Library

## Introduction

This crate is a simple wrapper over the `esp-idf-sys` bindings to make servo control easier using the LEDC module. 

## Basic Usage

Create a basic Servo Configuration - this one specific for the SG90 series of servo motors. 

```
    let peripherals = Peripherals::take().unwrap();

    let servo_cfg = ServoConfig {
        max_angle: 180,
        min_width_us: 500,
        max_width_us: 2500,
        frequency: 50,
        timer_number: ledc_timer_t_LEDC_TIMER_0,
        pin: peripherals.pins.gpio0.pin(),
        channel: esp_idf_hal::ledc::CHANNEL0::channel(),
        speed_mode: ledc_mode_t_LEDC_LOW_SPEED_MODE,
    };
```

Initialize the servo: 

```
    let servo = Servo::init(servo_cfg);
```

Change the angle of the servo within your program: 

```
    loop {
        for i in 0..180 {

            servo.write_angle((i as f64));

            FreeRtos::delay_ms(20);
        }

        for i in (0..180).rev() {
            
            servo.write_angle((i as f64));

            FreeRtos::delay_ms(20);
        }
    }
```