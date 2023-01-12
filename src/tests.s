.section .init
.global _start

/*
ecall is used to signal a successful test. ebreak is a failed test.
_start is at 0x1000, so we can use anything before that to test memory
*/
_start:
    j test1
    ebreak
test1:
    ecall

    addi x6, zero, 40
    addi x7, zero, 41
    bne x6, x7, test2
    ebreak
test2:
    ecall

    addi x6, zero, 40
    addi x7, zero, 41
    addi x6, x6, 1
    beq x6, x7, test3
    ebreak
test3:
    ecall

    lui x8, %hi(0x11111111)
    addi x7, zero, 0x11
    addi x6, zero, 8
    sll x7, x7, x6
    addi x7, x7, 0x11
    sll x7, x7, x6
    addi x7, x7, 0x10
    addi x6, zero, 8
    sll x7, x7, x6
    beq x7, x8, test4
    ebreak
test4:
    ecall

    jal ra, test5_mid
test5_test:
    ebreak
test5_mid:
    la x6, test5_test
    beq ra, x6, test5
    ebreak
test5:
    ecall

    addi x31, zero, 0x11
    slli x31, x31, 24
    lui x30, %hi(0x11000000)
    beq x30, x31, test6
    ebreak
test6:
    ecall

    lui x16, %hi(0xdeadbeef)
    addi x16, x16, %lo(0xdeadbeef)
    sw x16, 0(zero)
    lw x14, 0(zero)
    beq x14, x16, test7
    ebreak
test7:
    ecall

    lui x17, %hi(0xdeadbeef)
    addi x17, x17, %lo(0xdeadbeef)
    lui x16, %hi(0xdead)
    addi x16, x16, %lo(0xdead)
    lui x15, %hi(0xbeef)
    addi x15, x15, %lo(0xbeef)
    sh x16, 2(zero)
    sh x15, 0(zero)
    lw x14, 0(zero)
    beq x14, x17, test8
    ebreak
test8:
    ecall

    lui x19, %hi(0xdeadbeef)
    addi x19, x19, %lo(0xdeadbeef)
    addi x15, zero, 0xde
    addi x16, zero, 0xad
    addi x17, zero, 0xbe
    addi x18, zero, 0xef
    sb x15, 3(zero)
    sb x16, 2(zero)
    sb x17, 1(zero)
    sb x18, 0(zero)
    lw x14, 0(zero)
    beq x14, x19, test9
    ebreak
test9:
    ecall

    lui x6, %hi(0xdeadbeef)
    addi x6, x6, %lo(0xdeadbeef)
    lui x7, %hi(0xbeef)
    addi x7, x7, %lo(0xbeef)
    sw x6, 0(zero)
    lhu x5, 0(zero)
    beq x5, x7, test10
    ebreak
test10:
    ecall

    lui x6, %hi(0xdeadbeef)
    addi x6, x6, %lo(0xdeadbeef)
    lui x7, %hi(0xadbe)
    addi x7, x7, %lo(0xadbe)
    sw x6, 0(zero)
    lhu x5, 1(zero)
    beq x5, x7, test11
    ebreak
test11:
    ecall

    lui x6, %hi(0xdeadbeef)
    addi x6, x6, %lo(0xdeadbeef)
    lui x7, %hi(0xdead)
    addi x7, x7, %lo(0xdead)
    sw x6, 0(zero)
    lhu x5, 2(zero)
    beq x5, x7, test12
    ebreak
test12:
    ecall

loop:
    j loop
