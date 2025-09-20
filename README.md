# Typing Test CLI

A terminal-based typing test built with Rust and Ratatui.

## Features

- Real-time character highlighting (green for correct, red for incorrect)
- Live progress tracking and statistics
- Real-time WPM calculation

## Usage

```bash
cargo run
```

### Controls

- **Enter** - Start typing test
- **Backspace** - Correct mistakes
- **r** - Restart test (after completion)
- **q** - Quit application

## Development Setup

### Pre-commit Hooks

Install and setup pre-commit hooks for code quality:

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install the hooks
pre-commit install
pre-commit install --hook-type commit-msg

# Set up commit message template
git config commit.template .gitmessage

# Test the setup
pre-commit run --all-files
```

### Code Quality

The project includes:
- **Rustfmt** - Code formatting
- **Clippy** - Linting with strict rules
- **Conventional Commits** - Commit message validation
- **File checks** - Trailing whitespace, large files, etc.

### Commit Convention

Use conventional commits format:
```
feat: add new feature
fix: resolve bug
docs: update documentation
style: format code
refactor: restructure code
test: add tests
chore: maintenance tasks
```
