# PasskeyManager

A learning project to explore encryption, password management, and passkeys by building a local desktop vault.

## Learning Goals

- Learn Rust and build a desktop user interface.
- Understand hashing, encryption, key derivation, and digital signatures.
- Use established cryptographic libraries to protect stored credentials.
- Learn key management, vault locking, and encrypted backups.
- Explore WebAuthn and how passkeys are created and used for authentication.
- Practice testing, documentation, and incremental development with Git.

## Project Scope

The first milestone is a local password vault with a graphical interface. Planned features include:

- Creating a vault and unlocking it with a master password.
- Encrypting stored credentials and decrypting them after unlocking.
- Adding, editing, deleting, and searching password entries.
- Generating passwords and copying them to the clipboard.
- Automatically locking the vault after inactivity.
- Creating encrypted backups and changing the master password.

Passkey support is a later milestone. It will begin with a WebAuthn test application, followed by investigating how the vault could act as a passkey provider through browser or operating system integration. Storing a key alone does not provide working passkey authentication.

## Planned Technology

- **Language:** Rust
- **Desktop UI:** egui / eframe
- **Storage:** Local encrypted vault file

## Status

Planning and early development. Features listed above are planned, not yet implemented or verified. This is an educational project, not a production-ready credential manager.
