# Project-wide settings
project_id = "itemtracker-456913"
region = "asia-southeast1"

# Backend configs
backend_service_name = "itemtracker-backend"
backend_image = "mhish/itemtracker_backend"
backend_image_tag    = "latest"
backend_domain = "api.itemtracker.hish.dev"
backend_cpu          = "1000m"
backend_memory       = "512Mi"
backend_env_vars     = {
  # TODO: anything needed here?
}

# Embedder configs
embedder_service_name = "itemtracker-embedder"
embedder_image = "mhish/itemtracker_embedder"
embedder_image_tag    = "latest"
embedder_domain = "embedder.itemtracker.hish.dev"
embedder_cpu          = "1000m"
embedder_memory       = "1Gi"
embedder_env_vars     = {
  # TODO: anything needed here?
}