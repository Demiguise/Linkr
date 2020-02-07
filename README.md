# Linkr

## Description

Linkr is a tool designed for use with dotfile repos, and other systems, which would like to centralise important configurations for use across multiple devices.

Each repository, or other system, should contain a file which specifies how to symlink or copy files within the repository to their correct loctaions. Dotfiles are a great example of this, due to the wide range of locations and requirements of each tool they configure.

---

## Usage

Each top level key in the YAML syntax denotes a block of actions to take. This block should require the full set of actions to take in order to fully distribute/install the configuration files for that particular tool/system.

```yaml
---
vim:
  symlink:
    dir: yes
    from: "some/relative/directory"
    to: "/some/absolute/directory"

  copy:
    dir: no
    from: "somefile"
    to: "/some/place/"
```
