# prepend

Command-line utility for adding text to the start of a file.  
Input text is received through standard input. Output file is specified as an argument.

**Warning:** This program functions by truncating the file to zero length before writing data to it.
Data loss is possible if the program is killed or interrupted in any way. Use at your own risk.

### Example usage:

`prepend shopping.txt <<< "Groceries"`

`prepend file_b < file_a`

### To-Do:

Make `prepend` work with non-UTF-8 data
