format ELF64
use64

public arx_engine_ignite

section '.text' executable

arx_engine_ignite:

.save_args:
    mov  r12d, edi
    mov  r13d, esi
    mov  r14d, edx
    mov  r15d, ecx

.create_socket:
    mov  eax, 41
    mov  edi, 1
    mov  esi, 1
    xor  edx, edx
    syscall
    mov  rbx, rax

.connect_socket:
    mov  eax, 42
    mov  edi, ebx
    lea  rsi, [arx_sockaddr]
    mov  edx, 20
    syscall

.send_setup:
    mov  eax, 1
    mov  edi, ebx
    lea  rsi, [arx_setup_request]
    mov  edx, 12
    syscall

.read_response:
    xor  eax, eax
    mov  edi, ebx
    lea  rsi, [arx_read_buffer]
    mov  edx, 4096
    syscall

.parse:
    mov  eax, dword [arx_read_buffer + 12]
    inc  eax
    mov  r8d, eax
    movzx eax, word [arx_read_buffer + 24]
    add  eax, 3
    and  eax, 0xFFFFFFFC
    movzx ecx, byte [arx_read_buffer + 29]
    lea  ecx, [ecx * 8]
    add  eax, ecx
    add  eax, 40
    lea  rbp, [arx_read_buffer + rax]
    mov  r9d, dword [rbp]
    movzx eax, word [rbp + 20]
    movzx ecx, word [rbp + 22]
    mov  r10d, dword [rbp + 32]
    movzx r11d, byte [rbp + 38]

.center:
    sub  eax, r14d
    sar  eax, 1
    test r12d, r12d
    cmovnz eax, r12d
    sub  ecx, r15d
    sar  ecx, 1
    test r13d, r13d
    cmovnz ecx, r13d

.patch:
    mov  byte [arx_create_map_buffer + 1], r11b
    mov  dword [arx_create_map_buffer + 4], r8d
    mov  dword [arx_create_map_buffer + 8], r9d
    mov  word [arx_create_map_buffer + 12], ax
    mov  word [arx_create_map_buffer + 14], cx
    mov  word [arx_create_map_buffer + 16], r14w
    mov  word [arx_create_map_buffer + 18], r15w
    mov  dword [arx_create_map_buffer + 24], r10d
    mov  dword [arx_create_map_buffer + 44], r8d

.send_create:
    mov  eax, 1
    mov  edi, ebx
    lea  rsi, [arx_create_map_buffer]
    mov  edx, 48
    syscall

.event_loop:
    xor  eax, eax
    mov  edi, ebx
    lea  rsi, [arx_read_buffer]
    mov  edx, 4096
    syscall
    test rax, rax
    jg   .event_loop

arx_engine_shutdown:
    mov  eax, 231
    xor  edi, edi
    syscall

section '.data' writeable

arx_sockaddr:
    dw  1
    db  '/tmp/.X11-unix/X0', 0

arx_setup_request:
    db  0x6C, 0x00
    dw  11, 0
    dw  0, 0
    dw  0

arx_create_map_buffer:
    db  1, 0
    dw  10
    dd  0
    dd  0
    dw  0, 0
    dw  0, 0
    dw  0, 1
    dd  0
    dd  0x00000202
    dd  0x00000000
    dd  1
    db  8, 0
    dw  2
    dd  0

section '.bss' writeable

arx_read_buffer:
    rb  4096
