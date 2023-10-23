# History

[wcwidth](https://www.man7.org/linux/man-pages/man3/wcwidth.3.html) ("wide char width") was first added to [glibc](https://www.gnu.org/software/libc/sources.html) on May 1, 1996 (commit 75cd5204dd829d849a6e41380a64cf61d7f406d0) with the description "determine columns needed for a wide character".  In this context, "wide" refers the number of bytes in the character type, not the visual width.  In 2001, `wcwidth` was added to the POSIX.1-2001 standard, despite its inability to properly handle Unicode.

`wcwidth` is widely viewed as being [fundamentally broken](https://fa.openbsd.tech.narkive.com/eUhVK16j/tmux-1-dealing-with-broken-wcwidth-3), as many Unicode code points do not have proper support, and it can return nonsensical widths for some strings.  Building a correct width function in C is difficult because C doesn't have a simple/native way to represent a grapheme cluster or even a code point.  The `char` type holds 8 bits and `wchar_t` holds 16 bits on most platforms.  17 bits are required to represent a unique Unicode code point, therefore a text encoding such as UTF-16 is required for processing.  The Unicode Consortium was incorporated in 1991, but C has yet to add native support for Unicode code points, much less full grapheme clusters.

Following the standardization of `wcwidth`, most terminal emulators adopted it.  As Unicode gained in popularity, many projects built their own custom implementation that solved (some of) the problems with `wcwdith`.  For example, [kitty](https://github.com/kovidgoyal/kitty) includes [wcwidth.py](https://github.com/kovidgoyal/kitty/blob/master/gen/wcwidth.py).

Others coalesced around `utf8proc`'s [utf8proc_charwidth](https://juliastrings.github.io/utf8proc/doc/utf8proc_8h.html#a45ae07c1a2b9836d5a1aae0ca4289d32), which correctly handles individual code points according to some font data.

Many still [advocate](https://github.com/alacritty/alacritty/issues/3975) the usage of broken wcwidth in an effort not to have divergent behavior with other applications that still use wcwidth.

`wcwidth` has seen a lot of improvement since its original development, and the name now refers to many different functions.  [Markus Kuhn](https://www.cl.cam.ac.uk/~mgk25/)'s implementation in [2007](https://www.cl.cam.ac.uk/~mgk25/ucs/wcwidth.c) includes support for Unicode 5.0.

## Further Reading

- <https://pubs.opengroup.org/onlinepubs/9699919799/functions/wcwidth.html>
