# itemtracker
## Info
This is an SaaS for scraping, analyzing, filtering and (WIP) classification of item listings across different marketplaces.

## Running locally
### Supabase + Google OAuth
This app uses Supabase + Google OAuth for authentication and database. [Follow the steps here](https://supabase.com/docs/guides/auth/social-login/auth-google?queryGroups=framework&framework=nextjs)
to get these set up.

### Environment variables
The backend uses the `packages/monolith/.env` file for environment variables. 
An example is given as `.env.example`, which can be renamed to `.env` to be used.
Some variables in the examples are pre-filled but can be updated, whilst others must be filled out by you:
- `ANTHROPIC_API_KEY`/`OPENAI_API_KEY`: API keys from both services (OpenAI's can be left as a dummy value as it isn't used at the moment)
- `DATABASE_URL`: The Postgres DB connection string (must include `?gssencmode=disable` at the end)
- `JWT_SECRET`: The JWT secret used for decoding Supabase JWTs (found in Supabase's Settings > API)

Similarly, the frontend example is `.env.local.example`; remove the `.example` and fill out the variables to use it:
- `VITE_SUPABASE_URL`: The Supabase connection URL
- `VITE_SUPABASE_ANON_KEY`: The Supabase API key

### Backend
You need Rust installed:
```Powershell
cd packages/monolith
cargo run --release
```

### Embedder
You need Python + uv installed:
```Powershell
cd packages/embedder
uv run app.py
```

### Frontend
You need npm + nodeJS installed:
```Powershell
cd packages/frontend
npm install
npm run dev
```

## CI/CD and infra
### Setup
A fairly standard setup is used:
- Backend/embedder is validated, then built into a Docker image and pushed to Docker Hub
- Terraform applies the updated image to a Cloud Run instances + ties them to the appropriate subdomain
- All this is run by a Github Action when needed

### Secrets
WIP


