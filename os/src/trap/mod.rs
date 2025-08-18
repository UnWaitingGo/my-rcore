//! 陷阱处理功能
//!
//! 在 rCore 中，我们有一个唯一的陷阱入口点，即 __alltraps。 
//! 在 [init()] 初始化时，我们将 stvec 控制状态寄存器（CSR）设置为指向该入口点。
//!
//! 所有陷阱都通过 __alltraps 处理，该入口点在 trap.S 中定义。
//! 汇编语言代码仅完成恢复内核空间上下文的必要工作，
//! 确保 Rust 代码能安全运行， 并将控制权转移到 [trap_handler()]。
//! 
//!
//! 随后，它会根据具体的异常类型调用不同的功能。
//! 例如，定时器中断会触发任务抢占，
//! 系统调用则会进入 [syscall()] 处理。

mod context;

use crate::batch::run_next_app;
use crate::syscall::syscall;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

global_asm!(include_str!("trap.S"));

///  将 CSR 寄存器 stvec 初始化为 __alltraps 的入口地址。
pub fn init() {
    unsafe extern "C" {
        safe fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[unsafe(no_mangle)] // Rust 2024 Edition新写法 函数或变量保持原有的名称，确保其他语言（如 C、汇编）或外部程序能通过原始名称正确识别和调用它们。
/// 从用户空间处理中断、异常或系统调用
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // 获取陷阱（trap）的原因
    let stval = stval::read(); // 获取额外值。
    
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}   

pub use context::TrapContext;
