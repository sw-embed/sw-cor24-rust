	.file	"demo_drop.9b9184a6343c9577-cgu.0"
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

	.section	.text._RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop,"ax",@progbits
	.p2align	1
	.type	_RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop,@function
_RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop:
	mov	0(r12), r12
	clr	r13
	call	#mem_write
	ret
.Lfunc_end1:
	.size	_RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop, .Lfunc_end1-_RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop

	.section	.text.guard_new,"ax",@progbits
	.globl	guard_new
	.p2align	1
	.type	guard_new,@function
guard_new:
	push	r10
	mov	r12, r10
	mov	#1, r13
	call	#mem_write
	mov	r10, r12
	pop	r10
	ret
.Lfunc_end2:
	.size	guard_new, .Lfunc_end2-guard_new

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
	sub	#2, r1
	mov	#256, r12
	call	#guard_new
	mov	#256, 0(r1)
	mov	r1, r12
	call	#_RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop
	mov	#256, r12
	mov	#255, r13
	call	#mem_write
.LBB4_1:
	jmp	.LBB4_1
.Lfunc_end4:
	.size	start, .Lfunc_end4-start

	.ident	"rustc version 1.93.0-nightly (c871d09d1 2025-11-24)"
	.section	".note.GNU-stack","",@progbits
