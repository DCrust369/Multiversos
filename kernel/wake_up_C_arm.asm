.org 0x8000

.equ    STACK_SIZE, 0x1000
.equ    KERNEL_ADDRESS, 0x30000000
.equ    KERNEL_SIZE, 0x10000

_start:
    ldr     sp, =STACK_SIZE
    mov     r0, #0
    mov     r1, #0
    bl      kernel_main

    b       1f

_stack:
    .space  STACK_SIZE
1:
    b       1b

kernel_main:
    // Aqui você deve implementar a sua própria função de inicialização do kernel
    // Por exemplo:
    // ldr     r0, =KERNEL_ADDRESS  @ endereço do kernel
    // mov     r1, #KERNEL_SIZE     @ tamanho do kernel
    // b       kernel_start

    // Aqui você pode implementar um loop infinito para manter o bootloader acordado
    // Por exemplo:
    b       1b

kernel_start:
    // Aqui você deve implementar a sua própria função de inicialização do kernel
    // Por exemplo:
    // ldr     r0, =0x30000000  @ endereço do kernel
    // mov     r1, #0x1000      @ tamanho do kernel
    // b       kernel_entry

    // Aqui você pode implementar o seu código do kernel
    // Por exemplo:
    // mov     r0, #0x1000
    // mov     r1, #0x2000
    // bl      exemplo

    // Aqui você pode implementar um loop infinito para manter o kernel executando
    // Por exemplo:
    b       kernel_start

exemplo:
    // Aqui você pode implementar um exemplo de código do kernel
    // Por exemplo:
    mov     r0, #0x1000
    mov     r1, #0x2000
    bl      ejemplo_entry

    // Aqui você pode implementar um loop infinito para manter o exemplo executando
    // Por exemplo:
    b       exemplo

exemplo_entry:
    // Aqui você pode implementar o seu código de entrada do exemplo
    // Por exemplo:
    mov     r0, #0x1000
    mov     r1, #0x2000
    bl      exemplo_exec

    // Aqui você pode implementar um loop infinito para manter o exemplo executando
    // Por exemplo:
    b       exemplo_entry

exemplo_exec:
    // Aqui você pode implementar o seu código de execução do exemplo
    // Por exemplo:
    mov     r0, #0x1000
    mov     r1, #0x2000
    bl      exemplo_exit

    // Aqui você pode implementar um loop infinito para manter o exemplo executando
    // Por exemplo:
    b       exemplo_exec

exemplo_exit:
    // Aqui você pode implementar o seu código de saída do exemplo
    // Por exemplo:
    mov     r0, #0x1000
    mov     r1, #0x2000
    b       exemplo_exit
