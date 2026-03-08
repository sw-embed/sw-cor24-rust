/* COR24 assembler */
#include <ctype.h>
#include <memory.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Starting counter for labels inserted here */
#define LBLSTRT 20000

/* Branch instructions */
#define BRA 0x13
#define BRF 0x14
#define BRT 0x15

/* JMP <address> instruction */
#define JMPA 0xc7

/* SP adjustment instructions */
#define ADDSP 0x0c
#define SUBSP 0xa2
#define MOVSPFP 0x69

/* Compare register with zero instructions */
#define CEQ_R0_Z 0xc8
#define CEQ_R1_Z 0xc9
#define CEQ_R2_Z 0xca
#define CLU_Z_R0 0xce
#define CLU_Z_R1 0xcf
#define CLU_Z_R2 0xd0

/* Move to register from C instructions */
#define MOV_R0_C 0x62
#define MOV_R1_C 0x63
#define MOV_R2_C 0x64

/* Move register to register instructions */
#define MOV_R0_R1 0x56
#define MOV_R0_R2 0x57
#define MOV_R1_R0 0x5a
#define MOV_R1_R2 0x5b
#define MOV_R2_R0 0x5e
#define MOV_R2_R1 0x5f

/* Sign and zero extend register to register instructions */
#define SXT_R0_R0 0xaf
#define SXT_R1_R1 0xb3
#define SXT_R2_R2 0xb7
#define ZXT_R0_R0 0xbe
#define ZXT_R1_R1 0xc2
#define ZXT_R2_R2 0xc6

/* Instruction/data form definition/reference types */
#define FIXCMNT -8      /* Comment */
#define FIXBDAT -7      /* Byte data initialization */
#define FIXWDAT -6      /* Word data initialization */
#define FIXBUMP -5      /* Bump location counter */
#define FIXCOMM -4      /* Common data declaration */
#define FIXSYMB -3      /* Symbol value definition */
#define FIXLABL -2      /* Label definition */
#define FIXOTHR -1      /* Other directive */
#define FIXINST 0       /* Instruction in no need of fixup */
#define FIXPCD8 1       /* Reference to label, 8 bit PC relative */
#define FIXDD24 2       /* Data symbol reference, 24 bit */
#define FIXID24 3       /* Instruction symbol reference, 24 bit */

/* Pointer to end of string buffer */
#define EOBUF(s) (s + strlen(s))

/* Label counter */
static int labeln;

/* Error */
static int error;

/* Line buffer */
#define LINSIZ 132
static char linebuf[LINSIZ];

/* Section number, location counter */
static int8_t secnum;
static uint32_t curloc;

/* Tokens */
#define MAXTOKS 32
#define TOKLEN 32
static int ntokens;
static char tokens[MAXTOKS][TOKLEN + 1];

/* Instruction/data forms for binary generation */
struct insform {
    int8_t fixtype;
    int8_t section;
    uint8_t refcnt;
    uint8_t bytes[TOKLEN + 1];
    char symref[TOKLEN + 1];
    uint32_t length;
    uint32_t symval;
    uint32_t lineno;
    char *comment;
    struct insform *next;
};
static struct insform tmpform;
static struct insform *tmpf = &tmpform;
static struct insform *ifirst;

/* Instruction definitions */
struct insdef {
    int code;
    char toks[4][8];
};
static struct insdef instab[] = {

    /* Directives */
    { -1, { ".bss", "", "", "" }},
    { -1, { ".byte", "", "", "" }},
    { -1, { ".comm", "", "", "" }},
    { -1, { ".data", "", "", "" }},
    { -1, { ".globl", "", "", "" }},
    { -1, { ".text", "", "", "" }},
    { -1, { ".word", "", "", "" }},

    /* Instructions */
    { 0x00, { "add", "r0", "r0", "" }},
    { 0x01, { "add", "r0", "r1", "" }},
    { 0x02, { "add", "r0", "r2", "" }},
    { 0x03, { "add", "r1", "r0", "" }},
    { 0x04, { "add", "r1", "r1", "" }},
    { 0x05, { "add", "r1", "r2", "" }},
    { 0x06, { "add", "r2", "r0", "" }},
    { 0x07, { "add", "r2", "r1", "" }},
    { 0x08, { "add", "r2", "r2", "" }},
    { 0x09, { "add", "r0", "i8", "" }},
    { 0x0a, { "add", "r1", "i8", "" }},
    { 0x0b, { "add", "r2", "i8", "" }},
    { 0x0c, { "add", "sp", "i8", "" }},
    { 0x0d, { "and", "r0", "r1", "" }},
    { 0x0e, { "and", "r0", "r2", "" }},
    { 0x0f, { "and", "r1", "r0", "" }},
    { 0x10, { "and", "r1", "r2", "" }},
    { 0x11, { "and", "r2", "r0", "" }},
    { 0x12, { "and", "r2", "r1", "" }},
    { 0x13, { "bra", "d8", "", "" }},
    { 0x14, { "brf", "d8", "", "" }},
    { 0x15, { "brt", "d8", "", "" }},
    { 0x16, { "ceq", "r0", "r1", "" }},
    { 0x17, { "ceq", "r0", "r2", "" }},
    { 0x18, { "ceq", "r1", "r2", "" }},
    { 0x19, { "cls", "r0", "r1", "" }},
    { 0x1a, { "cls", "r0", "r2", "" }},
    { 0x1b, { "cls", "r1", "r0", "" }},
    { 0x1c, { "cls", "r1", "r2", "" }},
    { 0x1d, { "cls", "r2", "r0", "" }},
    { 0x1e, { "cls", "r2", "r1", "" }},
    { 0x1f, { "clu", "r0", "r1", "" }},
    { 0x20, { "clu", "r0", "r2", "" }},
    { 0x21, { "clu", "r1", "r0", "" }},
    { 0x22, { "clu", "r1", "r2", "" }},
    { 0x23, { "clu", "r2", "r0", "" }},
    { 0x24, { "clu", "r2", "r1", "" }},
    { 0x25, { "jal", "r1", "0", "r0" }},
    { 0x26, { "jmp", "0", "r0", "" }},
    { 0x27, { "jmp", "0", "r1", "" }},
    { 0x28, { "jmp", "0", "r2", "" }},
    { 0x29, { "la", "r0", "i24", "" }},
    { 0x2a, { "la", "r1", "i24", "" }},
    { 0x2b, { "la", "r2", "i24", "" }},
    { 0x2c, { "lb", "r0", "o8", "r0" }},
    { 0x2d, { "lb", "r0", "o8", "r1" }},
    { 0x2e, { "lb", "r0", "o8", "r2" }},
    { 0x2f, { "lb", "r0", "o8", "fp" }},
    { 0x30, { "lb", "r1", "o8", "r0" }},
    { 0x31, { "lb", "r1", "o8", "r1" }},
    { 0x32, { "lb", "r1", "o8", "r2" }},
    { 0x33, { "lb", "r1", "o8", "fp" }},
    { 0x34, { "lb", "r2", "o8", "r0" }},
    { 0x35, { "lb", "r2", "o8", "r1" }},
    { 0x36, { "lb", "r2", "o8", "r2" }},
    { 0x37, { "lb", "r2", "o8", "fp" }},
    { 0x38, { "lbu", "r0", "o8", "r0" }},
    { 0x39, { "lbu", "r0", "o8", "r1" }},
    { 0x3a, { "lbu", "r0", "o8", "r2" }},
    { 0x3b, { "lbu", "r0", "o8", "fp" }},
    { 0x3c, { "lbu", "r1", "o8", "r0" }},
    { 0x3d, { "lbu", "r1", "o8", "r1" }},
    { 0x3e, { "lbu", "r1", "o8", "r2" }},
    { 0x3f, { "lbu", "r1", "o8", "fp" }},
    { 0x40, { "lbu", "r2", "o8", "r0" }},
    { 0x41, { "lbu", "r2", "o8", "r1" }},
    { 0x42, { "lbu", "r2", "o8", "r2" }},
    { 0x43, { "lbu", "r2", "o8", "fp" }},
    { 0x44, { "lc", "r0", "i8", "" }},
    { 0x45, { "lc", "r1", "i8", "" }},
    { 0x46, { "lc", "r2", "i8", "" }},
    { 0x47, { "lcu", "r0", "u8", "" }},
    { 0x48, { "lcu", "r1", "u8", "" }},
    { 0x49, { "lcu", "r2", "u8", "" }},
    { 0x4a, { "lw", "r0", "o8", "r0" }},
    { 0x4b, { "lw", "r0", "o8", "r1" }},
    { 0x4c, { "lw", "r0", "o8", "r2" }},
    { 0x4d, { "lw", "r0", "o8", "fp" }},
    { 0x4e, { "lw", "r1", "o8", "r0" }},
    { 0x4f, { "lw", "r1", "o8", "r1" }},
    { 0x50, { "lw", "r1", "o8", "r2" }},
    { 0x51, { "lw", "r1", "o8", "fp" }},
    { 0x52, { "lw", "r2", "o8", "r0" }},
    { 0x53, { "lw", "r2", "o8", "r1" }},
    { 0x54, { "lw", "r2", "o8", "r2" }},
    { 0x55, { "lw", "r2", "o8", "fp" }},
    { 0x56, { "mov", "r0", "r1", "" }},
    { 0x57, { "mov", "r0", "r2", "" }},
    { 0x58, { "add", "r0", "fp", "" }},
    { 0x59, { "mov", "r0", "sp", "" }},
    { 0x5a, { "mov", "r1", "r0", "" }},
    { 0x5b, { "mov", "r1", "r2", "" }},
    { 0x5c, { "add", "r1", "fp", "" }},
    { 0x5d, { "mov", "r1", "sp", "" }},
    { 0x5e, { "mov", "r2", "r0", "" }},
    { 0x5f, { "mov", "r2", "r1", "" }},
    { 0x60, { "add", "r2", "fp", "" }},
    { 0x61, { "mov", "r2", "sp", "" }},
    { 0x62, { "mov", "r0", "c", "" }},
    { 0x63, { "mov", "r1", "c", "" }},
    { 0x64, { "mov", "r2", "c", "" }},
    { 0x65, { "mov", "fp", "sp", "" }},
    { 0x66, { "mov", "sp", "r0", "" }},
    { 0x67, { "mov", "iv", "r0", "" }},
    { 0x68, { "jmp", "0", "ir", "" }},
    { 0x69, { "mov", "sp", "fp", "" }},
    { 0x6a, { "mul", "r0", "r0", "" }},
    { 0x6b, { "mul", "r0", "r1", "" }},
    { 0x6c, { "mul", "r0", "r2", "" }},
    { 0x6d, { "mul", "r1", "r0", "" }},
    { 0x6e, { "mul", "r1", "r1", "" }},
    { 0x6f, { "mul", "r1", "r2", "" }},
    { 0x70, { "mul", "r2", "r0", "" }},
    { 0x71, { "mul", "r2", "r1", "" }},
    { 0x72, { "mul", "r2", "r2", "" }},
    { 0x73, { "or", "r0", "r1", "" }},
    { 0x74, { "or", "r0", "r2", "" }},
    { 0x75, { "or", "r1", "r0", "" }},
    { 0x76, { "or", "r1", "r2", "" }},
    { 0x77, { "or", "r2", "r0", "" }},
    { 0x78, { "or", "r2", "r1", "" }},
    { 0x79, { "pop", "r0", "", "" }},
    { 0x7a, { "pop", "r1", "", "" }},
    { 0x7b, { "pop", "r2", "", "" }},
    { 0x7c, { "pop", "fp", "", "" }},
    { 0x7d, { "push", "r0", "", "" }},
    { 0x7e, { "push", "r1", "", "" }},
    { 0x7f, { "push", "r2", "", "" }},
    { 0x80, { "push", "fp", "", "" }},
    { 0x81, { "sb", "r0", "o8", "r1" }},
    { 0x82, { "sb", "r0", "o8", "r2" }},
    { 0x83, { "sb", "r0", "o8", "fp" }},
    { 0x84, { "sb", "r1", "o8", "r0" }},
    { 0x85, { "sb", "r1", "o8", "r2" }},
    { 0x86, { "sb", "r1", "o8", "fp" }},
    { 0x87, { "sb", "r2", "o8", "r0" }},
    { 0x88, { "sb", "r2", "o8", "r1" }},
    { 0x89, { "sb", "r2", "o8", "fp" }},
    { 0x8a, { "shl", "r0", "r1", "" }},
    { 0x8b, { "shl", "r0", "r2", "" }},
    { 0x8c, { "shl", "r1", "r0", "" }},
    { 0x8d, { "shl", "r1", "r2", "" }},
    { 0x8e, { "shl", "r2", "r0", "" }},
    { 0x8f, { "shl", "r2", "r1", "" }},
    { 0x90, { "sra", "r0", "r1", "" }},
    { 0x91, { "sra", "r0", "r2", "" }},
    { 0x92, { "sra", "r1", "r0", "" }},
    { 0x93, { "sra", "r1", "r2", "" }},
    { 0x94, { "sra", "r2", "r0", "" }},
    { 0x95, { "sra", "r2", "r1", "" }},
    { 0x96, { "srl", "r0", "r1", "" }},
    { 0x97, { "srl", "r0", "r2", "" }},
    { 0x98, { "srl", "r1", "r0", "" }},
    { 0x99, { "srl", "r1", "r2", "" }},
    { 0x9a, { "srl", "r2", "r0", "" }},
    { 0x9b, { "srl", "r2", "r1", "" }},
    { 0x9c, { "sub", "r0", "r1", "" }},
    { 0x9d, { "sub", "r0", "r2", "" }},
    { 0x9e, { "sub", "r1", "r0", "" }},
    { 0x9f, { "sub", "r1", "r2", "" }},
    { 0xa0, { "sub", "r2", "r0", "" }},
    { 0xa1, { "sub", "r2", "r1", "" }},
    { 0xa2, { "sub", "sp", "i24", "" }},
    { 0xa3, { "sw", "r0", "o8", "r0" }},
    { 0xa4, { "sw", "r0", "o8", "r1" }},
    { 0xa5, { "sw", "r0", "o8", "r2" }},
    { 0xa6, { "sw", "r0", "o8", "fp" }},
    { 0xa7, { "sw", "r1", "o8", "r0" }},
    { 0xa8, { "sw", "r1", "o8", "r1" }},
    { 0xa9, { "sw", "r1", "o8", "r2" }},
    { 0xaa, { "sw", "r1", "o8", "fp" }},
    { 0xab, { "sw", "r2", "o8", "r0" }},
    { 0xac, { "sw", "r2", "o8", "r1" }},
    { 0xad, { "sw", "r2", "o8", "r2" }},
    { 0xae, { "sw", "r2", "o8", "fp" }},
    { 0xaf, { "sxt", "r0", "r0", "" }},
    { 0xb0, { "sxt", "r0", "r1", "" }},
    { 0xb1, { "sxt", "r0", "r2", "" }},
    { 0xb2, { "sxt", "r1", "r0", "" }},
    { 0xb3, { "sxt", "r1", "r1", "" }},
    { 0xb4, { "sxt", "r1", "r2", "" }},
    { 0xb5, { "sxt", "r2", "r0", "" }},
    { 0xb6, { "sxt", "r2", "r1", "" }},
    { 0xb7, { "sxt", "r2", "r2", "" }},
    { 0xb8, { "xor", "r0", "r1", "" }},
    { 0xb9, { "xor", "r0", "r2", "" }},
    { 0xba, { "xor", "r1", "r0", "" }},
    { 0xbb, { "xor", "r1", "r2", "" }},
    { 0xbc, { "xor", "r2", "r0", "" }},
    { 0xbd, { "xor", "r2", "r1", "" }},
    { 0xbe, { "zxt", "r0", "r0", "" }},
    { 0xbf, { "zxt", "r0", "r1", "" }},
    { 0xc0, { "zxt", "r0", "r2", "" }},
    { 0xc1, { "zxt", "r1", "r0", "" }},
    { 0xc2, { "zxt", "r1", "r1", "" }},
    { 0xc3, { "zxt", "r1", "r2", "" }},
    { 0xc4, { "zxt", "r2", "r0", "" }},
    { 0xc5, { "zxt", "r2", "r1", "" }},
    { 0xc6, { "zxt", "r2", "r2", "" }},

    { 0xc7, { "jmp", "d24", "", "" }},

    { 0xc8, { "ceq", "r0", "z", "" }},
    { 0xc9, { "ceq", "r1", "z", "" }},
    { 0xca, { "ceq", "r2", "z", "" }},
    { 0xcb, { "cls", "r0", "z", "" }},
    { 0xcc, { "cls", "r1", "z", "" }},
    { 0xcd, { "cls", "r2", "z", "" }},
    { 0xce, { "clu", "z", "r0", "" }},
    { 0xcf, { "clu", "z", "r1", "" }},
    { 0xd0, { "clu", "z", "r2", "" }},

    { 0xd1, { "jal", "r1", "0", "r1" }},
    { 0xd2, { "jal", "r1", "0", "r2" }}
};

/* Disassemble instruction from description in table */
static void disasm(obuf, inst)
char *obuf;
struct insform *inst;
{
    int d, i, j, n, z;
    uint32_t dimm;

    /* Find instruction in table */
    obuf[0] = '\0';
    d = z = 0;
    i = 0;
    n = sizeof(instab)/sizeof(struct insdef);
    while (i < n) {

        /* Skip over directives */
        if (instab[i].code < 0) {
            ++i;
            continue;
        }

        /* Instructions */
        if (instab[i].code == inst->bytes[0]) {
            j = 0;
            while ((j < 4) && instab[i].toks[j][0]) {
                if ((j == 2) && !z) {
                    strcpy(EOBUF(obuf), ",");
                }
                if (d && (j == (d + 1))) {
                    strcpy(EOBUF(obuf), "(");
                }
                if (!strcmp(instab[i].toks[j], "0")) {
                    d = j;
                    z = 1;
                    ++j;
                    continue;
                }
                if ((j && (instab[i].toks[j][0] == 'd')) || \
                    (j && ((instab[i].toks[j][0] == 'i') && \
                           isdigit(instab[i].toks[j][1]))) || \
                    (j && !strcmp(instab[i].toks[j], "o8")) || \
                    (j && (instab[i].toks[j][0] == 'u'))) {
                    if (inst->fixtype) {
                        strcpy(EOBUF(obuf), inst->symref);
                    } else {
                        if (instab[i].toks[j][1] == '8') {
                            d = j;
                            if (instab[i].toks[j][0] == 'u') {
                                dimm = (uint8_t)inst->bytes[1];
                            } else {
                                dimm = (int8_t)inst->bytes[1];
                            }
                        } else {
                            dimm = inst->bytes[1] << 0;
                            dimm += inst->bytes[2] << 8;
                            dimm += inst->bytes[3] << 16;
                            if (instab[i].toks[j][0] == 'i') {
                                dimm = ((int32_t)dimm << 8) >> 8;
                            }
                        }
                        if (strcmp(instab[i].toks[j], "o8") || dimm) {
                            sprintf(EOBUF(obuf), "%d", dimm);
                        }
                    }
                } else {
                    strcpy(EOBUF(obuf), instab[i].toks[j]);
                }
                if (!j) {
                    strcpy(EOBUF(obuf), "\t");
                }
                if (d && (j == (d + 1))) {
                    strcpy(EOBUF(obuf), ")");
                }

                ++j;
            }

            return;
        }

        ++i;
    }

    sprintf(obuf, "? Line %d: unknown instruction code: %02x",
            inst->lineno, inst->bytes[0]);
    error = 1;
}

/* Scan strings for numbers */
static int scani32(s, pi32)
char *s;
int32_t *pi32;
{
    char extra[TOKLEN];

    if (isdigit(s[0]) && (s[strlen(s) - 1] == 'h')) {
        if (!(sscanf(s, "%x", pi32) == 1)) {
            return 0;
        }
    } else {
        if (!(sscanf(s, "%d%s", pi32, &extra[0]) == 1)) {
            return 0;
        }
    }

    return 1;
}
static int scani8(s, pi32)
char *s;
int32_t *pi32;
{
    if (!scani32(s, pi32)) {
        return 0;
    }
    if ((*pi32 < -128) || (127 < *pi32)) {
        return 0;
    }

    return 1;
}
static int scanu8(s, pi32)
char *s;
int32_t *pi32;
{
    if (!scani32(s, pi32)) {
        return 0;
    }
    if ((*pi32 < 0) || (255 < *pi32)) {
        return 0;
    }

    return 1;
}
static int scani24(s, pi32)
char *s;
int32_t *pi32;
{
    if (!scani32(s, pi32)) {
        return 0;
    }
    if ((*pi32 < -(1 << 24)) || !(*pi32 < (1 << 24))) {
        return 0;
    }

    return 1;
}

/* Find a symbol definition */
static struct insform *symfind(name)
char *name;
{
    struct insform *inext;

    inext = ifirst;
    while (inext) {
        if (((inext->fixtype == FIXCOMM) || (inext->fixtype == FIXLABL) || \
             (inext->fixtype == FIXSYMB)) && !strcmp(inext->symref, name)) {
            return inext;
        }
        inext = inext->next;
    }

    return NULL;
}
static int dupsym(name, lineno)
char *name;
uint32_t lineno;
{
    if (symfind(name)) {
        fprintf(stderr, "? Line %d: duplicate symbol definition: %s\n",
                lineno, name);
        error = 1;
        return 1;
    }

    return 0;
}
static struct insform *gblfind(name)
char *name;
{
    struct insform *inext;

    inext = ifirst;
    while (inext) {
        if (((inext->fixtype == FIXCOMM) && \
             !strcmp(inext->symref, name)) || \
            ((inext->fixtype == FIXOTHR) && \
             !strcmp(inext->symref, ".globl") && \
             !strcmp((char *)inext->bytes, name))) {
            return inext;
        }
        inext = inext->next;
    }

    return NULL;
}

/* Assemble instructions, process labels and directives */
static struct insform *findins(lineno)
uint32_t lineno;
{
    int i, j, k, n;
    uint32_t dimm;
    struct insform *svp;

    /* Clear the form, set the section and line numbers */
    memset(tmpf, 0, sizeof(struct insform));
    tmpf->lineno = lineno;
    tmpf->section = secnum;

    /* Is there a comment on the line ? */
    i = 0;
    while ((i < LINSIZ) && linebuf[i] && !(linebuf[i] == '\n')) {
        if (linebuf[i] == ';') {
            break;
        }
        ++i;
    }
    if ((i < LINSIZ) && (linebuf[i] == ';')) {
        tmpf->comment = (char *)malloc(strlen(&linebuf[i]) + 1);
        if (!tmpf->comment) {
            fprintf(stderr, "? Line %d: malloc failed\n", lineno);
            error = 1;
            return NULL;
        }
        strcpy(tmpf->comment, &linebuf[i]);
    }

    /* Full line comment ? */
    if (ntokens < 0) {
        tmpf->fixtype = FIXCMNT;
        return tmpf;
    }

    /* Is it a label or symbol definition ? */
    n = strlen(tokens[0]) - 1; 
    if ((ntokens == 1) && (tokens[0][n] == ':')) {
        tmpf->fixtype = FIXLABL;
        strncpy(tmpf->symref, tokens[0], n);
        tmpf->symref[n] = '\0';
        if (dupsym(tmpf->symref, lineno)) {
            return NULL;
        }
        return tmpf;
    }        
    if ((ntokens == 3) && !isdigit(tokens[0][0]) && isdigit(tokens[2][0]) &&
        (tokens[1][0] == '=') && scani24(tokens[2], &dimm)) {
        if (dupsym(tokens[0], lineno)) {
            return NULL;
        }
        tmpf->fixtype = FIXSYMB;
        strcpy(tmpf->symref, tokens[0]);
        tmpf->symval = dimm;
        return tmpf;
    }

    /* Bump location counter */
    if ((ntokens == 5) && (tokens[0][0] == '.') && (tokens[1][0] == '=') &&
        (tokens[2][0] == '.') && (tokens[3][0] == '+') &&
        scani24(tokens[4], &dimm)) {
        tmpf->fixtype = FIXBUMP;
        tmpf->length = dimm;
        return tmpf;
    }

    /* Look for it in table */
    n = sizeof(instab)/sizeof(struct insdef);
    i = 0;
    while (i < n) {

        /* No match on first token */
        if (strcmp(instab[i].toks[0], tokens[0])) {
            ++i;
            continue;
        }

        /* Directives */
        if (instab[i].code < 0) {

            /* .bss */
            if (!strcmp(tokens[0], ".bss")) {
                tmpf->fixtype = FIXOTHR;
                secnum = 2;
                tmpf->section = -1;
                return tmpf;
            }

            /* .byte */
            if (!strcmp(tokens[0], ".byte")) {
                k = 0;
                while (k < (ntokens - 1)) {
                    if (!scanu8(tokens[k + 1], &dimm)) {
                        break;
                    }
                    tmpf->bytes[k] = (uint8_t)dimm;
                    ++k;
                }
                if (k < (ntokens - 1)) {
                    ++i;
                    continue;
                }
                tmpf->fixtype = FIXBDAT;
                tmpf->length = k;
                return tmpf;
            }

            /* .comm */
            if (!strcmp(tokens[0], ".comm")) {
                if (!(ntokens == 3) || isdigit(tokens[1][0]) ||
                    !scani24(tokens[2], &dimm)) {
                    ++i;
                    continue;
                }
                if ((svp = symfind(tokens[1]))) {
                    if ((svp->fixtype == FIXCOMM) && (svp->length < dimm)) {
                        svp->length = dimm;
                    }
                    return NULL;
                }
                tmpf->fixtype = FIXCOMM;
                tmpf->length = dimm;
                strcpy(tmpf->symref, tokens[1]);
                return tmpf;
            }

            /* .data */
            if (!strcmp(tokens[0], ".data")) {
                tmpf->fixtype = FIXOTHR;
                secnum = 1;
                tmpf->section = -1;
                return tmpf;
            }

            /* .globl */
            if (!strcmp(tokens[0], ".globl")) {
                tmpf->fixtype = FIXOTHR;
                strcpy(tmpf->symref, tokens[0]);
                strcpy((char *)tmpf->bytes, tokens[1]);
                return tmpf;
            }

            /* .text */
            if (!strcmp(tokens[0], ".text")) {
                tmpf->fixtype = FIXOTHR;
                secnum = 0;
                tmpf->section = -1;
                return tmpf;
            }

            /* .word */
            if (!strcmp(tokens[0], ".word")) {
                k = 0;
                while (k < (ntokens - 1)) {
                    if (!scani24(tokens[k + 1], &dimm)) {
                        break;
                    }
                    tmpf->bytes[k*3 + 0] = (uint8_t)(dimm >> 0);
                    tmpf->bytes[k*3 + 1] = (uint8_t)(dimm >> 8);
                    tmpf->bytes[k*3 + 2] = (uint8_t)(dimm >> 16);
                    ++k;
                }
                if (!(k < (ntokens - 1))) {
                    memset(&tmpf->bytes[k*3], 0, sizeof(tmpf->bytes) - k*3);
                    tmpf->fixtype = FIXWDAT;
                    tmpf->length = k*3;
                    return tmpf;
                }

                tmpf->fixtype = FIXDD24;
                tmpf->length = 3;
                strcpy(tmpf->symref, tokens[1]);
                return tmpf;
            }

            tmpf->fixtype = FIXOTHR;
            strcpy(tmpf->symref, tokens[0]);
            return tmpf;
        }
            
        /* Instructions */
        tmpf->bytes[0] = (uint8_t)instab[i].code;
        tmpf->length = 1;
        j = 0;
        while ((j < 4) && instab[i].toks[j][0]) {
            if (!(j < ntokens)) {
                break;
            }

            k = 0;
            if (!strcmp(instab[i].toks[j], "i8")) {
                tmpf->length = 2;
                k = 1;
            }
            if (!strcmp(instab[i].toks[j], "u8")) {
                tmpf->length = 2;
                k = 2;
            }
            if (!strcmp(instab[i].toks[j], "d8")) {
                tmpf->length = 2;
                k = 3;
            }
            if (!strcmp(&instab[i].toks[j][1], "24")) {
                tmpf->length = 4;
                k = 4;
            }
            if (!strcmp(instab[i].toks[j], "o8")) {
                tmpf->length = 2;
                k = 5;
            }

            if (k) {
                if (isdigit(tokens[j][0]) || 
                    (tokens[j][0] == '-') || (tokens[j][0] == '+')) {
                    if ((k == 1) || (k == 3) || (k == 5)) {
                        if (!scani8(tokens[j], &dimm)) {
                            break;
                        }
                    }
                    if (k == 2) {
                        if (!scanu8(tokens[j], &dimm)) {
                            break;
                        }
                    }
                    if (k == 4) {
                        if (!scani24(tokens[j], &dimm)) {
                            break;
                        }
                        tmpf->bytes[2] = (uint8_t)(dimm >> 8);
                        tmpf->bytes[3] = (uint8_t)(dimm >> 16);
                    }
                    tmpf->bytes[1] = (uint8_t)dimm;
                } else {
                    if (!((k == 3) || (k == 4))) {
                        break;
                    }
                    switch (k) {
                        case 3 :
                            tmpf->fixtype = FIXPCD8;
                            break;
                        case 4 :
                            tmpf->fixtype = FIXID24;
                            break;
                    }
                    strcpy(tmpf->symref, tokens[j]);
                }
            } else {
                if (strcmp(tokens[j], instab[i].toks[j])) {
                    break;
                }
            }

            ++j;
        }

        if (!(j < 4) || !instab[i].toks[j][0]) {
            break;
        }

        ++i;
    }

    if (i < n) {
        return tmpf;
    }

    /* Default word data initialization ? */
    if (ntokens == 1) {
        if (scani24(tokens[0], &dimm)) {
            tmpf->fixtype = FIXWDAT;
            tmpf->length = 3;
            tmpf->bytes[0] = (uint8_t)(dimm >> 0);
            tmpf->bytes[1] = (uint8_t)(dimm >> 8);
            tmpf->bytes[2] = (uint8_t)(dimm >> 16);
            return tmpf;
        }
        if (!isdigit(tokens[0][0])) {
            tmpf->fixtype = FIXDD24;
            tmpf->length = 3;
            strcpy(tmpf->symref, tokens[0]);
            return tmpf;
        }
    }

    /* Unknown instruction/directive */
    fprintf(stderr, "? Line %d: unknown instruction/directive: '", lineno);
    i = 0;
    while (i < ntokens) {
        if (i) {
            fputc(' ', stderr);
        }
        fputs(tokens[i], stderr);
        ++i;
    }
    fputs("'\n", stderr);
    error = 1;

    return NULL;
}

/* Scan a line and get all the tokens */
static void scan(lineno)
uint32_t lineno;
{
    char *cp;
    int i, t;

    cp = linebuf;
    ntokens = 0;
    i = t = 0;
    while (*cp && !(*cp == '\n') && !(*cp == ';')) {
        if (!(ntokens < MAXTOKS)) {
            fprintf(stderr, "? Line %d: too many tokens\n", lineno);
            error = 1;
            return;
        }
        if (isspace(*cp) || (*cp == ',') || (*cp == '(') || (*cp == ')')) {
            if (t) {
                tokens[ntokens][i] = '\0';
                i = t = 0;
                ++ntokens;
            } else {
                if (*cp == '(') {
                    tokens[ntokens][0] = '0';
                    i = t = 1;
                    continue;
                }
            }
            ++cp;
            continue; 
        }
        if (!(i < TOKLEN)) {
            tokens[ntokens][TOKLEN] = '\0';
            fprintf(stderr, "? Line %d: token '%s ...' too long\n",
                    lineno, tokens[ntokens]);
            error = 1;
            return;
        }
        t = 1;
        tokens[ntokens][i] = *cp;
        ++i;
        ++cp;
    }
    if (t) {
        tokens[ntokens][i] = '\0';
        ++ntokens;
    }

    if (!ntokens && (*cp == ';')) {
        ntokens = -1;
    }
}

/* Display usage help */
static void usage(name)
char *name;
{
    fprintf(stderr, "usage: %s [-a][-c|-l|-S][-O]\n", name);
}

/* Untabify an output string */
static void untabify(s)
char *s;
{
    char c, t[LINSIZ];
    int i, j, k;

    i = j = k = 0;
    while ((j < (LINSIZ - 1)) && (c = s[i++])) {
        if (c == '\t') {
            do {
                t[j++] = ' ';
                ++k;
            } while (k % 8);
        } else {
            t[j++] = c;
            ++k;
        }
        if (c == '\n') {
            k = 0;
        }
    }
    t[j] = '\0';
    strcpy(s, t);
}

/* Load-and-go script for running program on target */
static void lgoput()
{
    int i;
    struct insform *inext, *s;

    /* Can't emit load script with errors */
    if (error) {
        fputs("? Errors, can't generate load script\n", stderr);
        return;
    }

    /* Dump the bytes in load commands */
    secnum = curloc = 0;
    while (secnum < 3) {
        inext = ifirst;
        while (inext) {
            if (!(inext->section == secnum)) {
                inext = inext->next;
                continue;
            }

            if (!(inext->fixtype < 0) || (inext->fixtype == FIXBDAT) || \
                (inext->fixtype == FIXWDAT)) {
                printf("L%06X", curloc);
                i = 0;
                while (i < inext->length) {
                    printf("%02X", inext->bytes[i]);
                    ++i;
                }
                putchar('\n');
            }
            curloc += inext->length;
            inext = inext->next;
        }

        ++secnum;
    }

    /* No input ? */
    if (!ifirst || !curloc) {
        return;
    }

    /* Go to "start", if defined */
    s = symfind("start");
    if (s) {
        printf("G%06X\n", s->symval);
    }
}

/* Assembly listing */
#define INSSIZ 32
static void lstput()
{
    char dbuf[INSSIZ], o[LINSIZ];
    int i, t;
    struct insform *inext;

    secnum = curloc = 0;
    t = FIXCMNT;
    while (secnum < 3) {
        inext = ifirst;
        while (inext) {
            if (!(inext->section == secnum)) {
                inext = inext->next;
                continue;
            }

            o[0] = '\0';
            switch (inext->fixtype) {
                case FIXBDAT :
                    i = 0;
                    while (i < inext->length) {
                        if (i) {
                            sprintf(EOBUF(o), "\n");
                        }
                        sprintf(EOBUF(o),
                                "%06x %02x", curloc + i, inext->bytes[i]);
                        ++i;
                    }
                    break;
                case FIXWDAT :
                case FIXDD24 :
                    i = 0;
                    while (i < inext->length) {
                        if (i) {
                            sprintf(EOBUF(o), "\n");
                        }
                        sprintf(EOBUF(o),
                                "%06x %02x %02x %02x", curloc + i,
                               inext->bytes[i + 0], inext->bytes[i + 1], \
                               inext->bytes[i + 2]);
                        i += 3;
                    }
                    break;
                case FIXBUMP :
                case FIXCOMM :
                case FIXLABL :
                    if (!(t < 0)) {
                        sprintf(EOBUF(o), "\n");
                    }
                    if ((inext->fixtype == FIXLABL) ||
                        (inext->fixtype == FIXCOMM)) {
                        sprintf(EOBUF(o), "%s:", inext->symref);
                    }
                    if (inext->fixtype == FIXCOMM) {
                        sprintf(EOBUF(o), "\n");
                    }
                    if ((inext->fixtype == FIXCOMM) ||
                        (inext->fixtype == FIXBUMP)) {
                        sprintf(EOBUF(o), "%06x", curloc);
                    }
                    break;
                case FIXSYMB :
                case FIXOTHR :
                case FIXCMNT :
                    break;
                default :
                    if (!(inext->fixtype < 0)) {
                        sprintf(EOBUF(o), "%06x ", curloc);
                        i = 0;
                        while (i < inext->length) {
                            sprintf(EOBUF(o), "%02x ", inext->bytes[i]);
                            ++i;
                        }
                        while (i < 4) {
                            sprintf(EOBUF(o), "   ");
                            ++i;
                        }
                        i = 0;
                        while (i < 5) {
                            sprintf(EOBUF(o), " ");
                            ++i;
                        }
                        disasm(dbuf, inext);
                        sprintf(EOBUF(o), "%s", dbuf);
                    }
                    break;
            }

            untabify(o);
            printf("%s", o);
            if (inext->comment) {
                if (inext->fixtype == FIXCMNT) {
                    if (!((t == FIXCMNT) || (t == FIXLABL) || \
                          (t == FIXBDAT) || (t == FIXDD24) || \
                          (t == FIXWDAT))) {
                        printf("\n");
                    }
                    i = 0;
                    while (i < 24) {
                        printf(" ");
                        ++i;
                    }
                    printf("%s", inext->comment);
                } else {
                    i = strlen(o);
                    while (i < 48) {
                        printf(" ");
                        ++i;
                    }
                    printf("%s", inext->comment);
                }
            } else {
                if (strlen(o)) {
                    printf("\n");
                }
            }

            if (strlen(o) || inext->comment) {
                t = inext->fixtype;
            }
            curloc += inext->length;
            inext = inext->next;
        }

        ++secnum;
    }

    if (ifirst) {
        printf("%06x\n", curloc);
    }
}

/* Output assemble-able source */
static void asmput(afmt)
int afmt;
{
    char dbuf[INSSIZ], o[LINSIZ];
    int i, sp, t;
    int32_t d;
    struct insform *inext;

    secnum = curloc = 0;
    t = FIXCMNT;
    while (secnum < 3) {
        sp = 0;
        inext = ifirst;
        while (inext) {
            if (!(inext->section == secnum)) {
                inext = inext->next;
                continue;
            }

            o[0] = '\0';
            if (!sp && !(inext->fixtype == FIXCMNT)) {
                if (!(t == FIXCMNT)) {
                    sprintf(EOBUF(o), "\n");
                }
                switch (secnum) {
                    case 0 :
                        sprintf(EOBUF(o), "\t.text\n");
                        break;
                    case 1 :
                        sprintf(EOBUF(o), "\t.data\n");
                        break;
                    case 2 :
                        sprintf(EOBUF(o), "\t.bss\n");
                        break;
                }
                sp = 1;
            }

            switch (inext->fixtype) {
                case FIXBDAT :
                    sprintf(EOBUF(o), "\t.byte\t");
                    i = 0;
                    while (i < inext->length) {
                        if (i) {
                            sprintf(EOBUF(o), ",");
                        }
                        sprintf(EOBUF(o), "%d", inext->bytes[i]);
                        ++i;
                    }
                    break;
                case FIXWDAT :
                    sprintf(EOBUF(o), "\t.word\t");
                    i = 0;
                    while (i < inext->length) {
                        if (i) {
                            sprintf(EOBUF(o), ",");
                        }
                        d = (inext->bytes[i + 0] << 0) + \
                            (inext->bytes[i + 1] << 8) + \
                            (inext->bytes[i + 2] << 16);
                        d = (d << 8);
                        d = (d >> 8);
                        sprintf(EOBUF(o), "%d", d);
                        i += 3;
                    }
                    break;
                case FIXDD24 :
                    sprintf(EOBUF(o), "\t.word\t%s", inext->symref);
                    break;
                case FIXBUMP :
                    sprintf(EOBUF(o), "\t. = . + %d", inext->length);
                    break;
                case FIXCOMM :
                    sprintf(EOBUF(o),
                            "\t.comm\t%s,%d", inext->symref, inext->length);
                    break;
                case FIXSYMB :
                    sprintf(EOBUF(o),
                            "\t%s = %d", inext->symref, inext->symval);
                    break;
                case FIXLABL :
                    sprintf(EOBUF(o), "%s:", inext->symref);
                    break;
                case FIXOTHR :
                    if (!strcmp(inext->symref, ".globl")) {
                        sprintf(EOBUF(o),
                                "\n\t%s\t%s", inext->symref, inext->bytes);
                    }
                    break;
                case FIXCMNT :
                    break;
                default :
                    if (!(inext->fixtype < 0)) {
                        sprintf(EOBUF(o), "\t");
                        disasm(dbuf, inext);
                        sprintf(EOBUF(o), "%s", dbuf);
                    }
                    break;
            }

            if (afmt) {
                untabify(o);
            }
            printf("%s", o);
            if (inext->comment) {
                if (inext->fixtype == FIXCMNT) {
                    if (afmt && !((t == FIXCMNT) || (t == FIXLABL))) {
                        printf("\n");
                    }
                    printf("%s", inext->comment);
                } else {
                    i = strlen(o);
                    while (i < 32) {
                        printf(" ");
                        ++i;
                    }
                    printf("%s", inext->comment);
                }
            } else {
                if (strlen(o)) {
                    printf("\n");
                }
            }

            if (strlen(o) || inext->comment) {
                t = inext->fixtype;
            }
            curloc += inext->length;
            inext = inext->next;
        }

        ++secnum;
    }
}

/* Linkable object output */
static void objput()
{
    int i, r;
    struct insform *inext, *svp;

    /* Can't emit linkable object with errors */
    if (error) {
        fputs("? Errors, can't generate linkable object\n", stderr);
        return;
    }

    /* Dump object records */
    inext = ifirst;
    while (inext) {
        switch (inext->fixtype) {
            case FIXLABL :
                r = !gblfind(inext->symref);
                printf("%s %d %s\n", 
                       r ? "D" : "G", inext->section, inext->symref);
                break;
            case FIXCOMM :
                printf("C %d %s %d\n",
                       inext->section, inext->symref, inext->length);
                break;
            case FIXBUMP :
                printf("A %d %d\n", inext->section, inext->length);
                break;
            case FIXDD24 :
            case FIXID24 :
                svp = symfind(inext->symref);
                if (!svp || !(svp->fixtype == FIXSYMB)) {
                    r = svp && !gblfind(inext->symref);
                    printf("%s %d", r ? "R" : "X", inext->section);
                    i = 0;
                    while (i < inext->length) {
                        printf(" %02X", inext->bytes[i]);
                        ++i;
                    }
                    printf(" %s ", inext->symref);
                    printf("%s\n", (inext->fixtype == FIXDD24) ? "0" : "1");
                    break;
                }
            case FIXBDAT :
            case FIXWDAT :
            case FIXINST :
            case FIXPCD8 :
                printf("B %d", inext->section);
                i = 0;
                while (i < inext->length) {
                    printf(" %02X", inext->bytes[i]);
                    ++i;
                }
                putchar('\n');
                break;
            default :
                break;
        }
        inext = inext->next;
    }

    if (ifirst) {
        printf("S -1\n");
    }
}

/* Destination register for MOV to register from C instructions */
static int dstmovc(b)
int b;
{
    switch (b) {
        case MOV_R0_C :
            return 0;
        case MOV_R1_C :
            return 1;
        case MOV_R2_C :
            return 2;
    }

    return -1;
}

/* Destination register for MOV register,register instructions */
static int dstmovr(b)
int b;
{
    switch (b) {
        case MOV_R0_R1 :
        case MOV_R0_R2 :
            return 0;
        case MOV_R1_R0 :
        case MOV_R1_R2 :
            return 1;
        case MOV_R2_R0 :
        case MOV_R2_R1 :
            return 2;
    }

    return -1;
}

/* Source register for compare register with zero instructions */
static int srcreqz(b)
int b;
{
    switch (b) {
        case CEQ_R0_Z :
            return 0;
        case CEQ_R1_Z :
            return 1;
        case CEQ_R2_Z :
            return 2;
    }

    return -1;
}
static int srczulr(b)
int b;
{
    switch (b) {
        case CLU_Z_R0 :
            return 0;
        case CLU_Z_R1 :
            return 1;
        case CLU_Z_R2 :
            return 2;
    }

    return -1;
}

/* Source register for MOV register,register instructions */
static int srcmovr(b)
int b;
{
    switch (b) {
        case MOV_R1_R0 :
        case MOV_R2_R0 :
            return 0;
        case MOV_R0_R1 :
        case MOV_R2_R1 :
            return 1;
        case MOV_R0_R2 :
        case MOV_R1_R2 :
            return 2;
    }

    return -1;
}

/* Make the desired compare register with zero instruction */
static int makceq(src)
int src;
{
    int i;

    i = 0;
    while (i < (CEQ_R2_Z - CEQ_R0_Z + 1)) {
        if (srcreqz(CEQ_R0_Z + i) == src) {
            return (CEQ_R0_Z + i);
        }
        ++i;
    }

    return -1;
}

/* Make the desired move from C instruction */
static int makmovc(dst)
int dst;
{
    int i;

    i = 0;
    while (i < (MOV_R2_C - MOV_R0_C + 1)) {
        if (dstmovc(MOV_R0_C + i) == dst) {
            return (MOV_R0_C + i);
        }
        ++i;
    }

    return -1;
}

/* Delete, insert an instruction form */
static void delinst(ilast, inext)
struct insform *ilast, *inext;
{
    if (inext == ifirst) {
        ifirst = inext->next;
    } else  {
        ilast->next = inext->next;
    }
    free(inext);
}
static struct insform *insinst(ilast, inext)
struct insform *ilast, *inext;
{
    struct insform *inew;

    inew = (struct insform *)malloc(sizeof(struct insform));
    if (!inew) {
        fprintf(stderr, "? malloc failed in insinst\n");
        return NULL;
    }

    memset(inew, 0, sizeof(struct insform));
    inew->next = inext;
    if (inext == ifirst) {
        ifirst = inew;
    } else  {
        ilast->next = inew;
    }

    return inew;
}

/* Fix "a branch too far" */
static int fixbra(ibra)
struct insform *ibra;
{
    struct insform *ilabel, *ijump;

    /* If it's an unconditional branch, replace it with a jump */
    if (ibra->bytes[0] == BRA) {
        ibra->bytes[0] = JMPA;
        ibra->bytes[1] = ibra->bytes[2] = ibra->bytes[3] = 0;
        ibra->fixtype = FIXID24;
        ibra->length = 4;
        return 1;
    }

    /* Conditional - reverse the sense, branch around */
    /* a jump to the target of the original condition */

    /* Insert a label after the current branch */
    ilabel = insinst(ibra, ibra->next);
    if (!ilabel) {
        return 0;
    }
    ilabel->fixtype = FIXLABL;
    sprintf(ilabel->symref, "L%d", labeln++);

    /* Then insert a jump before the label */
    ijump = insinst(ibra, ilabel);
    if (!ijump) {
        return 0;
    }
    strcpy(ijump->symref, ibra->symref);
    ijump->bytes[0] = JMPA;
    ijump->bytes[1] = ibra->bytes[2] = ibra->bytes[3] = 0;
    ijump->fixtype = FIXID24;
    ijump->length = 4;

    /* Now change the sense and the target of the original branch */
    ibra->bytes[0] = (ibra->bytes[0] == BRT) ? BRF : BRT;
    strcpy(ibra->symref, ilabel->symref);

    return 1;
}

int main(argc, argv)
int argc;
char *argv[];
{
    int e, afmt, asmout, i, dst, dst2, lstout, objout, optout, src, src2;
    int32_t d;
    uint32_t l;
    struct insform *inext, *ilast, *svp;

    /* Get options */
    afmt = 1;
    asmout = lstout = objout = optout = 0;
    if (!(argc < 5)) {
        usage(argv[0]);
        return 1;
    }
    i = 1;
    while (i < argc) {
        if (!strcmp(argv[i], "-a")) {
            afmt = 0;
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-c")) {
            objout = 1;
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-l")) {
            lstout = 1;
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-O")) {
            optout = 1;
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-S")) {
            asmout = 1;
            ++i;
            continue;
        }

        break;
    }
    if ((i < argc) || \
        (asmout && lstout) || (asmout && objout) || (objout && lstout)) {
        usage(argv[0]);
        return 1;
    }
    
    /* First, read the text */
    error = 0;
    labeln = LBLSTRT;
    ifirst = ilast = NULL;
    secnum = 0;
    l = 0;
    while (fgets(linebuf, LINSIZ, stdin)) {

        /* Scan to get tokens */
        scan(++l);
        if (!ntokens) {
            continue;
        }

        /* Find the instruction or directive */
        if (!findins(l)) {
            continue;
        }

        /* Allocate and fill an instruction form */
        inext = (struct insform *)malloc(sizeof(struct insform));
        if (!inext) {
            fprintf(stderr, "? Line %d: malloc failed\n", l);
            error = 1;
            break;
        }
        memcpy((char *)inext, (char *)tmpf, sizeof(struct insform));
        if (!ifirst) {
            ifirst = inext;
        } else {
            ilast->next = inext;
        }
        ilast = inext;
        inext = inext->next;
    }

    /* Now, process the forms */
again :
    /* First, re-set symbol values, clear reference counts */
    secnum = curloc = 0;
    while (secnum < 3) {
        inext = ifirst;
        while (inext) {
            if (!(inext->section == secnum)) {
                inext = inext->next;
                continue;
            }

            if ((inext->fixtype == FIXCOMM) || (inext->fixtype == FIXLABL)) {
                inext->symval = curloc;
                inext->refcnt = 0;
            }
            if (inext->fixtype == FIXSYMB) {
                inext->refcnt = 0;
            }

            curloc += inext->length;
            inext = inext->next;
        }

        ++secnum;
    }

    /* Then, fix up the symbol references */
    secnum = curloc = 0;
    while (secnum < 3) {
        ilast = inext = ifirst;
        while (inext) {
            if (!(inext->section == secnum)) {
                ilast = inext;
                inext = inext->next;
                continue;
            }

            /* A reference, rather than a definition */
            if (0 < inext->fixtype) {
                if (!(svp = symfind(inext->symref))) {
                    if ((inext->fixtype == FIXPCD8) || \
                        !(asmout || lstout || objout)) {
                        fprintf(stderr,
                            "? Line %d: unresolved symbol: %s\n",
                            inext->lineno, inext->symref);
                        error = 1;
                    }
                } else {

                    /* Note reference */
                    svp->refcnt = 1;

                    /* PC-relative branch to label ? */
                    if (inext->fixtype == FIXPCD8) {

                        /* Symbol not a label ? */
                        e = 0;
                        if (!(svp->fixtype == FIXLABL)) {
                            e = 1;
                        }

                        /* Label too far away ? */
                        d = svp->symval - (curloc + 4);
                        if ((d < -128) || !(d < 128)) {
                            if (fixbra(inext)) {
                                goto again;
                            } else {
                                /* Didn't fix it */
                                fprintf(stderr,
                                        "? Line %d: label too far for branch\n",
                                        inext->lineno);
                                e = 1;
                            }
                        }

                        /* Skip the rest on error */
                        if (e) {
                            error = e;
                        } else {

                            /* Branch to next instruction ? */
                            /* Delete if -O specified */
                            if ((d == -2) && optout) {
                                delinst(ilast, inext);
                                goto again;
                            }

                            /* Fill in displacement */
                            inext->bytes[1] = (uint8_t)d;
                        }
                    }

                    /* Word value */
                    d = -1;
                    if (inext->fixtype == FIXDD24) {
                        d = 0;
                    }
                    if (inext->fixtype == FIXID24) {
                        d = 1;
                    }
                    if (!(d < 0) && (!objout || (svp->fixtype == FIXSYMB))) {
                        inext->bytes[d + 0] = (uint8_t)(svp->symval >> 0);
                        inext->bytes[d + 1] = (uint8_t)(svp->symval >> 8);
                        inext->bytes[d + 2] = (uint8_t)(svp->symval >> 16);
                    }
                }
            }

            curloc += inext->length;
            ilast = inext;
            inext = inext->next;
        }

        ++secnum;
    }

    /* Other -O improvements */
    if (optout) {

        /* Change sub sp,nnnnnn to add sp,-nn if possible */
        ilast = inext = ifirst;
        while (inext) {
            if (!(inext->fixtype < 0) && (inext->bytes[0] == SUBSP)) {
                if ((inext->bytes[1] < 129) && !inext->bytes[2] && \
                    !inext->bytes[3]) {
                    inext->bytes[0] = ADDSP;
                    inext->bytes[1] = -(int8_t)inext->bytes[1];
                    inext->length = 2;
                    inext->bytes[2] = inext->bytes[3] = 0;
                    inext->fixtype = 0;
                    inext->symref[0] = '\0';
                    inext->symval = 0;
                    goto again;
                }
            }
            ilast = inext;
            inext = inext->next;
        }

        /* Turn a pair of adds to SP, into one, if possible */
        ilast = inext = ifirst;
        while (inext) {
            if ((ilast->fixtype == FIXINST) && (inext->fixtype == FIXINST) && \
                (ilast->bytes[0] == ADDSP) && (inext->bytes[0] == ADDSP)) {
                d = (int8_t)ilast->bytes[1] + (int8_t)inext->bytes[1];
                if ((d < 128) && !(d < -128)) {
                    ilast->bytes[1] = (uint8_t)d;
                    delinst(ilast, inext);
                    goto again;
                }
            }
            ilast = inext;
            inext = inext->next;
        }

        /* Delete add sp,0 */
        ilast = inext = ifirst;
        while (inext) {
            if (!(inext->fixtype < 0) && (inext->bytes[0] == ADDSP)) {
                if (!inext->bytes[1]) {
                    delinst(ilast, inext);
                    goto again;
                }
            }
            ilast = inext;
            inext = inext->next;
        }

        /* Add to SP followed by mov sp,fp - skip the add */
        ilast = inext = ifirst;
        while (inext) {
            if ((ilast->fixtype == FIXINST) && (inext->fixtype == FIXINST) && \
                (ilast->bytes[0] == ADDSP) && (inext->bytes[0] == MOVSPFP)) {
                ilast->bytes[0] = MOVSPFP;
                ilast->length = 1;
                delinst(ilast, inext);
                goto again;
            }
            ilast = inext;
            inext = inext->next;
        }

        /* Conditional branch around unconditional branch ... */
        ilast = inext = ifirst;
        while (inext) {

            /* ... becomes conditional branch with reversed sense */
            if ((ilast->fixtype == FIXPCD8) && \
                (inext->fixtype == FIXPCD8) && !ilast->bytes[1] && \
                (((ilast->bytes[0] == BRF) && (inext->bytes[0] == BRA)) || \
                 ((ilast->bytes[0] == BRT) && (inext->bytes[0] == BRA)))) {
                ilast->bytes[0] = (ilast->bytes[0] == BRT) ? BRF : BRT;
                strcpy(ilast->symref, inext->symref);
                delinst(ilast, inext);
                goto again;
            }
            ilast = inext;
            inext = inext->next;
        }

        /* Branch to unconditional branch ... */
        inext = ifirst;
        while (inext) {

            /* ... becomes branch to second branch's target */
            if ((inext->fixtype == FIXPCD8) && \
                ((inext->bytes[0] == BRA) || (inext->bytes[0] == BRF) || \
                 (inext->bytes[0] == BRT))) {
                ilast = ifirst;
                while (ilast) {
                    if ((ilast->fixtype == FIXLABL) && \
                        !strcmp(inext->symref, ilast->symref)) {
                        while (ilast && ilast->next && \
                               (ilast->next->fixtype == FIXCMNT)) {
                            ilast = ilast->next;
                        }
                        if (ilast && ilast->next && \
                            ((ilast->next->fixtype == FIXPCD8) && \
                             (ilast->next->bytes[0] == BRA))) {

                            /* But only if something would change */
                            if (strcmp(inext->symref, ilast->next->symref)) {
                                strcpy(inext->symref, ilast->next->symref);
                                goto again;
                            }
                        }
                    }
                    ilast = ilast->next;
                }
            }
            inext = inext->next;
        }

        /* Remove instructions in the shadow of an unconditional branch */
        ilast = inext = ifirst;
        while (inext) {
            if (!(ilast->fixtype < 0) && \
                (!(inext->fixtype < 0) || (inext->fixtype == FIXCMNT)) \
                && (ilast->bytes[0] == BRA)) {
                delinst(ilast, inext);
                goto again;
            }
            ilast = inext;
            inext = inext->next;
        }

        /* Remove unreferenced local labels and symbols */
        ilast = inext = ifirst;
        while (inext) {
            if (((inext->fixtype == FIXLABL) && (inext->symref[0] == 'L')) || \
                (inext->fixtype == FIXSYMB)) {
                if (!inext->refcnt) {
                    delinst(ilast, inext);
                    goto again;
                }
            }
            ilast = inext;
            inext = inext->next;
        }

        /* Change nonzero test, complement C, to just zero test */
        ilast = inext = ifirst;
        while (inext) {
            if ((ilast->fixtype == FIXINST) && (inext->fixtype == FIXINST)) {
                src = srczulr(ilast->bytes[0]);
                dst = dstmovc(inext->bytes[0]);
                if ((inext->next && (inext->next->fixtype == FIXINST) && \
                     inext->next->next) && \
                    (inext->next->next->fixtype == FIXINST)) {
                    src2 = srcreqz(inext->next->bytes[0]);
                    dst2 = dstmovc(inext->next->next->bytes[0]);
                } else {
                    src2 = dst2 = -1;
                }
                if (!(dst < 0) && !(dst2 < 0) && !(src < 0) && \
                    !(src2 < 0) && (dst == src2) && (dst == dst2)) {
                    ilast->bytes[0] = makceq(src);
                    delinst(inext, inext->next);
                    delinst(ilast, inext);
                    goto again;
                }
            }
            ilast = inext;
            inext = inext->next;
        }

        /* After move to register from C, delete sign or zero extension */
        ilast = inext = ifirst;
        while (inext) {
            if ((ilast->fixtype == FIXINST) && (inext->fixtype == FIXINST)) {
                switch (ilast->bytes[0]) {
                    case MOV_R0_C :
                        if ((inext->bytes[0] == SXT_R0_R0) || \
                            (inext->bytes[0] == ZXT_R0_R0)) {
                            delinst(ilast, inext);
                            goto again;
                        }
                        break;
                    case MOV_R1_C :
                        if ((inext->bytes[0] == SXT_R1_R1) || \
                            (inext->bytes[0] == ZXT_R1_R1)) {
                            delinst(ilast, inext);
                            goto again;
                        }
                        break;
                    case MOV_R2_C :
                        if ((inext->bytes[0] == SXT_R2_R2) || \
                            (inext->bytes[0] == ZXT_R2_R2)) {
                            delinst(ilast, inext);
                            goto again;
                        }
                        break;
                }
            }
            ilast = inext;
            inext = inext->next;
        }

        /* Change move to src from C, move dst,src, to move to dst from C */
        ilast = inext = ifirst;
        while (inext) {
            if ((ilast->fixtype == FIXINST) && (inext->fixtype == FIXINST)) {
                dst = dstmovc(ilast->bytes[0]);
                src = srcmovr(inext->bytes[0]);
                if (!(dst < 0) && !(src < 0) && (dst == src)) {
                    dst = dstmovr(inext->bytes[0]);
                    ilast->bytes[0] = makmovc(dst);
                    ilast->length = 1;
                    delinst(ilast, inext);
                    goto again;
                }
            }
            ilast = inext;
            inext = inext->next;
        }
    }

    /* Put out finished code */
    if (lstout) {
        lstput();
    } else {
        if (objout) {
            objput();
        } else {
            if (asmout) {
                asmput(afmt);
            } else {
                lgoput();
            }
        }
    }

    /* Release instruction/data forms */
    while (ifirst) {
        if (ifirst->comment) {
            free(ifirst->comment);
        }
        inext = ifirst->next;
        free(ifirst);
        ifirst = inext;
    }

    return error;
}
