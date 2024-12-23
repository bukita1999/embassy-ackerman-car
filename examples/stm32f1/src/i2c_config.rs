use embassy_stm32::{i2c, peripherals, bind_interrupts};
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_stm32::mode::Async;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

pub fn setup_i2c(
    i2c: peripherals::I2C1,
    scl: peripherals::PB6,
    sda: peripherals::PB7,
    tx_dma: peripherals::DMA1_CH6,
    rx_dma: peripherals::DMA1_CH7,
) -> I2c<'static, Async> {  // 只需要指定生命周期和模式
    I2c::new(
        i2c,
        scl,
        sda,
        Irqs,
        tx_dma,
        rx_dma,
        Hertz(100_000),
        Default::default(),
    )
}