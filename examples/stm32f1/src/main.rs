// src/main.rs
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

mod pwm;
mod i2c_config;

use crate::pwm::PwmController;
use crate::i2c_config::setup_i2c;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Starting PWM test");
    
    let p = embassy_stm32::init(Default::default());

    // 初始化I2C - 不使用DMA
    let i2c = setup_i2c(
        p.I2C1, 
        p.PB6, 
        p.PB7,
        p.DMA1_CH6,
        p.DMA1_CH7
    );

    // 创建PWM控制器实例
    let mut pwm_ctrl = match PwmController::new(i2c) {
        Ok(ctrl) => ctrl,
        Err(_) => {
            error!("Failed to create PWM controller");
            return;
        }
    };
    
    // 初始化PWM控制器
    match pwm_ctrl.init().await {
        Ok(()) => info!("PWM controller initialized successfully"),
        Err(_) => error!("Failed to initialize PWM controller: I2C error"),
    }

    // 测试循环
    loop {
        info!("Setting PWM output...");
        
        // 测试通道0的不同占空比
        if let Err(_e) = pwm_ctrl.set_pwm_duty(0, 25.0).await {
            error!("Failed to set PWM duty 1");
        }
        embassy_time::Timer::after_millis(2000).await;
        
        if let Err(_e) = pwm_ctrl.set_pwm_duty(0, 75.0).await {
            error!("Failed to set PWM duty 2");
        }
        embassy_time::Timer::after_millis(2000).await;
    }
}