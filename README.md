# ItemTracker
## Info
This is an SaaS for facilitating the scraping, analysis and classification of item listings across different marketplaces.

## Running locally
### Supabase + Google OAuth
This app uses Supabase + Google OAuth for authentication. [Follow the steps here](https://supabase.com/docs/guides/auth/social-login/auth-google?queryGroups=framework&framework=nextjs)
to get these set up.

### Environment variables
WIP

### Backend
You need Rust installed:
```Powershell
cd packages/monolith
cargo run --release
```

### Embedder
You need Python + conda installed:
```Powershell
cd packages/image_classifier
conda env create environment.yml
```

### Frontend
You need npm + nodeJS installed:
```Powershell
cd packages/frontend
npm install
npm run dev
```

