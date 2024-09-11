section .data
msg db 'Result: ', 0
buffer db 20 dup(0)
v0 dq 0
v1 dq 0
v2 dq 0
section .text
global _start
_start:
mov rax, 12
mov [v0], qword rax
mov rax, 13
mov [v1], qword rax
mov rax, qword [v1]
mov rax, qword [v0]
add rax, rbx
mov [v2], qword rax
mov rax, qword [v2]
mov rsi, rax
call print_integer
mov rsi, msg
mov rdx, 8
mov rax, 1
mov rdi, 1
syscall
mov rax, 60
xor rdi, rdi
syscall
print_integer:
    mov rbx, 10
    xor rdx, rdx
    mov rdi, buffer + 19
    mov byte [rdi], 0
convert_loop:
    dec rdi
    div rbx
    add dl, '0'
    mov [rdi], dl
    test rax, rax
    jnz convert_loop
    mov rsi, rdi
    mov rdx, buffer + 19 - rdi
    ret
