const std = @import("std");

const StatusCripto = enum {
    criptografado,
    descriptografado,
};

const SistemaUniversos = struct {
    window: StatusCripto,
    browser: StatusCripto,
    screen: StatusCripto,
    bits: StatusCripto,
    files: StatusCripto,
    
    const link_bitcoin = "https://github.com/bitcoin/bitcoin";
    const link_aes = "https://github.com/ilvn/aes256";
};

pub fn main() void {
    const meu_kernel = SistemaUniversos{
        .window = StatusCripto.criptografado,
        .browser = StatusCripto.criptografado,
        .screen = StatusCripto.criptografado,
        .bits = StatusCripto.criptografado,
        .files = StatusCripto.criptografado,
    };
}
