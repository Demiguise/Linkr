# Linkr Design Document

## Usecases

1. A user wishes to initialise a new dotfile repository on their device.

   - User downloads their dotfile repo.
   - User runs Linkr in the repository.
   - Linkr attempts to find a .Linkr file in the repository.
   - Linkr parses and attempts to perform each action described in the file.

2. A user wishes to update their dotfile repository on their device.
3. A user wishes to remove their dotfile repository from their device.

## Possible Failures

- Not enough permissions to perform the task.
- File already exists.
- Symlink already exists.

## Feature Thoughts

- CLI flag for selecting which directory to run from.
- CLI flag for selecting which file to read.
- Ansible like syntax with "Modules" for directories, symlinks, copy + paste, etc.

## Requirements

- MUST be able to read/parse a file with YAML syntax.
- MUST be able to use system calls to perform symlinks.
- MUST be able to use system calls to perform file copies.
- MUST handle permission errors when working without SUDO.
