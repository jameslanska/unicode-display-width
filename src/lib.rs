#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod code_point_ranges;

use code_point_ranges::{ASCII_TABLE, CodePointRange, DOUBLEWIDE_TABLE};
use unicode_segmentation::UnicodeSegmentation;

/// Check if char `c` is in array of code point ranges using binary search.
///
/// This operation is theoretically faster with the range-set data structure.  See <https://github.com/CarlKCarlK/range-set-blaze>.  In the interest of fewer dependencies and simpler code, this optimization has not been implemented.
fn in_table(arr: &[CodePointRange], c: char) -> bool {
    let c = c as u32;
    arr.binary_search_by(|range| {
        if range.contains(&c) {
            std::cmp::Ordering::Equal
        } else {
            range.start().cmp(&c)
        }
    })
    .is_ok()
}

/// Check if the char `c` has double width.
///
/// ## Examples
///
/// ```rust
/// use unicode_display_width::is_double_width;
///
/// // assert_eq!(is_double_width('🛡'), false);
/// assert_eq!(is_double_width('✅'), true);
/// ```
pub fn is_double_width(c: char) -> bool {
    // Since ASCII characters are so much more common in English text, check these first
    if in_table(ASCII_TABLE, c) {
        return false;
    }

    if in_table(DOUBLEWIDE_TABLE, c) {
        return true;
    }

    false
}

/// Get the number of columns required to display the grapheme cluster in a monospace font
///
/// Returns either `1` or `2`
fn get_grapheme_width(grapheme_cluster: &str) -> u64 {
    for scalar_value in grapheme_cluster.chars() {
        // emoji style variation selector
        if scalar_value == '\u{FE0F}' {
            return 2;
        }

        if is_double_width(scalar_value) {
            return 2;
        }
    }

    1
}

/// Return the number of columns required to display the `text` string in a monospace font as a sequence of extended grapheme clusters.  
///
/// Overflow is not realistically possible in this function with `u64` since each operation takes ~20 nanoseconds to complete (~500 years of continuous operation to overflow).  In terms of memory, an 18 exabyte string would need to be parsed to overflow.
///
/// ## Examples
///
/// ```rust
/// use unicode_display_width::width;
///
/// assert_eq!(width("👨‍👩‍👧‍👧"), 2);
/// assert_eq!(width("слава україні"), 13); // Glory to Ukraine in Ukrainian
/// assert_eq!(width("ݓ΅ɓԶѥƘҕ࠹ɇঐԢظٰ"), 12); // randomly generated Unicode
/// ```
pub fn width(text: &str) -> u64 {
    text.graphemes(true).fold(0, |acc, grapheme_cluster| {
        acc + (get_grapheme_width(grapheme_cluster))
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case('🛡', false)]
    #[test_case('✅', true)]
    fn test_width(text: char, wide: bool) {
        assert_eq!(is_double_width(text), wide);
    }

    #[test_case("🛡", 1; "length 1 grapheme")]
    #[test_case("\u{2764}", 1; "Heavy Black Heart emoji")]
    #[test_case("\u{2764}\u{FE0F}", 2; "Heavy Black Heart emoji with emoji style variation selector in Hex representation")]
    #[test_case("❤️", 2; "Heavy Black Heart emoji with emoji style variation selector")] // VS Code doesn't seem to support variation selectors
    #[test_case("✅", 2; "length 2 grapheme")]
    #[test_case("👨‍👩‍👧‍👧", 2; "grapheme composed of multiple emojis, at least one of which is length 2")]
    #[test_case("test test", 9; "ASCII text")]
    #[test_case("🗡", 1; "single width because it may be paired with the shield which is also a length 1 code point")]
    #[test_case("🔥🗡🍩👩🏻‍🚀⏰💃🏼🔦👍🏻", 15; "U+1F608")] // 😈
    #[test_case("слава україні", 13; "Glory to Ukraine in Ukrainian")]
    #[test_case("슬라바 우크라이나", 17; "Glory to Ukraine in Korean")]
    #[test_case("Ẓ̌á̲l͔̝̞̄̑͌g̖̘̘̔̔͢͞͝o̪̔T̢̙̫̈̍͞e̬͈͕͌̏͑x̺̍ṭ̓̓ͅ", 9; "corrupted text")]
    fn test_string_width(text: &str, length: u64) {
        assert_eq!(width(text), length);
    }

    // The results of Indic script text may not be useful
    #[test_case("ണ്‍", 1; "Indic text with zero width joiner")]
    #[test_case("ന്‍", 1; "Indic text with zero width joiner 2")]
    #[test_case("ര്‍", 1; "Indic text with zero width joiner 3")]
    #[test_case(
        "\u{0924}\u{094D}\u{200D}\u{0928}",
        1;
        "Half letter form" // https://www.unicode.org/faq/indic.html
    )]
    #[test_case(
        "\u{0924}\u{094D}\u{200C}\u{0928}",
        2;
        "Single glyph form" // https://www.unicode.org/faq/indic.html
    )]
    fn indic_script(text: &str, length: u64) {
        assert_eq!(width(text), length);
    }
}
