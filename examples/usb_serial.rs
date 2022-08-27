//! CDC-ACM serial port example using polling in a busy loop.
//! Target board: any STM32F4 with a OTG FS peripheral and a 25MHz HSE crystal
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::otg_fs::{ UsbBus, USB };
use stm32f4xx_hal::{ pac, prelude::*, interrupt };
use usb_device::prelude::*;
use cortex_m::{ interrupt::Mutex };
use usb_device::{ prelude::*, class_prelude::UsbBusAllocator };
use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::cell::Cell;

static G_USB_BUS: Mutex<RefCell<Option<UsbBusAllocator<UsbBus<USB>>>>> = Mutex::new(
    RefCell::new(None)
);

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[interrupt]
fn OTG_FS() {
    /*
    if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        */
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk((168).MHz()).pclk1((8).MHz()).freeze();

    let gpioa = dp.GPIOA.split();

    let usb = USB {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into_alternate(),
        pin_dp: gpioa.pa12.into_alternate(),
        hclk: clocks.hclk(),
    };

    cortex_m::interrupt::free(|cs| {
        let mut usb_bus = G_USB_BUS.borrow(cs)
            .borrow_mut()
            .get_or_insert(UsbBus::new(usb, unsafe { &mut EP_MEMORY }));

        let mut usb_bus2 = G_USB_BUS.borrow(cs).borrow_mut().as_ref().unwrap();

        //let mut usb_bus2 = G_USB_BUS.borrow(cs).borrow_mut().unwrap();

        //let mut usb_bus = G_USB_BUS.borrow(cs).borrow_mut().;

        let mut serial = usbd_serial::SerialPort::new(&usb_bus2);
        /*
        let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")
            .device_class(usbd_serial::USB_CLASS_CDC)
            .build();
 */
        //let mut global_usb_bus = G_USB_BUS.borrow(cs).borrow_mut();

        //global_usb_bus.insert(usb_bus);
    });

    loop {
    }
}