use stm32f0xx_hal::{
    gpio::gpioa::{PA5, PA6, PA7, PA11, PA12},
    gpio::gpiob::{PB3, PB4, PB5, PB7, PB6},
    gpio::{Alternate, AF4, AF1, Output, PushPull},
};

pub type SDA_PIN = PB7<Alternate<AF1>>;
pub type SCL_PIN = PB6<Alternate<AF1>>;

pub type CAN_TX_PIN = PA12<Alternate<AF4>>;
pub type CAN_RX_PIN = PA11<Alternate<AF4>>;

pub type USR_LED_PIN = PB3<Output<PushPull>>;