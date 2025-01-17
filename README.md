# ItemTracker
## Info
This is an end-to-end SaaS for facilitating the scraping, analysis and classification of item listings across different marketplaces.

## Running locally
Run the below services in separate terminals.

### Backend
You need Rust installed:
```Powershell
cd packages/monolith
cargo run --release
```

### Scraper
You need Python + a virtual env and package manager installed. We use `uv` but the same thing can be accomplished with `venv` + `pip`:
```Powershell
cd packages/scraper
uv venv # This only needs to be ran once, to instantiate the virtual env
.venv\Scripts\activate
uv pip install requirements.txt # This only needs to be ran whenever there are changes to dependencies 
scrapyd
```

### Frontend
WIP

### Image Classifier
WIP