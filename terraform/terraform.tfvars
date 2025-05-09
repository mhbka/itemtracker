## NOTE: all empty variables are (should be) applied as secrets in Github Actions
# Project-wide settings
project_id = ""
region = "asia-southeast1"

# Backend configs
backend_image = "mhish/itemtracker_monolith"
backend_image_tag    = "latest"
backend_domain = ""
backend_cpu          = "1000m"
backend_memory       = "512Mi"
backend_env_vars     = {
}

# Embedder configs
embedder_image = "mhish/itemtracker_embedder"
embedder_image_tag    = "latest"
embedder_domain = ""
embedder_cpu          = "1000m"
embedder_memory       = "1Gi"
embedder_env_vars     = {
}