use riscv::register::sstatus::{self, SPP, Sstatus};
/// Trap 上下文
#[repr(C)]
pub struct TrapContext {
    /// 通用寄存器 [0..31]
    pub x: [usize; 32],
    /// sstatus 控制状态寄存器 CSR       
    pub sstatus: Sstatus,
    /// sepc 控制状态寄存器 CSR 
    pub sepc: usize,
}

impl TrapContext {
    /// 设置栈指针 x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); // sstatus 控制状态寄存器 CSR
        sstatus.set_spp(SPP::User); //之前的特权模式：用户模式
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // 应用程序的入口点
        };
        cx.set_sp(sp); // 应用程序的用户栈指针
        cx // 返回应用程序的初始Trap Context
    }
}
