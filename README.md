# gwcheck

## Description

gwcheck is a tool to check .gnu.warning.* sections in ELF object
files and display their content.

For an introduction to .gnu.warning.* sections, please refer to this
[article][1].

Support for emitting linker warnings when using a symbol for which a
.gnu.warning.symbol section exists is implemented in GNU linkers (ld
and gold), but currently not in LLVM's LLD linker.

C libraries using this mechanism:

- FreeBSD libc
- NetBSD libc
- OpenBSD libc
- DragonFlyBSD libc
- GNU C Library (glibc)
- Newlib C library
- diet libc
- uClibc

## Requirements

gwcheck requires Python 3 and the following Python module:

- pyelftools

## Usage

	gwcheck [-hv] object

The options are as follows:

	-h	Display usage.
	-v	Display version.

## Example

Here is the output of running gwcheck on OpenBSD 7.0 libc:

	$ gwcheck libc.so.96.1
	.gnu.warning.strcpy:
		strcpy() is almost always misused, please use strlcpy()
	.gnu.warning.stpcpy:
		stpcpy() is dangerous; do not use it
	.gnu.warning.wcscat:
		wcscat() is almost always misused, please use wcslcat()
	.gnu.warning.sprintf:
		sprintf() is often misused, please use snprintf()
	.gnu.warning.tempnam:
		tempnam() possibly used unsafely; consider using mkstemp()
	.gnu.warning.vsprintf:
		vsprintf() is often misused, please use vsnprintf()
	.gnu.warning.mktemp:
		mktemp() possibly used unsafely; consider using mkstemp()
	.gnu.warning.strcat:
		strcat() is almost always misused, please use strlcat()
	.gnu.warning.wcscpy:
		wcscpy() is almost always misused, please use wcslcpy()
	.gnu.warning.rand_r:
		rand_r() is not random, it is deterministic.
	.gnu.warning.rand:
		rand() may return deterministic values, is that what you want?
	.gnu.warning.getwd:
		getwd() possibly used unsafely; consider using getcwd()
	.gnu.warning.random:
		random() may return deterministic values, is that what you want?
	.gnu.warning.tmpnam:
		tmpnam() possibly used unsafely; consider using mkstemp()

## License

gwcheck is released under the BSD 2-Clause license. See `LICENSE` file for details.

## Author

gwcheck is developed by Frederic Cambus.

- Site: https://www.cambus.net

[1]: https://ninjalj.blogspot.com/2011/11/your-own-linker-warnings-using-gnu.html
