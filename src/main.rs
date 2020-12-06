#![no_std]
#![no_main]

mod pin_cfg;
mod can;
mod peripherals_stm32f04k6;

use peripherals_stm32f04k6 as dp;

use rtic::app;
use cortex_m::asm::delay;
use rtt_target::{rprintln, rtt_init_print};
use stm32f0xx_hal::{
    prelude::*,
    stm32,
    i2c::I2c
};

#[app(device = stm32f0xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        can: can::Can,
        i2c: I2c<stm32::I2C1, pin_cfg::SCL_PIN, pin_cfg::SDA_PIN>,
        usr_led: pin_cfg::USR_LED_PIN,
    }
    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        rtt_init_print!();
        let (can, i2c, usr_led) = dp::setup_peripherals(ctx.device);

        init::LateResources {
            can,
            i2c,
            usr_led,
        }
    }
    #[idle(resources = [can, usr_led, i2c])]
    fn idle(ctx: idle::Context) -> ! {
        let mut can = ctx.resources.can;
        let mut usr_led = ctx.resources.usr_led;
        let mut i2c = ctx.resources.i2c;

        loop {
            delay(6_000_000);
            let mut _devices = 0;

            /*i2c.lock(|i2c|{

                i2c.write(0x28, &[0x07, 0x00]);
                i2c.write_read(0x28, &[0x00], &mut buff);

                delay(1_000_000);
                rprintln!("buff: {:x}", buff[0]);
            });*/
            delay(6_000_000);
            /*can.lock(|can|{
                can.write_to_mailbox(can::IdType::Extended, 0x11112222, &[1,2,3,4,5]);
            });*/
            usr_led.toggle();
        }

    }

    /*#[task(binds = I2C1, priority = 3 , resources = [i2c])]
    fn i2c1_irq(ctx: i2c1_irq::Context){
        let mut i2c = ctx.resources.i2c;
        rprintln!("I2C_IRQ: ");
    }*/

    #[task(binds = CEC_CAN, priority = 2 , resources = [can, i2c])]
    fn can_irq(ctx: can_irq::Context){
        let can: &mut can::Can = ctx.resources.can;
        let mut i2c = ctx.resources.i2c;
        let mut buff :[u8; 8] = [0; 8];
        can.irq_state_machine(|id, data|{
            rprintln!("CAN_IRQ_RX: id: {:x}; Data: {:?}", id, data);
            i2c.write(0x28, &[0x07, data[0]]);
            i2c.write_read(0x28, &[data[1]], &mut buff);
        });
        if can.receive_flag {
            can.write_to_mailbox(can::IdType::Extended, 0x00000001, &buff);
        }
    }

};


use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use core::borrow::BorrowMut;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("Panic: {:?}", info);
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}