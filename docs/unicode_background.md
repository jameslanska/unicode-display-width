# Unicode Background

A few definitions are helpful for further discussion:

> **Unicode Definition:** A [*grapheme cluster*](https://unicode.org/glossary/#grapheme_cluster) is the text between grapheme cluster boundaries as specified by [Unicode Standard Annex #29](https://www.unicode.org/reports/tr29/), "Unicode Text Segmentation."
>
> A grapheme cluster represents a horizontally segmentable unit of text, consisting of some grapheme base together with any number of nonspacing marks applied to it.

> **Unicode Definition:** A [*code point*](https://unicode.org/glossary/#code_point) is any value in the Unicode codespace; that is, the range of integers from 0 to 10FFFF (base 16)

A grapheme cluster can be thought of as a collection of one or more code points (an atomic part of text) that is displayed as a single ["user-perceived character"](https://unicode.org/reports/tr29/).

Determining the width of an arbitrary string is inherently difficult because there is not a single source of truth on the width of an arbitrary grapheme cluster in the Unicode standard.

## East Asian Width Property

The [Unicode Standard Annex /#11](https://www.unicode.org/reports/tr11/tr11-11.html) defines the East Asian Width property on Unicode *code points* with the possible values of

- `W` (Wide)
- `Na` (Narrow)
- `H` (Half-width)
- `F` (Full-width)
- `A` (Ambiguous)
- `Neutral` (Not East Asian)

Unfortunately, it also states:

> **Note:** The East_Asian_Width property is not intended for use by modern terminal emulators without appropriate tailoring on a case-by-case basis. Such terminal emulators need a way to resolve the halfwidth/fullwidth dichotomy that is necessary for such environments, but the East_Asian_Width property does not provide an off-the-shelf solution for all situations. The growing repertoire of the Unicode Standard has long exceeded the bounds of East Asian legacy character encodings, and terminal emulations often need to be customized to support edge cases and for changes in typographical behavior over time.

and

> Combining marks have been classified and are given a property assignment based on their typical applicability. For example combining marks typically applied to characters of class N, Na or W are classified as A. Combining marks for purely non-East Asian scripts are marked as N, and non-spacing marks used only with wide characters are given a W. Even more so than for other characters, the **East Asian Width property for combining marks is not the same as their display width**.

In other words, the width of a code point is not exclusively determined by its East Asian Width property.

The number of possible graphemes is infinite, so defining width only in terms of code points limits the combinatorial complexity.  To visualize what infinite grapheme possibilities looks like, peruse <https://glitchtextgenerator.com/>.  It is a great source of strange Unicode text.  For a bit of fun, pass `"Ẓ̌á̲l͔̝̞̄̑͌g̖̘̘̔̔͢͞͝o̪̔T̢̙̫̈̍͞e̬͈͕͌̏͑x̺̍ṭ̓̓ͅ"` into any text program to see how it behaves.

This project *assumes* that if there are one or more code points with East Asian Width property of either `W` (Wide) or `F` (Fullwidth) in a grapheme, the entire grapheme will be rendered with width two, otherwise width one.  Therefore, to determine the width of a string, it is split into a sequence of graphemes, and then the sum of the width of each grapheme is returned.

This approach sidesteps the issue of combining marks and eliminates the possibility that a character's width will be outside of the set `{1, 2}`.

## Ambiguous Width

The [Unicode Standard Annex /#11](https://www.unicode.org/reports/tr11/tr11-11.html) describes *ambiguous width* as follows:

> Ambiguous width characters are all those characters that can occur as fullwidth characters in any of a number of East Asian legacy character encodings. They have a “resolved” width of either narrow or wide depending on the context of their use. If they are not used in the context of the specific legacy encoding to which they belong, their width resolves to narrow. Otherwise, it resolves to fullwidth or halfwidth. The term context as used here includes extra information such as explicit markup, knowledge of the source code page, font information, or language and script identification. For example:
>
> - Greek characters resolve to narrow when used with a standard Greek font, because there is no East Asian legacy context.
> - Private-use character codes and the replacement character have ambiguous width, because they may stand in for characters of any width.
> - Ambiguous quotation marks are generally resolved to wide when they enclose and are adjacent to a wide character, and to narrow otherwise.

This project defaults to standard width (1) for ambiguous characters, as is the default for modern character sets.

If there is interest in legacy CJK context processing, please open up an issue and describe your use case.

## Joiners and Modifiers

Unicode has two primary methods of connecting multiple code points into a larger grapheme: joiners and modifiers.  Modifiers are primarily used for diacritics but are sometimes used for things beyond their original purpose.

![](https://imgs.xkcd.com/comics/vomiting_emoji.png)

As an alternative, zero-width joiner sequences are used, where an emoji is encoded as a series of simpler emoji and zero-width joiners.

## Emoji Presentation

Unicode Defines six [emoji character properties](https://www.unicode.org/reports/tr51/#Emoji_Properties):

- Emoji
- Emoji_Presentation
- Emoji_Modifier
- Emoji_Modifier_Base
- Emoji_Component
- Extended_Pictographic

Emoji_Presentation describes "characters that have emoji presentation by default".  This project assumes such characters have double width.

## Emoji Variation Selector

> Emoji are typically presented in color (aka “emoji presentation”), but some can also be presented in black and white (aka “text presentation”). Whether a particular emoji displays in color or in black and white can be controlled through the use of variation selectors, but it can also vary from platform to platform.
>
> - <https://www.unicode.org/faq/emoji_dingbats.html>

The use of an emoji base followed by a variation selector is called a [variation sequence](https://www.unicode.org/glossary/#variation_sequence).

The `U+FE0F` variation selector requests the "emoji style" which is typically colored.  `U+FE0E` for the text style (black and white).

For example, the "Heavy Black Heart" Unicode code point U+2764 "❤" is often part of a variation sequence.  With the "emoji style" variation selector, it becomes the red heart emoji "❤️".  Note that the default VS Code fonts may not properly support variation selectors.  `U+2764` was added to Unicode 1.1 in 1993 with a "Neutral" East Asian Width.  The variation sequence was added to Emoji 1.0 in 2015.

Unicode Display Width assumes that any grapheme with U+FE0F has double width.

## Further Reading

- [Unicode Technical Documentation](http://unicode.org/main.html)
