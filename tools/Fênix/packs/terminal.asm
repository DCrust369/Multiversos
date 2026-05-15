bits 32                         ; Operar em 32 bits (Padrão Multiboot)

section .multiboot              ; Cabeçalho vital para o GRUB
    align 4
    dd 0x1BADB002               ; Magic Number
    dd 0x00                     ; Flags
    dd -(0x1BADB002 + 0x00)     ; Checksum

section .text
global _start

_start:
    mov esp, stack_space        ; Configura a pilha de memória

    ; --- LIMPAR A TELA ---
    mov edi, 0xB8000            ; Endereço da memória de vídeo VGA
    mov ecx, 80 * 25            ; Tamanho da tela (80 colunas x 25 linhas)
    mov ax, 0x0F20              ; Fundo preto (0), Letra branca (F), Espaço (20)
    rep stosw                   ; Preenche a tela toda

    ; --- ESCREVER PROMPT ---
    mov esi, msg_prompt
    mov edi, 0xB8000            ; Começa no topo da tela
    mov ah, 0x02                ; COR VERDE (Hacker mode)
    call print_string

    ; --- SIMULAR COMANDO PENTEST ---
    mov esi, msg_command
    mov edi, 0xB8000 + 40       ; Move para frente do prompt (20 chars * 2 bytes)
    mov ah, 0x0F                ; COR BRANCA
    call print_string

    ; --- SIMULAR OUTPUT ---
    mov esi, msg_output
    mov edi, 0xB8000 + 160      ; Próxima linha (80 chars * 2 bytes)
    mov ah, 0x04                ; COR VERMELHA (Alerta)
    call print_string

    hlt                         ; Para o processador

; Função auxiliar para imprimir strings
print_string:
.loop:
    lodsb                       ; Carrega char da string para AL
    or al, al                   ; Checa se é o fim (0)
    jz .done
    mov [edi], al               ; Escreve o char na memória de vídeo
    mov [edi+1], ah             ; Escreve a cor
    add edi, 2                  ; Próximo caractere na tela
    jmp .loop
.done:
    ret

section .data
    msg_prompt  db "universos@pentest:# ", 0
    msg_command db "nmap -sV 192.168.1.1", 0
    msg_output  db "PORT 80: OPEN [HTTP SERVER FOUND]", 0

section .bss
    resb 8192                   ; Reserva 8KB para a pilha
stack_space:
