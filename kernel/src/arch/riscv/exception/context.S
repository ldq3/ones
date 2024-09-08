.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __call_sys
    .globl __ret_user
    .align 2

# handle_exp()
# handle exception
__handle_exp:
    # sp -> user stack, sscratch -> kernel stack, the next instruction will switch them
    csrw sscratch, sp // FIXME
    csrrw sp, sscratch, sp
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8
    # save general-purpose registers on kernel stack
    sd x1, 1*8(sp)
    ## skip x2(sp)
    sd x3, 3*8(sp)
    ## skip x4(tp)
    ## x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

    ## sstatus and sepc
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    ## user stack pointer
    csrr t2, sscratch
    sd t2, 2*8(sp)

    # call distribute(cx: &mut Context)
    mv a0, sp
    call distribute

# expt_ret(cx: &mut Context)
# return after handling exception
__expt_ret:
    mv sp, a0
    # restore sstatus, sepc and user stack pointer
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    addi sp, sp, 34*8
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp
    sret