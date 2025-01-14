# ItemTracker
## Info
This is an end-to-end project for facilitating the scraping, analysis and classification of item listings across different marketplaces.

## Running locally
Run the below services in separate terminals.

### Backend
You need Rust installed:
```Powershell
cd packages/monolith
cargo run --release
```

### Scraper
You need Python + a virtual env and package manager installed. We use `uv` but the same things can be accomplished with `venv` + `pip`:
```Powershell
cd packages/scraper
uv venv
.venv\Scripts\activate
uv pip install requirements.txt
scrapyd
```

### Frontend
Not available yet

### Image Classifier
Not available yet