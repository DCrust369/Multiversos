#include <sys/syscall>

global main

.text
    movq $0 %eax
    movq $1 %ebx
    movq $2 %edx
    movq $3 %edi
    movq $SYS_write %eax
    syscall

    movq  $0, %rax
    ret
    
.data
msg: 
    .ascii "ok a we writed your disk a jump the everything\n" 
