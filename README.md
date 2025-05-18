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
Some variables in the examples are pre-filled (but can be updated), whilst others must be filled out by you:
- `ANTHROPIC_API_KEY`/`OPENAI_API_KEY`: API keys from both services (OpenAI's can be left as a dummy value as it isn't used at the moment)
- `DATABASE_URL`: The Postgres DB connection string (must include `?gssencmode=disable` at the end)
- `JWT_SECRET`: The JWT secret used for decoding Supabase JWTs (found in Supabase's Settings > API)

Similarly, the frontend example is `.env.local.example`; remove the `.example` and fill out the variables to use it:
- `VITE_SUPABASE_URL`: The Supabase connection URL
- `VITE_SUPABASE_ANON_KEY`: The Supabase API key

### Backend
You need Rust + [Diesel](https://diesel.rs/) installed:
```Powershell
cd packages/monolith
diesel migration run
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
### CI/CD
A fairly standard setup is used:
- The backend/embedder Github Action is triggered
- Backend/embedder is validated, then built into Docker images and pushed to Docker Hub
- Terraform applies its config

The frontend is deployed differently. It is simply built and deployed to Github Pages, via Github Actions.
Its build artifacts sit on the `gh-pages` branch.

Each of the deployment flows can be viewed via its workflow in `/.github/workflows`.

### Backend
The backend is deployed to Compute Engine, and uses Cloud DNS for mapping the domain to the instance.

The first time this is deployed, some additional DNS setup is required.
Go to the Cloud DNS console -> the backend zone -> *Registrar setup*; add these NS records for the backend (sub)domain.

### Embedder
The embedder is deployed to Cloud Run + Cloud Run Domain Mapping for mapping its domain.

The first time Terraform successfully deploys this, some additional DNS setup is required to properly map the domains.
Go to the Cloud Run console -> the service instance -> *Networking* -> *its custom URL*; add these DNS records for the embedder (sub)domain.

### Secrets
The following secrets are needed by Github Actions:
- `BACKEND_DOMAIN_URL`/`EMBEDDER_DOMAIN_URL` - (sub)domains to be mapped to the backend/embedder services (for setting them up, see *Service domains* below)
- `DOCKER_HUB_USERNAME`/`DOCKER_HUB_PAT` - Username and Personal Access Token for pushing/pulling service images to Docker Hub
- `GCP_PROJECT_ID`/`GCP_SERVICE_ACCOUNT_CREDENTIALS` - GCP project ID to deploy services under + service account credentials to access it
- `ANTHROPIC_API_KEY`/`OPENAI_API_KEY` - API keys for Anthropic and OpenAI, for LLM analysis
- `DATABASE_URL` - URL to the database (must include the `gssencode=disable` option)
- `JWT_SECRET` - Secret used by Supabase for signing auth JWTs

### GCP service account
Note that the GCP service account requires the *Owner* role to properly deploy.

### Service domains
Before being able to map any (sub)domains to the services, you must verify the domain [under GCP](https://www.google.com/webmasters/verification/verification). 

Afterwards, 

