/* COR24 linker */
#include <ctype.h>
#include <memory.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Object record types */
#define OBJBUMP 1
#define OBJDATA 2
#define OBJIREF 3
#define OBJXREF 4
#define OBJCDEF 5
#define OBJIDEF 6
#define OBJXDEF 7
#define OBJSEPF 8
#define OBJCMNT 9

/* Error */
static int error;

/* Clear uninitialized data, relocatable, strip symbols */
static int clunid, reloc, strip;

/* Line buffer */
#define LINSIZ 80
static char linebuf[LINSIZ];

/* Location counter */
static uint32_t atloc, curloc;

/* Tokens */
#define MAXTOKS 32
#define TOKLEN 64
static int ntokens;
static char tokens[MAXTOKS][TOKLEN + 1];

/* Object records */
struct objrec {
    int8_t rectype;
    int8_t section;
    uint8_t bytes[24];
    char symref[TOKLEN + 1];
    uint32_t length;
    uint32_t symval;
    char *comment;
    struct objrec *next;
};
static struct objrec tmprec;
static struct objrec *tmpo = &tmprec;
static struct objrec *ofirst;

/* Find a symbol definition */
static struct objrec *symfind(name)
char *name;
{
    struct objrec *onext;

    onext = ofirst;
    while (onext) {
        if (((onext->rectype == OBJCDEF) || (onext->rectype == OBJIDEF) || \
             (onext->rectype == OBJXDEF)) && !strcmp(onext->symref, name)) {
            return onext;
        }
        onext = onext->next;
    }

    return NULL;
}

/* Parse an object record */
static struct objrec *parsobj(lineno)
uint32_t lineno;
{
    int i, nbyts;
    int32_t d, g;
    struct objrec *svp;

    /* Clear the record */
    memset(tmpo, 0, sizeof(struct objrec));

    /* Comment ? */
    if (ntokens < 0) {
        tmpo->comment = (char *)malloc(strlen(linebuf) + 1);
        if (!tmpo->comment) {
            fprintf(stderr, "? Line %d: malloc failed\n", lineno);
            error = 1;
            return NULL;
        }
        tmpo->rectype = OBJCMNT;
        tmpo->section = 2;
        strcpy(tmpo->comment, linebuf);
        return tmpo;
    }

    /* Get the section number */
    if ((ntokens < 2) || !(sscanf(tokens[1], "%d", &d) == 1)) {
        error = 1;
    } else {

        /* Decode object records */
        g = 0;
        tmpo->section = d;
        nbyts = ntokens - 2;
        switch (tokens[0][0]) {
            case 'A' :
                if ((ntokens < 3) || !(sscanf(tokens[2], "%d", &d) == 1) || 
                    (d < 0)) {
                    break;
                }
                tmpo->rectype = OBJBUMP;
                tmpo->length = d;
                return tmpo;
            case 'R' :
            case 'X' :
                nbyts -= 2;
            case 'B' :
                if (!(0 < nbyts)) {
                    break;
                }
                i = 0;
                while (i < nbyts) {
                    if (!(sscanf(tokens[i + 2], "%02X", &d) == 1)) {
                        break;
                    }
                    tmpo->bytes[i] = (uint8_t)d;
                    ++i;
                }
                if (i < nbyts) {
                    break;
                }
                tmpo->length = i;
                if (tokens[0][0] == 'B') {
                    tmpo->rectype = OBJDATA;
                } else {
                    strcpy(tmpo->symref, tokens[i + 2]);
                    tmpo->rectype = (tokens[0][0] == 'R') ? OBJIREF : OBJXREF;
                    if (!sscanf(tokens[i + 3], "%d",
                                (int *)&tmpo->bytes[tmpo->length]) == 1) {
                        break;
                    }
                }
                return tmpo;
            case 'C' :
                if ((ntokens < 4) || !(sscanf(tokens[3], "%d", &d) == 1)) {
                    break;
                }
                if ((svp = symfind(tokens[2]))) {
                    if ((svp->rectype == OBJCDEF) && (svp->length < d)) {
                        svp->length = d;
                    }
                    return NULL;
                }
                tmpo->rectype = OBJCDEF;
                tmpo->length = d;
                strcpy(tmpo->symref, tokens[2]);
                return tmpo;
            case 'D' :
                ++g;
            case 'G' :
                if (ntokens < 3) {
                    break;
                }
                if (!g && symfind(tokens[2])) {
                    fprintf(stderr, "? Duplicate global symbol: %s\n",
                            tokens[2]);
                    error = 1;
                    return NULL;
                }
                tmpo->rectype = g ? OBJIDEF : OBJXDEF;
                strcpy(tmpo->symref, tokens[2]);
                return tmpo;
            case 'S' :
                tmpo->rectype = OBJSEPF;
                return tmpo;
            case 'U' :
                return NULL;
            default :
                break;
        }
    }

    /* Unknown record type */
    fprintf(stderr, "? Line %d: unknown object record: '", lineno);
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
        if (isspace(*cp)) {
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
    fprintf(stderr, "Usage: %s [-a][-C <msg>][-r|-s][-x]\n", name);
}

/* Relocatable (re-linkable) object output */
static void objput()
{
    int i, r;
    struct objrec *onext;

    /* Can't emit linkable object with errors */
    if (error) {
        fputs("? Errors, can't generate linkable object\n", stderr);
        return;
    }

    /* Dump object records */
    onext = ofirst;
    while (onext) {

        r = 0;
        switch (onext->rectype) {
            case OBJBUMP :
                printf("A %d %d\n", onext->section, onext->length);
                break;
            case OBJDATA :
                printf("B %d", onext->section);
                i = 0;
                while (i < onext->length) {
                    printf(" %02X", onext->bytes[i]);
                    ++i;
                }
                putchar('\n');
                break;
            case OBJIREF :
                ++r;
            case OBJXREF :
                if (!r && !symfind(onext->symref)) {
                    printf("U -1 %s\n", onext->symref);
                }
                printf("%s %d", r ? "R" : "X", onext->section);
                i = 0;
                while (i < onext->length) {
                    printf(" %02X", onext->bytes[i]);
                    ++i;
                }
                printf(" %s ", onext->symref);
                printf("%d\n", onext->bytes[onext->length]);
                break;
            case OBJCDEF :
                printf("C %d %s %d\n",
                       onext->section, onext->symref, onext->length);
                break;
            case OBJIDEF :
                ++r;
            case OBJXDEF :
                printf("%s %d %s\n", r ? "D" : "G",
                       onext->section, onext->symref);
                break;
            default :
                break;
        }

        onext = onext->next;
    }

    if (ofirst) {
        printf("S -1\n");
    }
}

/* Load-and-go script for running program on target */
static void lgoput(msg)
char *msg;
{
    char c;
    int g, i, secsloc[3], secnum;
    struct objrec *onext, *s;

    /* Can't emit load script with errors */
    if (error) {
        fputs("? Errors, can't generate load script\n", stderr);
        return;
    }

    /* Dump the bytes in load commands */
    secsloc[0] = secsloc[1] = secsloc[2] = -1;
    curloc = atloc;
    secnum = 0;
    while (secnum < 3) {
        onext = ofirst;
        while (onext) {
            if (!(onext->section == secnum)) {
                onext = onext->next;
                continue;
            }

            /* Note section starts */
            if (secsloc[secnum] < 0) {
                secsloc[secnum] = curloc;
                if (!strip) {
                    switch (secnum) {
                        case 0:
                            printf("; TEXT\n");
                            break;
                        case 1:
                            printf("; DATA\n");
                            break;
                        case 2:
                            printf("; BSS\n");
                            break;
                    }
                }
            }

            g = 0;
            switch (onext->rectype) {
                case OBJIREF :
                case OBJXREF :
                    if (!strip) {
                        printf("; %s %s\n",
                               (onext->rectype == OBJIREF) ? "R" : "X",
                               onext->symref);
                    }
                case OBJDATA :
                    printf("L%06X", curloc);
                    i = 0;
                    while (i < onext->length) {
                        printf("%02X", onext->bytes[i]);
                        ++i;
                    }
                    putchar('\n');
                    break;
                case OBJCDEF :
                    ++g;
                case OBJIDEF :
                    ++g;
                case OBJXDEF :
                    if (!strip) {
                        switch (g) {
                            case 0 :
                                c = 'G';
                                break;
                            case 1 :
                                c = 'D';
                                break;
                            case 2 :
                                c = 'C';
                                break;
                        }
                        printf("; %c %s %06X\n", c, onext->symref, curloc);
                    }

                    /* Clear common block */
                    if ((g == 2) && clunid) {
                        i = 0;
                        while (i < onext->length) {
                            printf("L%06X", curloc + i);
                            printf("%02X\n", 0);
                            ++i;
                        }
                    }
                    break;
                case OBJCMNT :
                    printf("%s", onext->comment);
                    break;
                case OBJBUMP :
                    break;
                default :
                    break;
            }

            curloc += onext->length;
            onext = onext->next;
        }

        ++secnum;
    }

    /* Load BSS with zeroes */
    if (clunid) {
        i = secsloc[2];
        while (i < curloc) {
            printf("L%06X", i);
            printf("%02X\n", 0);
            ++i;
        }
    }

    /* Insert comment if supplied */
    if (strlen(msg)) {
        printf("; %s\n", msg);
    }

    /* No input ? */
    if (!ofirst || !curloc) {
        return;
    }

    /* Go to "start", if defined */
    s = symfind("start");
    if (s) {
        printf("G%06X\n", s->symval);
    }
}

int main(argc, argv)
int argc;
char *argv[];
{
    char msg[LINSIZ];
    int d, fno, i, l, secnum;
    struct objrec *olast, *onext, *svp;

    /* Get options */
    msg[0] = '\0';
    clunid = 1;
    reloc = strip = 0;
    i = 1;
    while (i < argc) {
        if (!strcmp(argv[i], "-a")) {
            ++i;
            if (!(i < argc) || !(sscanf(argv[i], "%x", &atloc) == 1)) {
                usage(argv[0]);
                return 1;
            }
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-C")) {
            ++i;
            if (!(i < argc)) {
                usage(argv[0]);
                return 1;
            }
            strcpy(msg, argv[i]);
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-r")) {
            reloc = 1;
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-s")) {
            strip = 1;
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-x")) {
            clunid = 0;
            ++i;
            continue;
        }

        break;
    }
    if ((i < argc) || (reloc && strip)) {
        usage(argv[0]);
        return 1;
    }
    
    /* First, read the text */
    error = l = 0;
    ofirst = olast = NULL;
    while (fgets(linebuf, LINSIZ, stdin)) {

        /* Scan to get tokens */
        scan(++l);
        if (!ntokens) {
            continue;
        }

        /* Parse the input object record */
        if (!parsobj(l)) {
            continue;
        }

        /* Allocate and fill an object record */
        onext = (struct objrec *)malloc(sizeof(struct objrec));
        if (!onext) {
            fprintf(stderr, "? Line %d: malloc failed\n", l);
            error = 1;
            break;
        }
        memcpy((char *)onext, (char *)tmpo, sizeof(struct objrec));
        if (!ofirst) {
            ofirst = onext;
        } else {
            olast->next = onext;
        }
        olast = onext;
        onext = onext->next;
    }

    /* Now, process the records */

    /* First, make internal symbol definitions and references unique */
    fno = 0;
    onext = ofirst;
    while (onext) {

        /* Internal symbols are suffixed with file numbers */
        if (onext->rectype == OBJSEPF) {
            ++fno;
        }
        if ((onext->rectype == OBJIDEF) || (onext->rectype == OBJIREF)) {
            sprintf(&onext->symref[strlen(onext->symref)], "_%d", fno);
        }

        onext = onext->next;
    }

    /* Next, set symbol values */
    curloc = atloc;
    secnum = 0;
    while (!reloc && (secnum < 3)) {
        onext = ofirst;
        while (onext) {
            if (!(onext->section == secnum)) {
                onext = onext->next;
                continue;
            }
            if ((onext->rectype == OBJCDEF) || (onext->rectype == OBJIDEF) || \
                (onext->rectype == OBJXDEF)) {
                onext->symval = curloc;
            }

            curloc += onext->length;
            onext = onext->next;
        }

        ++secnum;
    }

    /* Then, fix up the symbol references */
    secnum = 0;
    while (!reloc && (secnum < 3)) {
        onext = ofirst;
        while (onext) {
            if (!(onext->section == secnum)) {
                onext = onext->next;
                continue;
            }
            if (!(onext->rectype == OBJIREF) && !(onext->rectype == OBJXREF)) {
                onext = onext->next;
                continue;
            }

            /* Unresolved ? */
            if (!(svp = symfind(onext->symref))) {
                fprintf(stderr, "? Unresolved symbol: %s\n", onext->symref);
                error = 1;
            } else {
                /* Found it */
                d = onext->bytes[onext->length];
                onext->bytes[d + 0] = (uint8_t)(svp->symval >> 0);
                onext->bytes[d + 1] = (uint8_t)(svp->symval >> 8);
                onext->bytes[d + 2] = (uint8_t)(svp->symval >> 16);
            }

            onext = onext->next;
        }

        ++secnum;
    }

    /* Put out relinkable or loadable code */
    if (reloc) {
        objput();
    } else {
        lgoput(msg);
    }

    /* Release object records */
    while (ofirst) {
        onext = ofirst->next;
        if (ofirst->rectype == OBJCMNT) {
            free(ofirst->comment);
        }
        free(ofirst);
        ofirst = onext;
    }

    return error;
}
