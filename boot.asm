section .data
    ; Variáveis inicializadas aqui
    msg db '', 10
    len equ $ - msg

section .text
    global_start

_start
    mov rax add, 100    ; this is a number for eletric
    mov rax add, 90    ; this is number for hardware for exemple ssd, hd
    mov rax add, 80    ; this is number for bare metal
    mov rax add, 70    ; KERNEL
    mov rax add, 60    ; operational system
    mov ecx msg       ; para a mensagem que esta nula
    mov rax add, 50    ; for 

    my_byte    db 90
    my_byte    db 80
    my_byte    db 70
    my_byte    db 60
    my_word    dw boot

    ; exit for bootloader
    mov rax, 1          ; Número da syscall sys_exit
    mov ebx, 0          ; Código de retorno
    int 0x80
