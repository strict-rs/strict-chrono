## Commit messages

- Subject: `type(scope): structural imperative description` — conventional-commit style, scope **required**, `!` appended for breaking changes. **Never `chore`** — pick the precise type (`feat`, `fix`, `refactor`, `perf`, `build`, `ci`, `docs`, `test`, `style`, `revert`, …).
- Headline the substance: the subject names the most significant behavior or API change; renames, moves, lockfile bumps, and generated artifacts are fallout, never the headline when real behavior also changed.
- Body: 1–5 sections sized to the commit. Each section starts with a plain-text header line (no `#`, no bold), followed by 3–5 imperative bullets describing structural changes; exactly one blank line between sections; fallout goes in the last section.
- Pass the message via HEREDOC to `git commit -m` so blank lines survive shell quoting.
