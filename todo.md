# Todo

- Add
  - Accept secret from stdin (piped from clipboard for example)

- Delete
  - Delete an entry (with re-prompt ARE YOU REALLY SURE)

- Code / List / Default? (viewing all entries and their codes, interactive mode)
  - UI for viewing all 2FA codes generated in real time
  - Displaying of countdown/progress-bar for next code generation
  - Interactive with controls like scroll up/down with j/k, q to quit

- Probably silence help/usage output unless error is a result of improper usage (unknown option, etc.)

- Test on Windows (MacOS ✅ | Linux (Debian) ✅)

## Nice to have

- Accept secret from QR code. Could be from image file, or better yet from image in clipboard taken with area screenshot.

- Import from Google Authenticator export. See example bash script [import-gauth-json.sh](https://github.com/zzzachzzz/bash-otp/blob/master/import-gauth-json.sh) for importing an export from [krissrex/google-authenticator-exporter](https://github.com/krissrex/google-authenticator-exporter).

- Option to use encryption key from source besides keyring (password prompt or password supplied from stdin)

