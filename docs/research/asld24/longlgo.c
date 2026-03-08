/* Longer "L" lines in lgo load-and-go script. Also -m for memory file. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Line buffers */
#define MAXLTTY 80
#define MAXLMEM 40
#define LINSIZ (MAXLTTY + 1)
static char lin0buf [LINSIZ];
static char lin1buf [LINSIZ];

int main(argc, argv)
int argc;
char *argv[];
{
    char *cp, *first, *second, *t, tbuf[7];
    int done, flen, i, llen, m, mlen, n;
    uint32_t a, faddr, saddr;

    /* Get options */
    a = flen = m = mlen = 0;
    i = 1;
    while (i < argc) {
        if (!strcmp(argv[i], "-a")) {
            ++i;
            if (!(i < argc) || !(sscanf(argv[i], "%x", &a) == 1)) {
                --i;
                break;
            }
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-l")) {
            ++i;
            if (!(i < argc) || !(sscanf(argv[i], "%x", &flen) == 1)) {
                --i;
                break;
            }
            ++i;
            continue;
        }
        if (!strcmp(argv[i], "-m")) {
            m = 1;
            ++i;
            continue;
        }
        break;
    }
    if (i < argc) {
        fprintf(stderr, "Bad option: %s\n", argv[i]);
        return 1;
    }

    /* Get first line */
    llen = !m ? MAXLTTY : MAXLMEM;
    first = lin0buf;
    second = lin1buf;
    done = !fgets(first, LINSIZ, stdin);

    /* Try to consolidate successive "L" lines */
    while (!done) {

        /* Get second line */
        if (!fgets(second, LINSIZ, stdin)) {
            done = 1;
        }

        /* First and second both "L", loading contiguous memory ? */
        if (!done && (first[0] == 'L') && (second[0] == 'L') && \
            (sscanf(&first[1], "%06X", &faddr) == 1) && \
            (sscanf(&second[1], "%06X", &saddr) == 1)) {

            /* Count nybbles loaded in first line */
            n = 0;
            cp = &first[7];
            while (*cp && !(*cp == '\n')) {
                ++cp;
                ++n;
            }
            if (n % 2) {
                fprintf(stderr, "Odd number of nybbles on L line: %d\n", n);
                exit(1);
            }

            /* Contiguous ? */
            if ((n/2) == (saddr - faddr)) {

                /* Append data from the second line to the first, */
                /* up to maximum line length, including a newline */
                n = strlen(first) - 1;
                cp = &second[7];
                while (*cp && !(*cp == '\n') && (n < (llen - 1))) {
                    first[n] = *cp;
                    saddr += (n % 2);
                    ++cp;
                    ++n;
                }
                strcpy(&first[n], "\n");

                /* Adjust address and shift characters in line 2 */
                if (*cp && !(*cp == '\n')) {
                    sprintf(tbuf, "%06X", saddr);
                    strncpy(&second[1], tbuf, 6);
                    strcpy(&second[7], cp);
                } else {
                    continue;
                }
            }
        }

        /* Output first line ... */
        if (!m) {
            fputs(first, stdout);
        } else {

            /* MEM file format nnnn:xx xx ... */
            i = 0;
            while (i < 6) {
                tbuf[i] = first[i + 1];
                ++i;
            }
            tbuf[i] = '\0';
            if (!(sscanf(tbuf, "%x", &faddr) == 1)) {
                fprintf(stderr, "Bad hex number\n");
                return 1;
            } else {
                faddr -= a;
            }
            i = 0;
            n = first[0] = 'L' ? (strlen(first) - 8)/2 : 0;
            while (i < n) {
                while (mlen < faddr) {
                    if (!(mlen % 16)) {
                        if (mlen) {
                            printf("\n");
                        }
                        printf("%04X:", mlen);
                    } else {
                        printf(" ");
                    }
                    printf("FF");
                    ++mlen;
                }
                if (!((faddr + i) % 16)) {
                    if (mlen) {
                        printf("\n");
                    }
                    printf("%04X:", faddr + i);
                } else {
                    printf(" ");
                }
                printf("%c%c", first[7 + i*2 + 0], first[7 + i*2 + 1]);
                ++mlen;
                ++i;
            }
            if (done && mlen && !(mlen < flen)) {
                printf("\n");
            }
        }

        /* ... then swap second and first line buffers */
        t = first;
        first = second;
        second = t;
    }

    /* Fill out to memory file length, if given */
    if (m) {
        if (mlen < flen) {
            while (mlen < flen) {
                if (!(mlen % 16)) {
                    printf("\n");
                    printf("%04X:FF", mlen);
                } else {
                    printf(" FF");
                }
                ++mlen;
            }
            printf("\n");
        }
    }

    return 0;
}
