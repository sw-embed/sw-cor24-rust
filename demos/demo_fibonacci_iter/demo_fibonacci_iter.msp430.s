	.file	"demo_fibonacci_iter.36beb96266d37940-cgu.0"
	.section	.text._RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind,"ax",@progbits
	.hidden	_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind
	.globl	_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind
	.p2align	1
	.type	_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind,@function
_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind:
.LBB0_1:
	jmp	.LBB0_1
.Lfunc_end0:
	.size	_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind, .Lfunc_end0-_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind

	.section	.text.demo_fibonacci_iter,"ax",@progbits
	.globl	demo_fibonacci_iter
	.p2align	1
	.type	demo_fibonacci_iter,@function
demo_fibonacci_iter:
	mov	#10, r12
	call	#fibonacci_iter
	mov	r12, r13
	mov	#256, r12
	call	#mem_write
.LBB1_1:
	jmp	.LBB1_1
.Lfunc_end1:
	.size	demo_fibonacci_iter, .Lfunc_end1-demo_fibonacci_iter

	.section	.text.fibonacci_iter,"ax",@progbits
	.globl	fibonacci_iter
	.p2align	1
	.type	fibonacci_iter,@function
fibonacci_iter:
	;APP
	; @cor24: push    r1
	; @cor24: lc      r1, 1
	; @cor24: lc      r2, 1
	; @cor24: ceq     r0, z
	; @cor24: brt     .fib_done
	; @cor24: .fib_loop:
	; @cor24: push    r1
	; @cor24: mov     r1, r2
	; @cor24: pop     r2
	; @cor24: add     r2, r1
	; @cor24: add     r0, -1
	; @cor24: ceq     r0, z
	; @cor24: brf     .fib_loop
	; @cor24: .fib_done:
	; @cor24: mov     r0, r1
	; @cor24: pop     r1
	;NO_APP
	ret
.Lfunc_end2:
	.size	fibonacci_iter, .Lfunc_end2-fibonacci_iter

	.section	.text.mem_write,"ax",@progbits
	.globl	mem_write
	.p2align	1
	.type	mem_write,@function
mem_write:
	mov.b	r13, 0(r12)
	ret
.Lfunc_end3:
	.size	mem_write, .Lfunc_end3-mem_write

	.section	.text.start,"ax",@progbits
	.globl	start
	.p2align	1
	.type	start,@function
start:
	call	#demo_fibonacci_iter
.Lfunc_end4:
	.size	start, .Lfunc_end4-start

	.ident	"rustc version 1.93.0-nightly (c871d09d1 2025-11-24)"
	.section	".note.GNU-stack","",@progbits
