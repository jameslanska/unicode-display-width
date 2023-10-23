# Contributing to Unicode Display Width

First off, thank you for considering contributing to Unicode Display Width.  This document will help answer common questions you may have during your first contribution.

## Issue Reporting

Not every contribution comes in the form of code. Submitting, confirming, and triaging issues is an important task for any project.

We use GitHub to track all project issues.  If you

- discover bugs,
- discover a incorrect width output,
- discover a misunderstanding of Unicode in the project's documentation,
- have ideas for improvements or new features,
- notice any other problems with the documentation, or
- notice that the version of Unicode supported is out of date,

please start by opening an issue on this repository.  We use issues to centralize the discussion and agree on a plan of action before spending time and effort writing code that might not get used.

If you find a security vulnerability, do ***NOT*** open an issue.  Follow the instructions in `SECURITY.md`.

### Submitting An Issue

1. Check that the issue has not already been reported.
2. Select the appropriate issue type, open an issue with a descriptive title, and follow the template.
3. Be clear, concise, and precise using grammatically correct, complete sentences in your summary of the problem.
4. Include any relevant markdown input and markdown output in the issue.

## Code Contributions

Unicode Display Width follows a [forking workflow](https://docs.github.com/en/get-started/quickstart/contributing-to-projects) with the following contribution process

1. Open an issue on the [project repository](https://github.com/jameslanska/unicode-display-width/issues), if appropriate.
2. Fork the project <https://github.com/jameslanska/unicode-display-width/fork>.
3. Create your feature branch `git checkout -b my-new-feature`.
4. Commit your changes git `commit -am 'Add some feature'`.
5. Push to the branch `git push origin my-new-feature`.
6. Create a [GitHub Pull Request](https://help.github.com/articles/about-pull-requests/) for your change following instructions in the pull request template.
7. Participate in a Code Review with the project maintainer on the pull request.

Releases will most likely follow quickly after merging into `main`.

### What We Are Looking For

- bug fixes
- documentation improvements
- code style improvements
- performance improvements
- maintenance updates (Unicode versions, dependencies, etc.)
- etc.

## Coding conventions

Start reading the code and you'll get the hang of it.  We optimize for readability:

- every change must have corresponding tests
- version numbers follow [Semantic Versioning](https://semver.org/)
- documentation must be updated (if applicable)
