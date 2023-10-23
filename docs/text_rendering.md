# Text Rendering

Text rendering represents an enormous spectrum of complexity.

At the simpler end are ANSI-compatible terminal emulators that iterate over each grid cell's character, mapping it to a font's bitmap glyph, and rendering it to the target surface at the appropriate position.  These emulators are limited by their design and only support a subset of Unicode's features.  Notably, each implementation seems to choose a different subset.  However, this design makes it simple to ensure the columns are properly monospaced.

At the other end of the spectrum is the text stack from a modern web browser.  VS Code and similar editors use the Chromium text stack through Electron.  As such, VS Code can properly represent all of Unicode including the zero width joiner, but it takes no action to ensure that characters line up across different lines.

This project is designed to determine the width of text displayed on editors toward the web stack end of the spectrum, i.e. editors that have full Unicode support.
