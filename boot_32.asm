section .data
    ; Variáveis inicializadas aqui
    msg db '', 10
    len equ $ - msg

section .text
    global_start

_start
    mov eax add, 100    ; this is a number for eletric
    mov eax add, 90    ; this is number for hardware for exemple ssd, hd
    mov eax add, 80    ; this is number for bare metal
    mov eax add, 70    ; KERNEL
    mov eax add, 60    ; operational system
    mov eax msg       ; para a mensagem que esta nula
    mov eax add, 50    ; for 

    my_byte    db 90
    my_byte    db 80
    my_byte    db 70
    my_byte    db 60
    my_word    dw boot

    ; exit for bootloader
    mov eax, 1          ; Número da syscall sys_exit
    mov ebx, 0          ; Código de retorno
    int 0x80
