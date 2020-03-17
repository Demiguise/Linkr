# Linkr Design Document

## Usecases

1. A user wishes to initialise a new dotfile repository on their device.

   - User downloads their dotfile repo.
   - User runs Linkr in the repository with no flags.
   - Linkr attempts to find a .linkr file in the repository.
   - Linkr parses the configuration, exiting if it is malformed.
   - Linkr attempts to perform each action described in the file.
   - Linkr copies the .linkr file to .linkr_current.

2. A user wishes to update their dotfile repository on their device.

   - User pulls new changes into their dotfile repo/updates files and configurations.
   - User runs Linkr in the repository with the "Update" flag.
   - Linkr attempts to parse the .linkr_current file to discover the currently in-use configuration.
   - Link parses the .linkr file in the repository.
   - Linkr determines any changes that need to occur between the current and new configurations.
   - Linkr makes changes to bring the current configuration up to date.
   - Linkr copies the .linkr file to .linkr_current.

3. A user wishes to remove their dotfile repository from their device.

   - User runs Linkr in the repository with the "remove" flag.
   - Linkr attempts to parse the .linkr_current file to discover the currently in-use configuration.
   - Linkr removes all symlinks and files related to the current configuration.
   - Linkr deletes the .linkr_current file if it exists.

## Possible Failures

- Not enough permissions to perform the task.
- File already exists.
- Symlink already exists.
- Incorrect YAML syntax.
- Incorrect/Missing properties from 'Modules'.

## Feature Thoughts

- CLI flag for selecting which directory to run from.
- CLI flag for selecting which file to read.
- CLI flag for immediately exiting on any failure. (Copy failed, for example)
- CLI flag for auto-accepting overwrites (Copy only).
- Ansible like syntax with "Modules" for directories, symlinks, copy + paste, etc.

## Requirements

- MUST be able to read/parse a file with YAML syntax.
- MUST be able to use system calls to perform symlinks.
- MUST be able to use system calls to perform file copies.
- MUST handle permission errors when working without SUDO.
- MUST have a pretty printed output when performing tasks.
- MUST have a quiet flag to suppress pretty printing output.

## General Thoughts

We can parse the YAML syntax and build up an in-memory representation of the actions to perform, then perform them.
This would give us the ability to reject malformed actions before taking ANY actions, which is good at protecting user configurations from being partially initialised.
