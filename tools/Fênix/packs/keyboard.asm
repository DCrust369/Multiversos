bits 32
section .multiboot
    align 4
    dd 0x1BADB002
    dd 0x03 ; Flags para pedir informações de memória e vídeo
    dd -(0x1BADB002 + 0x03)

section .text
global _start

_start:
    ; 1. Limpa a tela e mostra o prompt
    mov edi, 0xB8000
    mov ah, 0x02 ; Verde
    mov esi, msg_prompt
    call print_string

    ; 2. Loop de leitura do teclado (Polling - o jeito mais simples)
wait_key:
    in al, 0x64      ; Lê o status do controlador do teclado
    and al, 0x01      ; Verifica se há um caractere pronto
    jz wait_key       ; Se não, espera
    
    in al, 0x60      ; Lê o "Scan Code" da tecla pressionada
    
    ; Aqui, se AL < 0x80, uma tecla foi apertada.
    ; Para simplificar, vamos apenas imprimir um '*' cada vez que carregas numa tecla
    mov byte [edi], '*'
    mov byte [edi+1], 0x0A ; Verde claro
    add edi, 2
    
    jmp wait_key

print_string:
    lodsb
    or al, al
    jz .done
    mov [edi], al
    mov [edi+1], ah
    add edi, 2
    jmp print_string
.done:
    ret

section .data
    msg_prompt db "UNIVERSOS OS > Digite algo: ", 0
