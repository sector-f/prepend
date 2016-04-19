% PREPEND(1) 2.1.3
%
% 2016-04-19

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
Data is read from standard input until EOF and is then added to the
beginning of each *FILE*. Standard output is unused unless **--tee**
is specified. Standard error is used only for error messages.

OPTIONS
=======

-t, --tee

:	Prints the contents of *FILE* to stdout after prepending data.
	If more than one *FILE* was specified, the new contents of
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

Check every .sh file in the current directory for a shebang,
and prepend '#!/usr/bin/env bash' to the file if none is found:

~~~
for file in ./*.sh; do
	if ! grep '#!' "$file" &> /dev/null; then
		prepend "$file" <<< '#!/usr/bin/env bash'
	fi
done
~~~

DIAGNOSTICS
===========

The error message that is most likely to be displayed
during normal usage is "Writing to file *FILE* failed: CAUSE",
with CAUSE being something like "Permission denied" or
"Is a directory". These errors should be self-explanatory.

Invalid usage will result in an error message
detailing how the usage was incorrect.

Other error messages may be displayed if I/O errors occur.

EXIT STATUS
===========

0

:	No errors occurred

1

:	Invalid syntax or usage

2

:	Files were specified, but none could be written to

3

:	Other errors occurred


BUGS
====

Potential for Data Loss
-------

Because **prepend** functions by truncating the *FILE*(s) to zero length before
writing data, data loss is possible if **prepend** is killed
or interrupted. However, this is a small risk, equivalent
to the chance of data loss occurring when using a shell to redirect
the output of a command to a file.

Reporting Bugs
--------------

Any bugs should be reported to <https://github.com/sector-f/prepend/issues>

COPYRIGHT
=========

Copyright (C) 2016, Adam Simeth. This software is licensed under the 3-clause BSD license; you should have received a copy of the license with this software. A copy of this license can be found at
<https://raw.githubusercontent.com/sector-f/prepend/master/LICENSE>

SEE ALSO
========

awk(1) and sed(1) for more-powerful text editing

Source code for **prepend** can be found at <https://github.com/sector-f/prepend>
