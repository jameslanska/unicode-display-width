# Editor Choice

The maintainers of Unicode Display Width are not aware of any text editor with a true monospaced render that supports the zero width joiner, i.e. can display `👨‍👩‍👧‍👧` as a single character.

VS Code and all other Electron-based editors don't include a true monospace render.  Instead, font rendering is [delegated to Chromium](https://github.com/Microsoft/vscode/issues/14589).  This means that the monospaced effect can be created with monospaced fonts, but the editor doesn't take any action to ensure it occurs.

`vim` does attempt to ensure monospacing regardless of font.  However, it doesn't support combined emojis.  For example `👨‍👩‍👧‍👧` will be rendered as ` 👨<200d>👩<200d>👧<200d>👧`.  This is not an issue with the terminal emulator, but instead with [vim itself](https://github.com/vim/vim/issues/7932).

Most shells including `bash` and `zsh` and most terminal emulators do not support the zero width joiner.

If your application needs the width as rendered by any of these systems, DO NOT use this crate.  This project is more suited towards editors that wrap a web engine such as Chromium with Electron (e.g. VS Code).

## Contributing

If you know of a text editor that supports true monospace render and can render compound emojis, please create a new issue listing the operating system and editor so that it can be added to the documentation.

If you have recommendations for font combinations to create the monospace effect for a specific editor/platform combination, please open an issue or create a PR.

## Further Reading

- [A Look into a terminal emulator's text stack by Christian Parpart](https://dev.to/christianparpart/look-into-a-terminal-emulator-s-text-stack-3poe)
- [kitty wcwidth related GitHub issues](https://github.com/kovidgoyal/kitty/issues?q=is%3Aissue+wcwidth+)
- [zsh width code](https://github.com/zsh-users/zsh/blob/70320635b4b50b1e84f70e17bf40f107d140bdcf/Src/wcwidth9.h#L7)
