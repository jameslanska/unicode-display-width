"""
See /docs/hex_range_generation.md for details
"""

from typing import Generator
from urllib.request import urlopen
from pathlib import Path

VERSION = "15.1.0"

EAW_URL = f"https://unicode.org/Public/{VERSION}/ucd/EastAsianWidth.txt"
EMOJI_DATA_URL = f"https://unicode.org/Public/{VERSION}/ucd/emoji/emoji-data.txt"


# Special Cases
#
#  - The unassigned code points in the following blocks default to "W":
#      CJK Unified Ideographs Extension A:        U+3400..U+4DBF
#      CJK Unified Ideographs:                    U+4E00..U+9FFF
#      CJK Compatibility Ideographs:              U+F900..U+FAFF
#
#  - All undesignated code points in Planes 2 and 3, whether inside or outside of allocated blocks, default to "W":
#      Plane 2 (Supplementary Ideographic Plane): U+20000..U+2FFFD
#      Plane 3 (Tertiary Ideographic Plane):      U+30000..U+3FFFD
#
WIDE_UNASSIGNED_RANGES = [
    (0x3400, 0x4DBF),
    (0x4E00, 0x9FFF),
    (0xF900, 0xFAFF),
    (0x20000, 0x2FFFD),
    (0x30000, 0x3FFFD),
]


def hexrange_to_range(hexrange: str) -> range:
    """
    Given a string like 1F300..1F320 representing an inclusive range, return the range of codepoints.  If the string is of the form 1F321, return a range of just that element.
    """

    fields = [int(val, 16) for val in hexrange.split("..")]

    # convert single element inputs into ranges
    if len(fields) == 1:
        fields += fields

    return range(fields[0], fields[1] + 1)


def get_hex_ranges_from_emoji_data() -> list[range]:
    """
    1. Download the current emoji character property information from Unicode.org
    2. Return the ranges that have a value of "W" or "F"

    Example line:
    2757          ; Emoji_Presentation   # E0.6   [1] (❗)       red exclamation mark
    """

    emoji_data = get_text_from_url(EMOJI_DATA_URL)

    ranges = [
        hexrange_to_range(line.split(" ")[0])
        for line in emoji_data
        if "; Emoji_Presentation" in line
    ]

    return ranges


def get_hex_ranges_from_eaw_data() -> list[range]:
    """
    1. Download the current East Asian Width information from Unicode.org
    2. Return the ranges that have a value of Emoji_Presentation
    """

    east_asian_width_data = get_text_from_url(EAW_URL)

    def is_wide(unicode_line: str) -> bool:
        # Example lines:
        # 01DD..024F     ; N  # L&   [115] LATIN SMALL LETTER TURNED E..LATIN SMALL LETTER Y WITH STROKE
        # 0250           ; N  # Ll         LATIN SMALL LETTER TURNED A
        EAW = unicode_line.split("#")[0].strip().split(";")[1].strip()
        return EAW in ["W", "F"]

    ranges = [
        hexrange_to_range(line.split(" ")[0])
        for line in east_asian_width_data
        if is_wide(line)
    ]

    return ranges


def get_text_from_url(url: str) -> list[str]:
    """
    1. Get text from `url`
    2. Write that text to a file
    3. Return each line from the text returned from url that includes data (no empty lines and no comments)
    """

    def is_data_line(line: str) -> bool:
        if line.startswith("#") or line == "":
            return False

        return True

    with urlopen(url) as response:
        response_content = response.read()
        text: str = response_content.decode("utf-8")
        text_path = Path.cwd() / url.rsplit("/", 1)[-1]
        text_path.write_text(text)
        return [line for line in text.split("\n") if is_data_line(line)]


def get_unsorted_wide_hex_ranges() -> list[range]:
    """Get all hex ranges from the Unicode standard that are displayed as wide glyphs"""

    hex_ranges: list[range] = []

    hex_ranges += get_hex_ranges_from_eaw_data()
    hex_ranges += get_hex_ranges_from_emoji_data()
    hex_ranges += WIDE_UNASSIGNED_RANGES

    return hex_ranges


def sort_and_merge_codepoint_ranges(
    unsorted_ranges: list[range],
) -> list[tuple[int, int]]:
    """
    Convert the list of ranges with possible duplicates and overlaps into a sorted list of disjoint inclusive tuples.
    """

    # Add all code points from the ranges into the set
    code_points = set()

    for r in unsorted_ranges:
        code_points.update(r)

    def group(L: list) -> Generator[tuple[int, int], None, None]:
        first = last = L[0]
        for n in L[1:]:
            if n - 1 == last:  # Part of the group, bump the end
                last = n
            else:  # Not part of the group, yield current group and start a new
                yield first, last
                first = last = n
        yield first, last  # Yield the last group

    groups = list(group(list(code_points)))
    return groups


def main():
    """
    1. Get all appropriate hex ranges
    2. Sort and merge ranges
    3. Apply ranges to template.rs
    """

    unsorted_ranges = get_unsorted_wide_hex_ranges()
    sorted_tuples = sort_and_merge_codepoint_ranges(unsorted_ranges)

    # ":x" formats the numbers as hexadecimal
    tuples_str = "\n".join(f"    0x{a[0]:x}..=0x{a[1]:x}," for a in sorted_tuples)

    rust_template_path = Path(__file__).parent / "template.rs"
    rust_template = rust_template_path.read_text()

    output_path = Path(__file__).parent / "code_point_ranges.rs"
    rust = rust_template.replace("{VERSION}", VERSION)
    rust = rust.replace("    // {RANGES}", tuples_str)

    output_path.write_text(rust)


if __name__ == "__main__":
    main()
