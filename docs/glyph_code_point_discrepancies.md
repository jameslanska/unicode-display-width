# Glyph / Cope Point Width Discrepancies

> **Unicode Definition:** A [*glyph*](https://unicode.org/glossary/#glyph) is a an abstract form that represents one or more glyph images.

> **Definition:** A [*font*](https://unicode.org/glossary/#font) is a collection of glyphs used for the visual depiction of character data.

The width of some glyphs do not match their East Asian Width value.  For example, `🛡` (shield emoji) and `🗡️` (dagger emoji) have an East Asian Width of `N` (Neutral).  However, the default emoji glyph used on Apple ([Apple Color Emoji](https://emojipedia.org/apple)), Google ([Noto Color Emoji](https://fonts.google.com/noto/specimen/Noto+Color+Emoji)), and Microsoft ([Segoe UI Symbol](https://learn.microsoft.com/en-us/typography/font-list/segoe-ui-symbol)) platforms to render it has double width.  This is why the example in the Readme, "🔥🗡🍩👩🏻‍🚀⏰💃🏼🔦👍🏻" has width 15, despite the fact that it appears as if it should have width 16.

This project uses the Unicode width uncritically.  Even if there is consensus among font authors that disagrees with the width defined in Unicode, this project will return the Unicode width.

Some emojis have glyphs in different fonts with different widths.  For example, the wheelchair symbol emoji, ♿ `"\u267F"` has double width in all [main emoji fonts](https://emojipedia.org/wheelchair-symbol#designs).  However, this code point is included as a single width glyph in Apple's Menlo font.  Therefore, it renders as a single width glyph in VS Code.

For applications where accurate width is critically important, font information is required.
