use core::arch::asm;

const SYSCALL_WRITE : usize = 64;
const SYSCALL_EXIT : usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}
//sys_write 使用一个 &[u8] 切片类型来描述缓冲区，
//这是一个 胖指针 (Fat Pointer)，里面既包含缓冲区的起始地址，还包含缓冲区的长度。
//我们可以分别通过 as_ptr 和 len 方法取出它们并独立地作为实际的系统调用参数。
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}