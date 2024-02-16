# YouFinance

This project aims to provide a simple and secure way to manage your finances. It lets you control where your data flows and how it is stored. It is built with Tauri, Rust, and Typescript.

# setup

## development

This project is using typeshare to create the typescript type definitions from the rust implementation. You can find more about it here https://github.com/1Password/typeshare.

```bash
typeshare . --lang=typescript --output-file=src/models/typeshare_definitions.ts
```

to get a faster start we are using https://www.material-tailwind.com/

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
