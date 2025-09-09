# Amazon Data Parser

A Rust tool for parsing and consolidating Amazon business data from PDF invoices and CSV files.

```bash
cp -r  ~/syncthing/projects/geld/steuern/2021/amazon ./data
cargo run
```

## Features

- **Invoice Processing**: Extracts totals from PDF invoices using regex pattern matching
- **Payment Data**: Merges multiple payment CSV files into a single consolidated file
- **Sales Data**: Consolidates sales CSV files from multiple sources

## Usage

```bash
cargo run
```

The tool processes data from:

- `data/invoices/` - PDF invoice files → `invoice_2021.csv`
- `data/payments/` - Payment TXT files → `payments_2021.csv`
- `data/sales/` - Sales TXT files → `sales_2021.csv`

## Dependencies

- PDF text extraction via `pdf-extract`
- CSV handling with `csv` crate
- Regex pattern matching for invoice totals

