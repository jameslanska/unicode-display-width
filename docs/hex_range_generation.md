# Hex Range Generation

This document describes the design decisions for `generate_ranges/gen.py` which generates `code_point_ranges.rs`.

## Private Code Point Ranges

Private use code point ranges are specified in the [private use FAQ](https://www.unicode.org/faq/private_use.html).

> "The General_Category property value Private_Use (Co) is immutable: the set of code points with that value will never change."
>
> - <https://www.unicode.org/policies/stability_policy.html#Property_Value>

```rust
PRIVATE_USE_RANGES: &[CodePointRange] = &[
    0x0E000..=0x0F8FF,
    0xF0000..=0xFFFFD,
    0x100000..=0x10FFFD,
]
```

Private use ranges are assumed to be width one, so they are ignored.

## Unassigned Code Point Ranges

From [EastAsianWidth-16.0.0.txt](https://www.unicode.org/Public/16.0.0/ucd/EastAsianWidth.txt):

> The unassigned code points in the following blocks default to "W":
>
> - CJK Unified Ideographs Extension A: U+3400..U+4DBF
> - CJK Unified Ideographs:             U+4E00..U+9FFF
> - CJK Compatibility Ideographs:       U+F900..U+FAFF
>
> All undesignated code points in Planes 2 (Supplementary Ideographic Plane) and 3 (Tertiary Ideographic Plane), whether inside or outside of allocated blocks, default to "W":
>
> - Plane 2: U+20000..=U+2FFFD
> - Plane 3: U+30000..=U+3FFFD

```rust
WIDE_UNASSIGNED_RANGES: &[CodePointRange] = &[
    0x3400..=0x4DBF,
    0x4E00..=0x9FFF,
    0xF900..=0xFAFF,
    0x20000..=0x2FFFD,
    0x30000..=0x3FFFD,
]
```

## Emojis

> They will typically have about the same vertical placement and advance width as CJK ideographs
>
> - <https://www.unicode.org/reports/tr51/>

Unicode Display Width assumes emojis defined with Emoji_Presentation character property will have width 2.

## Wide East Asian Width Properties

Unicode's East Asian Width property has two values that denote double width: `W` (Wide) and `F` (Full-width).  All other values are ignored.
