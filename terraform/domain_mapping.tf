## Used for mapping service domains to the services

# Enable the domains API
resource "google_project_service" "domains_api" {
  service            = "domains.googleapis.com"
  disable_on_destroy = false
}

# Verify domain ownership
resource "google_cloud_run_domain_mapping" "backend_domain" {
  name     = var.backend_domain
  location = var.region
  metadata {
    namespace = var.project_id
  }

  spec {
    route_name = google_cloud_run_service.backend.name
  }

  depends_on = [
    google_cloud_run_service.backend,
    google_project_service.domains_api
  ]
}

# Output the DNS verification details
output "domain_verification" {
  value = try(google_cloud_run_domain_mapping.backend_domain.status[0].resource_records, [])
  description = "DNS records to add to your domain's DNS configuration"
}