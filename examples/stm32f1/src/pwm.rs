// src/pwm.rs
use pwm_pca9685::{Address, Channel, Pca9685, Error};

pub struct PwmController<I2C> {
    pwm: Pca9685<I2C>,
}

use embedded_hal::i2c::I2c as I2cTrait;         // 添加同步 I2C trait
use embedded_hal_async::i2c::I2c as AsyncI2c;   // 添加异步 I2C trait

impl<I2C> PwmController<I2C>
where
    I2C: I2cTrait + AsyncI2c,
{
    pub fn new(i2c: I2C) -> Result<Self, Error<I2C::Error>> {
        let address = Address::default();
        Ok(Self {
            pwm: Pca9685::new(i2c, address)?,
        })
    }

    /// 初始化PWM控制器
    pub async fn init(&mut self) -> Result<(), Error<I2C::Error>> {
        // 启用设备
        self.pwm.enable().await?;
        // 设置PWM频率（这里设置为50Hz作为示例）
        self.pwm.set_prescale(100).await?;
        Ok(())
    }

    /// 设置指定通道的PWM占空比（0.0 - 100.0）
    pub async fn set_pwm_duty(&mut self, channel: u8, duty: f32) -> Result<(), Error<I2C::Error>> {
        let duty = duty.clamp(0.0, 100.0);
        let value = (duty * 40.95) as u16; // 4095 * (duty/100)
        
        let channel = match channel {
            0 => Channel::C0,
            1 => Channel::C1,
            2 => Channel::C2,
            3 => Channel::C3,
            4 => Channel::C4,
            5 => Channel::C5,
            6 => Channel::C6,
            7 => Channel::C7,
            8 => Channel::C8,
            9 => Channel::C9,
            10 => Channel::C10,
            11 => Channel::C11,
            12 => Channel::C12,
            13 => Channel::C13,
            14 => Channel::C14,
            15 => Channel::C15,
            _ => return Ok(()),
        };

        // 设置PWM占空比
        self.pwm.set_channel_on_off(channel, 0, value).await?;
        Ok(())
    }
}