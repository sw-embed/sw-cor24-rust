	.text

	.globl	_fib
_fib:
	push	fp
	push	r2
	push	r1
	mov	fp,sp
	add	sp,-3
	lw	r2,9(fp)
; line 7, file "fib.c"
	lc	r0,2
	cls	r2,r0
	brf	L17
; line 8, file "fib.c"
	lc	r0,1
	bra	L16
L17:
; line 11, file "fib.c"
	mov	r0,r2
	add	r0,-1
	push	r0
	la	r0,_fib
	jal	r1,(r0)
	add	sp,3
	sw	r0,-3(fp)
	mov	r0,r2
	add	r0,-2
	push	r0
	la	r0,_fib
	jal	r1,(r0)
	add	sp,3
	lw	r1,-3(fp)
	add	r0,r1
L16:
	mov	sp,fp
	pop	r1
	pop	r2
	pop	fp
	jmp	(r1)

	.globl	_main
_main:
	push	fp
	push	r2
	push	r1
	mov	fp,sp
	add	sp,-3
; line 18, file "fib.c"
	la	r0,L20
	push	r0
	la	r0,_printf
	jal	r1,(r0)
	add	sp,3
; line 20, file "fib.c"
	lc	r0,33
	push	r0
	la	r0,_fib
	jal	r1,(r0)
	add	sp,3
	sw	r0,-3(fp)
; line 22, file "fib.c"
	lw	r0,-3(fp)
	push	r0
	la	r0,L21
	push	r0
	la	r0,_printf
	jal	r1,(r0)
	add	sp,6
; line 24, file "fib.c"
	lc	r0,0
	mov	sp,fp
	pop	r1
	pop	r2
	pop	fp
	jmp	(r1)

	.data
L20:
	.byte	70,105,98,111,110,97,99,99
	.byte	105,32,51,51,10,0
L21:
	.byte	37,100,10,0
