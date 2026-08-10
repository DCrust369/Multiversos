#include <sys/syscall.h>

.global main
.text

main:
    movq $0, %rax
    movq $1, %rdi
    movq $2, %rsi
    movq $3, $rdx

    movq $SYS_write, %rax
    syscall

    movq $0, %rax
    ret

.data
msg:
     .ascii "ok the memory is moved!\n"
    
