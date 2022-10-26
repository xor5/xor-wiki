global _start

section .data
    msg: db	"Hello, Asm!", 10
    .len: equ $ - msg

section .text

_start: db 0x90
    mov r11, _start+0x0f
    jmp r11
    ret
    nop
    mov rax, label+1
    add r11, 0x15
    jmp -20
    ret
    call lol+2
    ret

lol: db 0xeb, 0xff
    jmp rax

label: db 0xE9
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, msg.len
    syscall

    mov rdi, 1
    mov rax, 60
	xor rdi, rdi
    syscall