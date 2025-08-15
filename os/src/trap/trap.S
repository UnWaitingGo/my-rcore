.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
                             #这一行之前 sp 指向用户栈， sscratch 指向内核
    csrrw sp, sscratch, sp   #现在 sp 指向内核栈， sscratch 指向用户栈。
   
    addi sp, sp, -34*8   # 在内核栈上保存 Trap 上下文
    # 保存通用寄存器
    sd x1, 1*8(sp)
         # 跳过 sp(x2), 我们会在后面保存
    sd x3, 3*8(sp)
         # 跳过保存 tp(x4), 应用不会使用它
    # x5~x31 这 27 个通用寄存器我们通过类似循环的 .rept 每次使用 SAVE_GP 宏来保存
    .set n, 5
    .rept 27           #在 trap.S 开头加上 .altmacro 才能正常使用 .rept 命令
        SAVE_GP %n
        .set n, n+1
    .endr

    # 我们可以自由使用 t0/t1/t2 寄存器, 因为他们是在内核栈上保存
    csrr t0, sstatus   #我们将 CSR sstatus 和 sepc 的值分别读到寄存器 t0 和 t1 中然后保存到内核栈对应的位置上。
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)

    # 将用户栈上的 sscratch 的值读到寄存器 t2 并保存到内核栈上
    csrr t2, sscratch
    sd t2, 2*8(sp)

    # 设置 trap_handler 的输入参数(cx: &mut TrapContext)
    mv a0, sp  #让寄存器 a0 指向内核栈的栈指针也就是我们刚刚保存的 Trap 上下文的地址
    call trap_handler

__restore:
    # case1: start running app by __restore
    # case2: back to U after handling trap
    mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    addi sp, sp, 34*8
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp
    sret
