% PREPEND(1) 2.1.1
%
% 2016-04-18

NAME
====

**prepend** - prepends data to files

SYNOPSIS
========

| **prepend** [**-t**|**--tee**] *FILE*...
| **prepend** **-h**|**--help**
| **prepend** **-V**|**--version**

DESCRIPTION
===========

**prepend**, as the name suggests, prepends data to files.
Data is read from standard input and stored in a buffer until EOF is reached.
*FILE* is then read into a separate buffer until EOF. *FILE* is then
truncated to zero length, and the standard input buffer and *FILE*'s
original contents are then written to *FILE*. This is performed
for each specified *FILE*, in the order in which they were specified.

OPTIONS
=======

-t, --tee

:	Prints the contents of *FILE* to stdout after prepending data;
	if more than one *FILE* was specified, the new contents of
	each *FILE* is printed in the order in which the *FILE*s were listed

-h, --help

:	Prints help information

-V, --version

:	Prints the current version number

EXAMPLES
========

Add the word "Groceries" (followed by a line break) to the start of shopping.txt:

	prepend shopping.txt <<< "Groceries"

Prepend the contents of foo.txt to bar.txt:

	prepend bar.txt < foo.txt

Check every .sh file in the current directory for a shebang, and prepend '#!/usr/bin/env bash' to the file if none is found:

~~~
for file in ./*.sh; do
	if ! grep '#!' "$file" &> /dev/null; then
		prepend "$file" <<< '#!/usr/bin/env bash'
	fi
done
~~~

DIAGNOSTICS
===========

The error message that is most likely to be displayed is "Writing to file FILE failed: CAUSE", with CAUSE being something like "Permission denied" or "Is a directory." These errors should be fairly self-explanatory.

Other error messages may be displayed if I/O errors occur.

EXIT VALUES
===========

0

:	No errors occurred

1

:	Invalid syntax or usage

2

:	Files were specified, but none could be written to

3

:	Other errors occurred

WARNING
=======

Because **prepend** functions by truncating the *FILE*(s) to zero length before
writing data, data loss is possible if the program is killed
or interrupted. However, this is a small risk, equivalent
to the chance of data loss occurring when using a shell to redirect
the output of a command to a file.

BUGS
====

Any bugs should be reported to <https://github.com/sector-f/prepend/issues>

SEE ALSO
========

Source code can be found at <https://github.com/sector-f/prepend>
