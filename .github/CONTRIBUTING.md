# Contributing Guide

Thank you very much for your interest in and contribution to BongoCat! Before submitting your contribution, please take some time to read the following guidelines to ensure that your contribution can proceed smoothly.

## Transparent Development

All work is conducted publicly on GitHub. Whether it is a Pull Request from a core team member or an external contributor, all go through the same review process.

## Submitting Issues

We use [Github Issues](https://github.com/ayangweb/BongoCat/issues) for bug reports and new feature suggestions. Before creating an issue, please make sure you have searched for similar issues, as they may have already been resolved or are currently being fixed. For bug reports, please include the complete steps to reproduce the problem. For new feature suggestions, please indicate the changes you want and the expected behavior.

## Submitting Pull Requests

### Collaboration Process

- Claim an issue: Create an issue on GitHub and claim it (or claim an existing issue directly), so everyone knows you are working on the fix and to avoid duplicate work.
- Develop: After completing the preparation work, perform bug fixes or feature development.
- Submit a PR.

### Prerequisites

- [Rust](https://v2.tauri.app/start/prerequisites/): Please install the Rust environment yourself according to the official website instructions.
- [Node.js](https://nodejs.org/en/): Required to run the project.
- [Pnpm](https://pnpm.io/): This project uses Pnpm for package management.

### Install Dependencies

```shell
pnpm install
```

### Run the Application

```shell
pnpm tauri dev
```

### Build the Application

> If you need to build for debugging, add `--debug` after the following command

```shell
pnpm tauri build
```

## Commit Guide

Commit messages should follow the [conventional-changelog standard](https://www.conventionalcommits.org/en/v1.0.0/).

### Commit Types

The following is a list of commit types:

- feat: New features or functionality
- fix: Bug fixes
- docs: Documentation updates
- style: Code style updates
- refactor: Code refactoring, without introducing new features or bug fixes
- perf: Performance optimizations
- chore: Other commits

We look forward to your participation and making BongoCat even better together!
