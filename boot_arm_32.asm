; Bootloader em assembly ARM 32 bits

; Definindo as constantes
STACK_SIZE = 4096

; Definindo as rotinas de inicialização
section .data
align 4
stack:
  resb STACK_SIZE
; Fim da seção de dados

section .text
align 4
global _start
_start:
; Inicializar o stack
mov sp, stack + STACK_SIZE
; Fim da inicialização do stack

; Carregar o kernel
mov r0, #0x8000 ; endereço do kernel
ldr pc, =kernel
; Fim da carregamento do kernel

; Rotina de tratamento de erro (kernel não foi carregado)
erro_kernel:
  mov r0, #1
  mov r1, #0
  mov r2, #0
  mov r3, #0
  bx lr

; Kernel
; Definindo as constantes
KERNEL_SIZE = 4096

; Definindo as rotinas de inicialização
section .data
align 4
kernel:
  resb KERNEL_SIZE
; Fim da seção de dados

section .text
align 4
global kernel_start
kernel_start:
; Inicializar o kernel
; Fim da inicialização do kernel

; Rotina de tratamento de erro (modo não foi definido)
erro_modo:
  mov r0, #1
  mov r1, #0
  mov r2, #0
  mov r3, #0
  bx lr

; Fim do kernel
; Fim do bootloader
