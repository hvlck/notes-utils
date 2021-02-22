# notes-utils

utilities for managing my notes

## Installation

### Requirements

+ [preen](https://github.com/EthanJustice/preen) (link-checking tool)

## Roadmap

+ `index` command to build an index based on page descriptions and other notes in the same directory
+ `unread` command to index unread articles by priority
+ metadata generation tools
+ content archiver (for every image and link that appears)
+ warnings for duplicate links
+ coloured text
+ `--type` arg for `new` command
+ validator to conform to style guide as defined in `notes/meta`
  + link types
  + alphabetical headings
+ timing details
+ `link` to auto-generate reference links from markdown links with no text (e.g. `[](https://github.com/)` would become `[GitHub: The Largest and Most Advanced Open Source Development Platform - Secure Public & Private Repositories - Code Review - Codespaces - Actions - CI CD - Project Management - DevOps · GitHub](https://github.com/)`)
  + use page `<title>`
  + `reqwest` crate
