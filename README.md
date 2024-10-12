# ARM Exceptions: Gotta Catch 'Em All!
Self imposed challenge to cause and handle as many nested exceptions as possible on a Cortex-M4.

A stack-backtrace will be our Pokédex, as it will contain all catched exceptions.

## Progress
The following exceptions have been catched.
- [x] Reset
- [x] NMI
- [x] HardFault
- [x] MemManage
- [x] BusFault
- [x] UsageFault
- [x] SVCall
- [ ] DebugMonitor
- [x] PendSV
- [x] SysTick

## Used resouces
- [Cortex-M4 Technical Reference Manual](https://documentation-service.arm.com/static/5f19da2a20b7cf4bc524d99a)
- [Armv7-M Architecture Reference Manual](https://documentation-service.arm.com/static/5f8fef3af86e16515cdbf816)
- [cortex_m docs.rs](https://docs.rs/cortex-m/latest/cortex_m/index.html)
- [cortex_m github](https://github.com/rust-embedded/cortex-m)

## Development Board
The used development board is a [STM32L452 Nucleo-64](https://www.st.com/en/evaluation-tools/nucleo-l452re.html). But only the features provided by the ARM Cortex-M4 core are used.

## Videos
Development of this code has been recorded.
- Video 1: [SysTick, PendSV and SVCall](https://www.youtube.com/watch?v=3kEA4-QZVao)
- Video 2: [UsageFault, BusFault, MemManage and HardFault](https://www.youtube.com/watch?v=qNcE33Vj4cw)
- Video 3: [NMI and Reset](https://www.youtube.com/watch?v=tczaBtahpHc)
