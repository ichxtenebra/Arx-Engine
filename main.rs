format ELF64
use64

extrn arx_engine_ignite

public arx_start

section '.text' executable

arx_start:
    xor  edi, edi
    xor  esi, esi
    mov  edx, 800
    mov  ecx, 600
    call arx_engine_ignite
