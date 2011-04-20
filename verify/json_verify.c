/*
 * Copyright (c) 2007-2011, Lloyd Hilaiel <lloyd@hilaiel.com>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

#include <yajl/yajl_parse.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void
usage(const char * progname)
{
    fprintf(stderr, "%s: validate json from stdin\n"
                    "usage: json_verify [options]\n"
                    "    -q quiet mode\n"
                    "    -c allow comments\n"
                    "    -u allow invalid utf8 inside strings\n",
            progname);
    exit(1);
}

int
main(int argc, char ** argv)
{
    yajl_status stat;
    size_t rd;
    yajl_handle hand;
    static unsigned char fileData[65536];
    int quiet = 0;
	int retval = 0, done = 0;
    yajl_parser_config cfg = { 0, 1 };

    /* check arguments.*/
    int a = 1;
    while ((a < argc) && (argv[a][0] == '-') && (strlen(argv[a]) > 1)) {
        unsigned int i;
        for ( i=1; i < strlen(argv[a]); i++) {
            switch (argv[a][i]) {
                case 'q':
                    quiet = 1;
                    break;
                case 'c':
                    cfg.allowComments = 1;
                    break;
                case 'u':
                    cfg.checkUTF8 = 0;
                    break;
                default:
                    fprintf(stderr, "unrecognized option: '%c'\n\n", argv[a][i]);
                    usage(argv[0]);
            }
        }
        ++a;
    }
    if (a < argc) {
        usage(argv[0]);
    }

    /* allocate a parser */
    hand = yajl_alloc(NULL, &cfg, NULL, NULL);

	while (!done) {
        rd = fread((void *) fileData, 1, sizeof(fileData) - 1, stdin);

        retval = 0;

        if (rd == 0) {
            if (!feof(stdin)) {
                if (!quiet) {
                    fprintf(stderr, "error encountered on file read\n");
                }
                retval = 1;
                break;
            }
            done = 1;
        }
        fileData[rd] = 0;

        if (done)
            /* parse any remaining buffered data */
            stat = yajl_parse_complete(hand);
        else
            /* read file data, pass to parser */
            stat = yajl_parse(hand, fileData, rd);

        if (stat != yajl_status_ok &&
            stat != yajl_status_insufficient_data)
        {
            if (!quiet) {
                unsigned char * str = yajl_get_error(hand, 1, fileData, rd);
                fprintf(stderr, "%s", (const char *) str);
                yajl_free_error(hand, str);
            }
            retval = 1;
            break;
        }
    }

    yajl_free(hand);

    if (!quiet) {
        printf("JSON is %s\n", retval ? "invalid" : "valid");
    }

    return retval;
}