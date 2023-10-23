# Fonts

No single font can define glyphs for all Unicode code points since Unicode has 1,114,112 possible code points (of which [289,460](https://www.unicode.org/versions/stats/charcountv15_1.html) are designated in [Unicode 15.1.0](https://www.unicode.org/versions/Unicode15.1.0/)) and each of the standard font file types (OTF, TTF, WOFF, WOFF2) can only include $2^{16}$ $(65,535)$ glyphs per font.

## Glyph Width

A few definitions are helpful for further discussion:

> **Definition:** *Sidebearings* are the empty space around a character, both to the left and right.

> **Definition:** *Advance width* is the horizontal distance that a glyph occupies, including its sidebearings.  In other words, the distance between the glyph's initial pen position and the next glyph's initial pen position.

Monospace fonts have a consistent advance width across all glyphs in the font, but do not necessarily have consistent advance width *between* fonts.  This means that text rendered with multiple different monospace fonts may not align perfectly across lines.

This issue is discussed regarding emojis in [VS Code Issue 100730](https://github.com/microsoft/vscode/issues/100730).  Emojis are printed almost (but not exactly) double the width of regular ASCII characters.  The effect will be slightly different depending on the fonts and operating system used.

## Non-exhaustive Font Family

On Windows, VS Code has the [default font family setting](https://code.visualstudio.com/docs/getstarted/settings)

```json
"editor.fontFamily": "Consolas, 'Courier New', monospace",
```

and on macOS

```json
"editor.fontFamily": "Menlo, Monaco, 'Courier New', monospace",
```

These font families are not exhaustive, i.e. they do not cover all Unicode code points.  When VS Code (i.e. Chromium) attempts to find a glyph for a grapheme that isn't covered in the above fonts, the browser engine uses the fonts available on the system.  This means that a font family with only monospace fonts can render using fonts that aren't monospace.  This is the case for most emojis, and many foreign characters.

## Emoji Fonts

On Apple platforms, the Apple emoji font "Apple Color Emoji" doesn't have an advance width for its emojis that is a multiple of the advance with of its regular monospace font "Menlo".  The same appears to be true of Google's and Microsoft's emoji fonts.

There are a relatively small number of emoji fonts; these are mostly created by major tech companies.  This leaves few alternatives to the default emoji fonts.  If you are looking for alternative fonts, [Fileformat.info](https://www.fileformat.info) is a great resource for determining which fonts include a glyph for a given code point.  For example, the fonts available for the [high voltage sign emoji](https://www.fileformat.info/info/unicode/char/26a1/fontsupport.htm).

## Platform Specific Emojis

Major platforms such as Apple define emojis that are [not in the Unicode standard](https://emojipedia.org/glossary#pua).  These emojis can only be viewed on their respective platforms.  For example, Memojis and animojis exclusively use Private Use Areas of the Unicode standard.  Unicode Display Width does not include any logic to account for such characters, as the semantics of such code points can change depending upon context.

## Further Reading

- <https://simoncozens.github.io/fonts-and-layout/concepts.html>
- <https://stackoverflow.com/questions/66154937/how-do-i-list-fonts-that-are-available-to-vscode-on-macos>
- <https://stackoverflow.com/questions/9351689/how-to-display-special-unicode-characters-using-monospace-font-in-html-with-pres>
- <https://github.com/microsoft/vscode/issues/100730>
- <https://github.com/microsoft/vscode/issues/32840#issuecomment-389704418>
