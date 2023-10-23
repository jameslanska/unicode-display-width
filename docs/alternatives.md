# Alternatives

## String Width Libraries

Other string width libraries do not perform the grapheme separation step and merely add together the individual widths of each code point.  This shortcut works on ASCII text but may lead to incorrect results with emojis and foreign characters.

For example, these libraries will return a width of 8 for the compound emoji "👨‍👩‍👧‍👧", despite the fact that it only takes two columns to display.  In the Readme for the `unicode-width` crate, they demonstrate merging two emojis to create a female scientist compound emoji:

```ignore
assert_eq!(UnicodeWidthStr::width("👩"), 2); // Woman
assert_eq!(UnicodeWidthStr::width("🔬"), 2); // Microscope
assert_eq!(UnicodeWidthStr::width("👩‍🔬"), 4);
```

A width of 4 is incorrect with modern rendering engines, as it only requires 2 columns to render.

> A decomposed Hangul syllable is a grapheme that consists of up to three code points. The first code point has width 2. The rest consists of Jamo vowels and/or a trailing consonant, both of which have width 1. This means that clients who naïvely sum individual characters' width, will compute string widths different from the intended width (2).
>
> <https://github.com/ridiculousfish/widecharwidth/blob/master/generate.py>

For projects built on terminal emulators and other legacy text rendering engines that don't support all of Unicode, these projects may be more accurate.

## Character Width Libraries

Most other projects only expose an API for querying the width of a single code point (or similar), not an entire string.  This includes:

- glibc's [wcwidth](https://www.man7.org/linux/man-pages/man3/wcwidth.3.html),
- Python [east_asian_width](https://docs.python.org/3/library/unicodedata.html#unicodedata.east_asian_width) from the unicodedata standard library module (only gives the letter from the East Asian Width property),
- ridiculousfish's cross-language [widecharwidth](https://github.com/ridiculousfish/widecharwidth/), and
- C library utf8proc's [utf8proc_charwidth](https://juliastrings.github.io/utf8proc/doc/utf8proc_8h.html#a45ae07c1a2b9836d5a1aae0ca4289d32).

These libraries do not all have consistent error behavior.  For example, `wcwidth` can return *negative* values since it uses error code -1 for some non-printing code points.
