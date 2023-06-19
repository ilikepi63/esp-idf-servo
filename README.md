# ESP IDF Servo Library

## Introduction

This crate is a simple wrapper over the `esp-idf-sys` bindings to make servo control easier using the LEDC module. 

## Basic Usage

Create a basic Servo Configuration - this one specific for the SG90 series of servo motors. 

```
let peripherals = Peripherals::take().unwrap();

let servo_cfg = ServoConfig {
    // below are servo specific configurations
    max_angle: 180,
    min_width_us: 500,
    max_width_us: 2500,
    frequency: 50,
    
    // choose your timer: see https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/?search=ledc_timer_t_LEDC_TIMER_0
    timer_number: 0,

    // whichever pin the servo is attached to
    pin: 5,

    // the channel number: see https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/type.ledc_channel_t.html
    channel: 0,

    // the speed mode: see https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/constant.ledc_mode_t_LEDC_LOW_SPEED_MODE.html
    speed_mode: 0,
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

        println!("Angle: {}", servo.read_angle());
    }

    for i in (0..180).rev() {
            
        servo.write_angle((i as f64));

        FreeRtos::delay_ms(20);
        
        println!("Angle: {}", servo.read_angle());
    }
}
```