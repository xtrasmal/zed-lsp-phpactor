# PHP language extension for Zed
[![Rust](https://github.com/xtrasmal/zed-lsp-phpactor/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/xtrasmal/zed-lsp-phpactor/actions/workflows/rust.yml)


>PHPACTOR IS NOW PART OF ZED'S EXTENSIONS. DON'T USE THIS ANYMORE ⚠️

## What is it?

>Intelligent completion and refactoring tool for PHP

## What is it?
- adds `phpactor` as language server provider
- adds PHP language support to Zed editor
- adds many features like:
  - configuration(https://phpactor.readthedocs.io/en/master/reference/configuration.html)
  - code completion(https://phpactor.readthedocs.io/en/master/reference/completion.html)
  - navigation(https://phpactor.readthedocs.io/en/master/reference/navigation.html#)
  - diagnostics(https://phpactor.readthedocs.io/en/master/reference/diagnostic.html)
  - indexing(https://phpactor.readthedocs.io/en/master/reference/indexer.html)
  - refactorings(https://phpactor.readthedocs.io/en/master/reference/refactorings.html)
  - quick fixes(https://phpactor.readthedocs.io/en/master/lsp/code-actions.html)
  - integrations(https://phpactor.readthedocs.io/en/master/integrations.html)
    - Behat
    - Drupal 8+
    - PHP-CS-Fixer
    - PHP_CodeSniffer
    - PHPStan
    - PHPUnit
    - Prophecy
    - Psalm
    - Symfony
  - debugging(https://phpactor.readthedocs.io/en/master/development/debugging.html)
  - profiling(https://phpactor.readthedocs.io/en/master/development/profiling.html)

## How to install it?
1. Download Phpactor: https://phpactor.readthedocs.io/en/master/usage/standalone.html
2. Add `phpactor` to your PATH(https://en.wikipedia.org/wiki/PATH_(variable))

Add the following config to your `settings.json`:
```json
{
    "languages": {
      "PHP": {
        "language_servers": ["phpactor", "tailwindcss-language-server"]
      },
    }
}
```
3. Restart Zed.

## Configuring Phpactor

Add a global configuration file to your home directory `~/me/.config/phpactor/.phpactor.json`:

```json
{
  "$schema": "phpactor.schema.json",
  // add configuration here
}
```

You can also add a project specific configuration file to your project directory `<project-dir>/.phpactor.json`:

```json
{
  "$schema": "phpactor.schema.json",
  // add configuration here
}
```

### Links
**Zed editor:**
- https://zed.dev

**Phpactor:**
- https://phpactor.readthedocs.io/en/master/index.html
- https://github.com/phpactor/phpactor

**Reference:**
- https://phpactor.readthedocs.io/en/master/reference.html
