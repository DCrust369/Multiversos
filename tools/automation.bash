#!/bin/bash
# direitos autorais DCrust 16/04/2026
# copyright (C) DCrust 16/04/2026

echo "select the execution mode:"

select option in "emprestimo_zig" "build_zig" "cargo_run" "zig_run" "sair"; do
    case $option in
        "emprestimo_zig") zig run src/main.zig  ;;
        "build_zig")      zig build              ;;
        "cargo_run")      cargo run              ;;
        "zig_run")        zig run src/main.zig  ;;
        "sair")           break                  ;;
        *) echo "Opção inválida"                 ;;
    esac
done
