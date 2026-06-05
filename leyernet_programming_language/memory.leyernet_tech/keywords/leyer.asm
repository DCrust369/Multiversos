section .data
    value dp 0
section .text
    global _start

_start
    ;****************************************
    ; the keyword for my programing language*
    ;****************************************
    lea rax, [#import]
    lea rax, [import-library]
    ;****************************************
    lea rax, [fn]
    lea rax, [function]
    ;****************************************
    lea rax, [till]
    lea rax, [local-advanced]
    ;************exit************************
    mov rax, 0
    mov rdi, 0
    syscall ; end
