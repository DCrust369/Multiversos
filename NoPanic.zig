// direitos autorais (Rust/Zig) DCrust 16/04/2026

const std = @import("std");
const NoPanic = @import("NoPanic.zig");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Executável principal
    const exe = b.addExecutable(.{
        .name = "dcrust-kernel",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Arquiteturas suportadas
    const x86_64_target = std.Target.Query{
        .cpu_arch = .x86_64,
    };
    const arm64_target = std.Target.Query{
        .cpu_arch = .aarch64,
    };

    _ = x86_64_target;
    _ = arm64_target;

    b.installArtifact(exe);
}
