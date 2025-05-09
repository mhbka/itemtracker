## Used for mapping service domains to the services

# Enable the domains API
resource "google_project_service" "domains_api" {
  service            = "domains.googleapis.com"
  disable_on_destroy = false
}

# Verify domain ownership
resource "google_cloud_run_domain_mapping" "embedder_domain" {
  name     = var.embedder_domain
  location = var.region
  metadata {
    namespace = var.project_id
  }

  spec {
    route_name = google_cloud_run_service.embedder.name
  }

  depends_on = [
    google_cloud_run_service.embedder,
    google_project_service.domains_api
  ]
}