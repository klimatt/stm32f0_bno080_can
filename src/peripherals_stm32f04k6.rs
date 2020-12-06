
use crate::pin_cfg::{SDA_PIN, SCL_PIN, USR_LED_PIN};
use crate::can;
use stm32f0xx_hal::{
    prelude::*,
    i2c::I2c,
    stm32
};
use cortex_m::interrupt::{free as disable_interrupts, CriticalSection};

pub fn setup_peripherals(mut ctx_dev: stm32::Peripherals) -> (can::Can, I2c<stm32::I2C1, SCL_PIN, SDA_PIN>, USR_LED_PIN){

    let cs = unsafe {CriticalSection::new()};
    let mut rcc = ctx_dev.RCC;

    rcc.apb1enr.modify(|_, w| w.canen().enabled()); // can time enb

    let mut clock = rcc
        .configure()
        .sysclk( 48.mhz())
        .freeze(&mut ctx_dev.FLASH);

    let gpioa = ctx_dev.GPIOA.split(&mut clock);
    let gpiob = ctx_dev.GPIOB.split(&mut clock);

    let usr_led = gpiob.pb3.into_push_pull_output(&cs);

    let can_rx = gpioa.pa11.into_alternate_af4(&cs);
    let can_tx = gpioa.pa12.into_alternate_af4(&cs);

    let can_params: can::CanParams = can::CanParams{
        work_mode: can::CanMode::NormalMode,
        automatic_retransmission: can::AutomaticRetransmission::Enabled,
        automatic_busoff_management: can::AutomaticBussOffManagement::Enabled,
        auto_wake_up: can::AutomaticWakeUpMode::Enabled,
        pclk_Hz: clock.clocks.pclk(),

        bitrate: can::BitRate::_1Mbs
    };

    let can  = can::Can::new(
        can_tx,
        can_rx,
        ctx_dev.CAN,
        can_params
    );

    let sda = gpiob.pb7.into_alternate_af1(&cs);
    let scl = gpiob.pb6.into_alternate_af1(&cs);

    ctx_dev.I2C1.cr1.modify(|_,w|w.rxie().enabled());
    ctx_dev.I2C1.cr1.modify(|_,w|w.txie().enabled());
    ctx_dev.I2C1.cr1.modify(|_,w|w.tcie().enabled());

    let mut i2c = I2c::i2c1(
        ctx_dev.I2C1,
        (scl, sda),
        400.khz(),
        &mut clock
    );



    (can, i2c, usr_led)

}