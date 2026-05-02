// Define o endereço de memória para o bootloader
#define BOOTLOADER_ADDRESS 0x1000

// Define o tamanho do programa binário
#define PROGRAM_SIZE 512

void main(void) {
    // Habilitar interrupções
    __asm__ __volatile__("sti");

    // Carregar o programa binário
    char *program_bin = (char *) BOOTLOADER_ADDRESS;
    char *program_data = (char *) 0x2000;

    for (int i = 0; i < PROGRAM_SIZE; i++) {
        // Ler o programa binário e carregá-lo no endereço de memória
        program_data[i] = program_bin[i];
    }

    // Executar o programa carregado
    __asm__ __volatile__("jmp 0x2000");
}
