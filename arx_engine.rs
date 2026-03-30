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

const OBSIDIAN_VOID: u32 = 0x000A0A0F;
const MIDNIGHT_PANEL: u32 = 0x0012121E;
const AMBER_ACCENT: u32 = 0x00C8A878;
const IVORY_RULE_COLOR: u32 = 0x00E8E4DF;
const DIM_BRONZE: u32 = 0x003D3528;
const SLATE_EDGE: u32 = 0x001E1E2C;

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
unsafe fn w32(buf: *mut u8, off: usize, val: u32) {
    unsafe { buf.add(off).cast::<u32>().write_unaligned(val.to_le()) }
}

#[inline(always)]
unsafe fn w16(buf: *mut u8, off: usize, val: u16) {
    unsafe { buf.add(off).cast::<u16>().write_unaligned(val.to_le()) }
}

#[inline(always)]
unsafe fn ws16(buf: *mut u8, off: usize, val: i16) {
    unsafe { buf.add(off).cast::<i16>().write_unaligned(val.to_le()) }
}

#[inline(always)]
unsafe fn r32(bp: *const u8, off: usize) -> u32 {
    unsafe { core::ptr::read_unaligned(bp.add(off) as *const u32) }
}

#[inline(always)]
unsafe fn r16(bp: *const u8, off: usize) -> u16 {
    unsafe { core::ptr::read_unaligned(bp.add(off) as *const u16) }
}

unsafe fn renderer(ox: i16, oy: i16, w: u16, h: u16) -> ! {
    unsafe {
        let fd = sys_socket();
        sys_connect(fd, SOCKADDR.as_ptr(), 20);
        sys_write(fd, SETUP_REQ.as_ptr(), 12);

        let mut buf = core::mem::MaybeUninit::<[u8; 4096]>::uninit();
        let bp = buf.as_mut_ptr() as *mut u8;
        sys_read(fd, bp, 4096);

        let rbp = bp as *const u8;
        let id_base = r32(rbp, 12);
        let vendor_len = r16(rbp, 24) as usize;
        let num_fmts = *rbp.add(29) as usize;
        let padded_vendor = (vendor_len + 3) & !3;
        let scr_off = 40 + padded_vendor + num_fmts * 8;

        let root_wid = r32(rbp, scr_off);
        let scr_w = r16(rbp, scr_off + 20) as i32;
        let scr_h = r16(rbp, scr_off + 22) as i32;
        let root_visual = r32(rbp, scr_off + 32);
        let root_depth = *rbp.add(scr_off + 38);

        let wid = id_base | 1;
        let gid = id_base | 2;

        let cx = ((scr_w - w as i32) / 2 + ox as i32) as i16;
        let cy = ((scr_h - h as i32) / 2 + oy as i32) as i16;

        let panel_h = (h / 12) as i16;
        let stripe_y = panel_h / 3;
        let margin = (w / 50) as i16;
        let body_h = h as i16 - panel_h;
        let rule1_y = panel_h + (body_h as i32 * 809 / 1309) as i16;
        let rule2_y = panel_h + (body_h as i32 * 500 / 1309) as i16;
        let w_i16 = w as i16;

        *bp.add(0) = 1;
        *bp.add(1) = root_depth;
        w16(bp, 2, 10);
        w32(bp, 4, wid);
        w32(bp, 8, root_wid);
        ws16(bp, 12, cx);
        ws16(bp, 14, cy);
        w16(bp, 16, w);
        w16(bp, 18, h);
        w16(bp, 20, 0);
        w16(bp, 22, 1);
        w32(bp, 24, root_visual);
        w32(bp, 28, 0x00000202);
        w32(bp, 32, OBSIDIAN_VOID);
        w32(bp, 36, 1);

        *bp.add(40) = 55;
        *bp.add(41) = 0;
        w16(bp, 42, 5);
        w32(bp, 44, gid);
        w32(bp, 48, wid);
        w32(bp, 52, 0x00000004);
        w32(bp, 56, MIDNIGHT_PANEL);

        *bp.add(60) = 70;
        *bp.add(61) = 0;
        w16(bp, 62, 5);
        w32(bp, 64, wid);
        w32(bp, 68, gid);
        ws16(bp, 72, 0);
        ws16(bp, 74, 0);
        w16(bp, 76, w);
        w16(bp, 78, panel_h as u16);

        *bp.add(80) = 56;
        *bp.add(81) = 0;
        w16(bp, 82, 4);
        w32(bp, 84, gid);
        w32(bp, 88, 0x00000004);
        w32(bp, 92, AMBER_ACCENT);

        *bp.add(96) = 70;
        *bp.add(97) = 0;
        w16(bp, 98, 5);
        w32(bp, 100, wid);
        w32(bp, 104, gid);
        ws16(bp, 108, 0);
        ws16(bp, 110, stripe_y);
        w16(bp, 112, w);
        w16(bp, 114, 2);

        *bp.add(116) = 66;
        *bp.add(117) = 0;
        w16(bp, 118, 5);
        w32(bp, 120, wid);
        w32(bp, 124, gid);
        ws16(bp, 128, 0);
        ws16(bp, 130, 0);
        ws16(bp, 132, w_i16 - 1);
        ws16(bp, 134, 0);

        *bp.add(136) = 56;
        *bp.add(137) = 0;
        w16(bp, 138, 4);
        w32(bp, 140, gid);
        w32(bp, 144, 0x00000004);
        w32(bp, 148, IVORY_RULE_COLOR);

        *bp.add(152) = 66;
        *bp.add(153) = 0;
        w16(bp, 154, 5);
        w32(bp, 156, wid);
        w32(bp, 160, gid);
        ws16(bp, 164, 0);
        ws16(bp, 166, panel_h);
        ws16(bp, 168, w_i16 - 1);
        ws16(bp, 170, panel_h);

        *bp.add(172) = 56;
        *bp.add(173) = 0;
        w16(bp, 174, 4);
        w32(bp, 176, gid);
        w32(bp, 180, 0x00000004);
        w32(bp, 184, SLATE_EDGE);

        *bp.add(188) = 66;
        *bp.add(189) = 0;
        w16(bp, 190, 5);
        w32(bp, 192, wid);
        w32(bp, 196, gid);
        ws16(bp, 200, margin);
        ws16(bp, 202, rule2_y);
        ws16(bp, 204, w_i16 - margin - 1);
        ws16(bp, 206, rule2_y);

        *bp.add(208) = 56;
        *bp.add(209) = 0;
        w16(bp, 210, 4);
        w32(bp, 212, gid);
        w32(bp, 216, 0x00000004);
        w32(bp, 220, DIM_BRONZE);

        *bp.add(224) = 66;
        *bp.add(225) = 0;
        w16(bp, 226, 5);
        w32(bp, 228, wid);
        w32(bp, 232, gid);
        ws16(bp, 236, margin);
        ws16(bp, 238, rule1_y);
        ws16(bp, 240, w_i16 - margin - 1);
        ws16(bp, 242, rule1_y);

        *bp.add(244) = 8;
        *bp.add(245) = 0;
        w16(bp, 246, 2);
        w32(bp, 248, wid);

        sys_write(fd, bp as *const u8, 252);

        loop {
            let nr = sys_read(fd, bp, 4096);
            if nr <= 0 {
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