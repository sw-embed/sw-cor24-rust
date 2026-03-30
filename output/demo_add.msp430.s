	.file	"msp430_demos.6c71831c4718c121-cgu.0"
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

	.section	.text.accumulate,"ax",@progbits
	.globl	accumulate
	.p2align	1
	.type	accumulate,@function
accumulate:
	push	r6
	push	r7
	push	r8
	push	r9
	push	r10
	mov	r12, r10
	mov	r10, r6
	inc	r6
	add	r6, r10
	mov	r10, r9
	add	r6, r9
	mov	r10, r13
	xor	r6, r13
	xor	r9, r13
	mov	r9, r8
	add	r10, r8
	xor	r8, r13
	mov	r8, r7
	add	r9, r7
	xor	r7, r13
	mov	#-256, r12
	call	#mmio_write
	mov	r6, r12
	call	#uart_putc
	mov	r10, r12
	call	#uart_putc
	mov	r9, r12
	call	#uart_putc
	mov	r8, r12
	call	#uart_putc
	mov	r7, r12
	call	#uart_putc
.LBB1_1:
	jmp	.LBB1_1
.Lfunc_end1:
	.size	accumulate, .Lfunc_end1-accumulate

	.section	.text.delay,"ax",@progbits
	.globl	delay
	.p2align	1
	.type	delay,@function
delay:
	sub	#2, r1
	tst	r12
	jeq	.LBB2_3
	add	#-1, r12
.LBB2_2:
	mov	r12, 0(r1)
	add	#-1, r12
	cmp	#-1, r12
	jne	.LBB2_2
.LBB2_3:
	add	#2, r1
	ret
.Lfunc_end2:
	.size	delay, .Lfunc_end2-delay

	.section	.text.demo_add,"ax",@progbits
	.globl	demo_add
	.p2align	1
	.type	demo_add,@function
demo_add:
	mov	#342, r12
	ret
.Lfunc_end3:
	.size	demo_add, .Lfunc_end3-demo_add

	.section	.text.demo_blinky,"ax",@progbits
	.globl	demo_blinky
	.p2align	1
	.type	demo_blinky,@function
demo_blinky:
.LBB4_1:
	mov	#-256, r12
	mov	#1, r13
	call	#mmio_write
	mov	#5000, r12
	call	#delay
	mov	#-256, r12
	clr	r13
	call	#mmio_write
	mov	#5000, r12
	call	#delay
	jmp	.LBB4_1
.Lfunc_end4:
	.size	demo_blinky, .Lfunc_end4-demo_blinky

	.section	.text.demo_button_echo,"ax",@progbits
	.globl	demo_button_echo
	.p2align	1
	.type	demo_button_echo,@function
demo_button_echo:
.LBB5_1:
	mov	#-256, r12
	call	#mmio_read
	mov	r12, r13
	and	#1, r13
	mov	#-256, r12
	call	#mmio_write
	jmp	.LBB5_1
.Lfunc_end5:
	.size	demo_button_echo, .Lfunc_end5-demo_button_echo

	.section	.text.demo_countdown,"ax",@progbits
	.globl	demo_countdown
	.p2align	1
	.type	demo_countdown,@function
demo_countdown:
	push	r10
	mov	#10, r10
.LBB6_1:
	mov	#-256, r12
	mov	r10, r13
	call	#mmio_write
	mov	#1000, r12
	call	#delay
	add	#-1, r10
	tst	r10
	jne	.LBB6_1
	mov	#-256, r12
	clr	r13
	call	#mmio_write
.LBB6_3:
	jmp	.LBB6_3
.Lfunc_end6:
	.size	demo_countdown, .Lfunc_end6-demo_countdown

	.section	.text.demo_fibonacci,"ax",@progbits
	.globl	demo_fibonacci
	.p2align	1
	.type	demo_fibonacci,@function
demo_fibonacci:
	mov	#-256, r12
	mov	#55, r13
	call	#mmio_write
.LBB7_1:
	jmp	.LBB7_1
.Lfunc_end7:
	.size	demo_fibonacci, .Lfunc_end7-demo_fibonacci

	.section	.text.demo_nested,"ax",@progbits
	.globl	demo_nested
	.p2align	1
	.type	demo_nested,@function
demo_nested:
	mov	#-256, r12
	call	#mmio_read
	add	#5, r12
	call	#level_a
.Lfunc_end8:
	.size	demo_nested, .Lfunc_end8-demo_nested

	.section	.text.demo_stack_vars,"ax",@progbits
	.globl	demo_stack_vars
	.p2align	1
	.type	demo_stack_vars,@function
demo_stack_vars:
	mov	#-256, r12
	call	#mmio_read
	inc	r12
	call	#accumulate
.Lfunc_end9:
	.size	demo_stack_vars, .Lfunc_end9-demo_stack_vars

	.section	.text.demo_uart_hello,"ax",@progbits
	.globl	demo_uart_hello
	.p2align	1
	.type	demo_uart_hello,@function
demo_uart_hello:
	mov	#72, r12
	call	#uart_putc
	mov	#101, r12
	call	#uart_putc
	mov	#108, r12
	call	#uart_putc
	mov	#108, r12
	call	#uart_putc
	mov	#111, r12
	call	#uart_putc
	mov	#10, r12
	call	#uart_putc
.LBB10_1:
	jmp	.LBB10_1
.Lfunc_end10:
	.size	demo_uart_hello, .Lfunc_end10-demo_uart_hello

	.section	.text.fibonacci,"ax",@progbits
	.globl	fibonacci
	.p2align	1
	.type	fibonacci,@function
fibonacci:
	cmp	#2, r12
	jhs	.LBB11_2
	mov	r12, r13
	jmp	.LBB11_4
.LBB11_2:
	mov	#2, r14
	clr	r15
	mov	#1, r13
.LBB11_3:
	mov	r13, r11
	mov	r15, r13
	add	r11, r13
	inc	r14
	cmp	r14, r12
	mov	r11, r15
	jhs	.LBB11_3
.LBB11_4:
	mov	r13, r12
	ret
.Lfunc_end11:
	.size	fibonacci, .Lfunc_end11-fibonacci

	.section	.text.level_a,"ax",@progbits
	.globl	level_a
	.p2align	1
	.type	level_a,@function
level_a:
	add	#10, r12
	call	#level_b
.Lfunc_end12:
	.size	level_a, .Lfunc_end12-level_a

	.section	.text.level_b,"ax",@progbits
	.globl	level_b
	.p2align	1
	.type	level_b,@function
level_b:
	mov	r12, r13
	add	r12, r12
	add	#3, r12
	call	#level_c
.Lfunc_end13:
	.size	level_b, .Lfunc_end13-level_b

	.section	.text.level_c,"ax",@progbits
	.globl	level_c
	.p2align	1
	.type	level_c,@function
level_c:
	push	r10
	mov	r13, r10
	mov	r12, r13
	mov	#-256, r12
	call	#mmio_write
	mov	r10, r12
	call	#uart_putc
.LBB14_1:
	jmp	.LBB14_1
.Lfunc_end14:
	.size	level_c, .Lfunc_end14-level_c

	.section	.text.mmio_read,"ax",@progbits
	.globl	mmio_read
	.p2align	1
	.type	mmio_read,@function
mmio_read:
	mov	0(r12), r12
	ret
.Lfunc_end15:
	.size	mmio_read, .Lfunc_end15-mmio_read

	.section	.text.mmio_write,"ax",@progbits
	.globl	mmio_write
	.p2align	1
	.type	mmio_write,@function
mmio_write:
	mov	r13, 0(r12)
	ret
.Lfunc_end16:
	.size	mmio_write, .Lfunc_end16-mmio_write

	.section	.text.uart_putc,"ax",@progbits
	.globl	uart_putc
	.p2align	1
	.type	uart_putc,@function
uart_putc:
	mov	r12, r13
	mov	#-255, r12
	call	#mmio_write
	ret
.Lfunc_end17:
	.size	uart_putc, .Lfunc_end17-uart_putc

	.ident	"rustc version 1.93.0-nightly (c871d09d1 2025-11-24)"
	.section	".note.GNU-stack","",@progbits
