terraform {
  backend "gcs" {
    # Run using backend.conf
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

# Enable required APIs
resource "google_project_service" "run_api" {
  service            = "run.googleapis.com"
  disable_on_destroy = false
}

resource "google_project_service" "iam_api" {
  service            = "iam.googleapis.com"
  disable_on_destroy = false
}

# Cloud Run service for main backend
resource "google_cloud_run_service" "backend" {
  name     = var.backend_service_name
  location = var.region

  template {
    spec {
      containers {
        image = "${var.backend_image}:${var.backend_image_tag}"
        
        resources {
          limits = {
            cpu    = var.backend_cpu
            memory = var.backend_memory
          }
        }
        
        dynamic "env" {
          for_each = var.backend_env_vars
          content {
            name  = env.key
            value = env.value
          }
        }
      }
    }
  }

  depends_on = [google_project_service.run_api]
}

# Cloud Run service for embedder service
resource "google_cloud_run_service" "embedder" {
  name     = var.embedder_service_name
  location = var.region

  template {
    spec {
      containers {
        image = "${var.embedder_image}:${var.embedder_image_tag}"
        
        resources {
          limits = {
            cpu    = var.embedder_cpu
            memory = var.embedder_memory
          }
        }
        
        dynamic "env" {
          for_each = var.embedder_env_vars
          content {
            name  = env.key
            value = env.value
          }
        }
      }
    }
  }

  depends_on = [google_project_service.run_api]
}

# IAM binding for making the services public
resource "google_cloud_run_service_iam_member" "backend_public" {
  location = google_cloud_run_service.backend.location
  service  = google_cloud_run_service.backend.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}

resource "google_cloud_run_service_iam_member" "embedder_public" {
  location = google_cloud_run_service.embedder.location
  service  = google_cloud_run_service.embedder.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}

# Outputs for service URLs
output "backend_url" {
  value = google_cloud_run_service.backend.status[0].url
}

output "embedder_url" {
  value = google_cloud_run_service.embedder.status[0].url
}