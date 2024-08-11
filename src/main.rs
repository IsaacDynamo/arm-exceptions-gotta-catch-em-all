#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use cortex_m_rt::{entry, exception};
use rtt_target::debug_rprintln as println;
use panic_rtt_target as _;
use cortex_m::{self as _, peripheral::scb::SystemHandler};

static DONE: AtomicBool = AtomicBool::new(false);


#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    println!("Hello, world!");



    unsafe {

        cortex_m::interrupt::enable();

        let mut core = cortex_m::Peripherals::take().unwrap();



        core.SCB.set_priority(SystemHandler::SysTick, 255);
        core.SCB.set_priority(SystemHandler::PendSV, 255 - 2);
        core.SCB.set_priority(SystemHandler::SVCall, 255 - 128);
        core.SCB.set_priority(SystemHandler::UsageFault, 0);


        core.SYST.csr.write(0b11);
        core.SYST.rvr.write(0x1000);
    }


    while ! DONE.load(Ordering::SeqCst) {

    }

    println!("End!");
    loop { }
}

#[exception]
unsafe fn SysTick() {
    println!("Hello from SysTick!");
    // Disable SysTick
    let syst = cortex_m::peripheral::SYST::PTR;
    (*syst).csr.write(0);

    // Set PendSV flag
    let scb = cortex_m::peripheral::SCB::PTR;
    (*scb).icsr.write(1 << 28);

    println!("Done from SysTick!");
}

#[exception]
unsafe fn PendSV() {
    println!("Hello from PendSV!");
    // core::arch::asm!("bkpt");
    core::arch::asm!("SVC #1");
    println!("Done from PendSV!");
}

#[exception]
unsafe fn SVCall() {
    println!("Hello from SVCall!");

    core::arch::asm!("udf.w #0");

    println!("Done from SVCall!");
}

#[exception]
unsafe fn UsageFault() {
    println!("Hello from UsageFault!");
    DONE.store(true, Ordering::SeqCst);
    println!("Done from UsageFault!");
}

#[exception]
unsafe fn NonMaskableInt() {
    println!("Hello from NMI!");
    DONE.store(true, Ordering::SeqCst);
    println!("Done from NMI!");
}

// #[exception]
// unsafe fn DebugMonitor() {
//     println!("Hello from PendSV!");
//     core::arch::asm!("SVC #1");
//     println!("Done from PendSV!");
// }

#[exception]
unsafe fn DefaultHandler(nr: i16) {
    println!("Hello from DefaultHandler! {}", nr);
}


// Exception number Exception
// 1 Reset
// 2 NMI
// 3 HardFault
// 4 MemManage
// 5 BusFault
// 6 UsageFault
// 7-10 Reserved
// 11 SVCall
// 12 DebugMonitor
// 13 Reserved
// 14 PendSV
// 15 SysTick
