const BUFFER_SIZE = 1024; // Exemplo

const Queue = struct {
    buf: [BUFFER_SIZE]u8,
    head: usize,
    tail: usize,
    proc_list: ?*u32,
};

fn outb(port: u16, value: u8) void {
    asm volatile ("outb %[value], %[port]"
        :
        : [value] "{al}" (value),
          [port] "N{dx}" (port),
    );
}

fn put_queue(queue: *Queue, value: u32) void {
    var temp_head = queue.head;
    var val = value;

    while (val > 0) {
        const next_head = (temp_head + 1) & (BUFFER_SIZE - 1);
        if (next_head == queue.tail) break; // Buffer cheio

        queue.buf[temp_head] = @truncate(val);
        temp_head = next_head;
        val >>= 8;
    }
    
    queue.head = temp_head;

    // Acorda o processo esperando pelo teclado, se houver
    if (queue.proc_list) |proc| {
        // Lógica de agendamento/scheduler aqui
        // Exemplo: proc.* = 0; 
    }
}
