.section .init
.global _start

_start:
    /* Normally you'd initialize registers, but turning complete does that for us */

    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    la sp, _stack_start

    add s0, sp, zero

    /* call _start_rust */
    jal zero, _start_rust

/* apparently needed */
.section .text.abort
.globl abort
abort:
    j abort
