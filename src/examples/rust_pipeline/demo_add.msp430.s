	.file	"demo_add.987a4901da4884d3-cgu.0"
	.section	.text.demo_add,"ax",@progbits
	.globl	demo_add
	.p2align	1
	.type	demo_add,@function
demo_add:
	mov	#342, r12
	ret
.Lfunc_end0:
	.size	demo_add, .Lfunc_end0-demo_add

	.section	.text.start,"ax",@progbits
	.globl	start
	.p2align	1
	.type	start,@function
start:
	call	#demo_add
	mov	r12, &256
.LBB1_1:
	jmp	.LBB1_1
.Lfunc_end1:
	.size	start, .Lfunc_end1-start

	.section	.text._RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind,"ax",@progbits
	.hidden	_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind
	.globl	_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind
	.p2align	1
	.type	_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind,@function
_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind:
.LBB2_1:
	jmp	.LBB2_1
.Lfunc_end2:
	.size	_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind, .Lfunc_end2-_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind

	.ident	"rustc version 1.93.0-nightly (c871d09d1 2025-11-24)"
	.section	".note.GNU-stack","",@progbits
