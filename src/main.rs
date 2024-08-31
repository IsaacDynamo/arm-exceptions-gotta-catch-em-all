#![no_std]
#![no_main]

use core::sync::atomic::{AtomicI8, Ordering};
use cortex_m::{self as _, peripheral::scb::SystemHandler};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use panic_rtt_target as _;
use rtt_target::debug_rprintln as println;

static ID: AtomicI8 = AtomicI8::new(0);

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    println!("Main");

    unsafe {
        cortex_m::interrupt::enable();

        // Setup exception priorities
        const STEP: u8 = 16;
        let mut core = cortex_m::Peripherals::take().unwrap();
        core.SCB.set_priority(SystemHandler::SysTick, 255);
        core.SCB.set_priority(SystemHandler::PendSV, 255 - STEP);
        core.SCB.set_priority(SystemHandler::SVCall, 255 - 2 * STEP);
        core.SCB
            .set_priority(SystemHandler::UsageFault, 255 - 3 * STEP);
        core.SCB
            .set_priority(SystemHandler::BusFault, 255 - 4 * STEP);
        core.SCB
            .set_priority(SystemHandler::MemoryManagement, 255 - 5 * STEP);

        // Enable UsageFault, BusFault and MemManage exceptions
        core.SCB
            .shcsr
            .modify(|v| v | (1 << 16) | (1 << 17) | (1 << 18));

        // Enable and start SysTick timer
        core.SYST.csr.write(0b11);
        core.SYST.rvr.write(0x1000);
    }

    loop {}
}

#[exception]
unsafe fn SysTick() {
    println!("SysTick");

    // Disable SysTick
    let syst = cortex_m::peripheral::SYST::PTR;
    (*syst).csr.write(0);

    // SVCall with ID 0
    ID.store(0, Ordering::SeqCst);
    core::arch::asm!("SVC #0");
}

#[exception]
unsafe fn PendSV() {
    println!("PendSV");

    // SVCall with ID 1
    ID.store(1, Ordering::SeqCst);
    core::arch::asm!("SVC #1");
}

#[exception]
unsafe fn SVCall() {
    let id = ID.load(Ordering::SeqCst);
    match id {
        0 => {
            println!("  SVCall {}", id);

            // Set PendSV flag
            let scb = cortex_m::peripheral::SCB::PTR;
            (*scb).icsr.write(1 << 28);
        }
        1 => {
            println!("SVCall {}", id);

            // Execute undefined instruction to cause a UsageFault
            core::arch::asm!("udf.w #0");
        }
        _ => unreachable!(),
    }
}

#[exception]
unsafe fn UsageFault() {
    println!("UsageFault");

    // Write to invalid memory to cause a BusFault
    let var_busfault = 0x1FFFFFFC as *mut u32;
    let _ = var_busfault.write_volatile(0);
}

#[exception]
unsafe fn BusFault() {
    println!("BusFault");

    // Call into Execute Never memory to cause a MemManage fault
    let func_xn: fn() = core::mem::transmute(0x40000000);
    func_xn();
}

#[exception]
unsafe fn MemoryManagement() {
    println!("MemoryManagement");

    // Call into Execute Never memory to cause a MemManage fault
    // This is upgraded to a Hardfault, because we are already in MemManage handler
    let func_xn: fn() = core::mem::transmute(0x40000000);
    func_xn();
}

#[exception]
unsafe fn HardFault(_: &ExceptionFrame) -> ! {
    println!("HardFault");
    core::arch::asm!("bkpt");
    loop {}
}

#[exception]
unsafe fn NonMaskableInt() {
    println!("NMI");
}

// #[exception]
// unsafe fn DebugMonitor() {
//     println!("Hello from PendSV!");
//     core::arch::asm!("SVC #1");
//     println!("Done from PendSV!");
// }

#[exception]
unsafe fn DefaultHandler(nr: i16) {
    println!("DefaultHandler {}", nr);
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
