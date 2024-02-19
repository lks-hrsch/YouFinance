# YouFinance

This project aims to provide a simple and secure way to manage your finances. It allows you to have control over where your data flows and how it is stored. The project is built using Tauri, Rust, and Typescript.

## Setup

### Tools Used

- [Rust](https://www.rust-lang.org/)
- [Tauri](https://tauri.app/)
- [Node.js](https://nodejs.org/)
- [Typescript](https://www.typescriptlang.org/)
- [Yarn](https://yarnpkg.com/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Material Tailwind](https://www.material-tailwind.com/)
- [Heroicons](https://heroicons.com/)
- [Typeshare](https://github.com/1password/typeshare)

### Development

This project utilizes Typeshare to generate TypeScript type definitions from the Rust implementation. You can find more information about Typeshare [here](https://github.com/1Password/typeshare).

To generate the TypeScript type definitions, run the following command:

```bash
typeshare . --lang=typescript --output-file=src/models/typeshare_definitions.ts
```

For rapid frontend development, we utilize Material Tailwind, which can be found at https://www.material-tailwind.com/.

To run the project in a development environment, execute the following command:

```bash
yarn tauri dev
```

To apply diesel migrations, run the following command:

```bash
cd src-tauri
diesel migration run --database-url <path/to/database>
```

to generate a new migration, run the following command:

```bash
cd src-tauri
diesel migration generate <migration_name>
```

To reapply the migrations, run the following command:

```bash
cd src-tauri
diesel migration redo --database-url <path/to/database> --number <depth>
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
