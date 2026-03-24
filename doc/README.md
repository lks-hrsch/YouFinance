# YouFinance Feature Specifications

Welcome to the YouFinance documentation directory. This folder contains structured markdown specifications that outline requirements, database schemas, and external data parsing patterns for key backend features in the application.

## 📖 Feature Index

- **[Loading CSV Data into Database](./features/load-csv-data-into-database.md)**
  - Outlines the structural requirements and data mapping necessary to deserialize standard comma-separated German banking exports into normalized backend structures. Highlights expected standard columns and empty field coercion.

- **[Loading MTA (MT940) Data into Database](./features/load-mta-data-into-database.md)**
  - A highly detailed specification for parsing standard `.mta` / `MT940` formats. Includes structured details detailing parser mappings for (`:61:`, `:86:`) markers, block-level configurations, and subfield (`?2x`) continuity rules. Addresses critical structural variations such as multiple-transaction blocks and fragmented remittance concatenation logic.

## 📝 Documenting New Features

When expanding YouFinance structures, please create a dedicated markdown document within the `features/` subdirectory and formally link it here to maintain an accessible, high-level project map.
