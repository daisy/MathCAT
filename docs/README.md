# MathCAT Documentation

The docs are built with [Jekyll](https://jekyllrb.com/) using the Cayman theme and hosted on GitHub Pages.

## Multi-language support

Translations live in language subdirectories (e.g. `fi/` for Finnish). Each translated page needs front matter with `lang` and `ref`:

```yaml
---
layout: default
lang: fi
ref: users
title: MathCAT-käyttöopas
---
```

- `lang` — the language code of this page
- `ref` — a shared ID linking translations of the same page (e.g. both `users.md` and `fi/users.md` have `ref: users`)

The language switcher in `_layouts/default.html` uses these to generate links between translations. It only appears on pages that have `ref` set.

To register a new language, add it to `_data/languages.yml`.

## Local development

Run _Jekyll_ from the repo root:

```
docker run --rm -v "${PWD}/docs:/srv/jekyll" -p 4000:4000 jekyll/jekyll bash -c "bundle install && bundle exec jekyll serve --host 0.0.0.0"
```

Then open http://localhost:4000, http://localhost:4000/fi/users, etc..
