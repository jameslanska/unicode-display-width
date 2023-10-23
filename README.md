[![Rust](https://github.com/jameslanska/unicode-display-width/actions/workflows/rust.yml/badge.svg)](https://github.com/jameslanska/unicode-display-width/actions/workflows/rust.yml) [![Latest Version](https://img.shields.io/crates/v/unicode-display-width.svg)](https://crates.io/crates/unicode-display-width) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)

# Unicode Display Width

A safe, performant Rust crate for determining the number of columns required to display an arbitrary string.  This conforms to [Unicode 15.1.0](https://www.unicode.org/versions/Unicode15.1.0/) and handles grapheme clusters correctly.

```rust
use unicode_display_width::width;

assert_eq!(width("🔥🗡🍩👩🏻‍🚀⏰💃🏼🔦👍🏻"), 15);
assert_eq!(width("🦀"), 2);
assert_eq!(width("👨‍👩‍👧‍👧"), 2);
assert_eq!(width("👩‍🔬"), 2);
assert_eq!(width("sane text"), 9);
assert_eq!(width("Ẓ̌á̲l͔̝̞̄̑͌g̖̘̘̔̔͢͞͝o̪̔T̢̙̫̈̍͞e̬͈͕͌̏͑x̺̍ṭ̓̓ͅ"), 9);
assert_eq!(width("슬라바 우크라이나"), 17);
```

This crate will never panic.  Even private use and unassigned code points are fully supported.

```rust
use unicode_display_width::width;

// unassigned code points are assumed to have width 1
assert_eq!(width("\u{00378}"), 1);

// private use code points are also assumed to have width 1
assert_eq!(width("\u{0E000}"), 1);
```

**Note:** GitHub's Markdown render is [not truly monospaced](./docs/editor_choice.md).

## Motivation

Emojis, ideographic Chinese, Japanese, and Korean characters, and many other Unicode graphemes are rendered with double the width of regular characters.

Determining the correct width of a grapheme is a critically important operation because it can determine how far to move the cursor when an arrow key is pressed, the delete key is pressed, or a character is added to a text document.  For example, the cursor doesn't behave intuitively with `ݓ΅ɓԶѥƘҕ࠹ɇঐԢظ` in VS Code or `🛡` in the macOS Terminal.

Width functions are used in more places than may be initially obvious.  For example, determining where to wrap text (when word wrapping is enabled).  To see the effect of VS Code's simplistic width function, paste "Ẓ̌á̲l͔̝̞̄̑͌g̖̘̘̔̔͢͞͝o̪̔T̢̙̫̈̍͞e̬͈͕͌̏͑x̺̍ṭ̓̓ͅ" onto a line repeatedly.

## How It Works

Unicode Display Width splits the string argument into a sequence of graphemes (user visible characters).  For example, `"🔥🗡👩🏻‍🚀"` is split into `"🔥", "🗡", "👩🏻‍🚀"`.

For each grapheme, it checks if any of the code points (atomic part of Unicode text) comprising that grapheme have a Unicode East Asian Width property value representing double width or an emoji character property of `Emoji_Presentation`.

- If at least one code point has double width, the width of the entire grapheme is 2 regardless of how many code points constitute the grapheme.
- If there aren't any such code points, the width of the grapheme is 1.

For example, "👩🏻‍🚀" has three code points: "👩🏻", "\u{200D}", and "🚀".  The first and third have a double wide East Asian Width property.  Therefore, "👩🏻‍🚀" has width 2.

Finally, the width of all graphemes is added together and returned.  This approach ensures that the set of possible width values for each user-visible character is `{1, 2}`.

For a fuller discussion of the nuances surrounding grapheme clusters, code points, Unicode, etc., see [Background](./docs/unicode_background.md).

## Limitations

![](https://imgs.xkcd.com/comics/unicode.png)

Unicode Display Width does not determine the rendered width in any specific editor or any specific font, but instead returns the notional width according to the Unicode 15.1.0 standard.

Legacy text rendering engines do not support all modern Unicode features, so the rendered width of some text may bear little resemblance to the notional result returned by Unicode Display Width.  This includes vim, emacs, most terminal emulators, and most shells.

Even modern browser-based text rendering solutions (e.g. VS Code, which uses Chromium through Electron) don't perfectly align with the notional Unicode width.  For example, "슬라바 우크라이나" renders with fewer horizontal pixels on GitHub than "🔥🗡🍩👩🏻‍🚀⏰💃🏼🔦👍🏻", but has a greater notional width.  For a more in-depth discussion, see [Fonts](./docs/fonts.md).

[Indic scripts](https://www.unicode.org/faq/indic.html) including Devanagari do not appear to have a [monospace representation](https://blog.denisbider.com/2015/09/when-monospace-fonts-arent-unicode.html).  In these scripts, the zero-width-joiner has [different semantics](https://www.unicode.org/faq/indic.html).  Unicode Display Width will not produce useful results on these code points.

Width is measured in terms of columns, so it is not useful with editors that are not monospaced.  These include WYSIWYG editors such as Microsoft Word or Google Docs.

For a more in-depth discussion of rendering engines and text editors, see [Editor Choice](./docs/editor_choice.md), [History](./docs/history.md), and [Alternatives](./docs/alternatives.md).

## Acknowledgements

I would like to express my deep and sincere gratitude to [Joe Lanska](https://github.com/josephlanska) for his unwavering support and for all the time he spent helping me improve the documentation.

## Support

If you would like to support further development, please consider [buying me a coffee](https://www.buymeacoffee.com/lanskajames).
