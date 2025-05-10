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

# For deploying container to GCE
module "gce-container" {
  source = "terraform-google-modules/container-vm/google"
  version = "~> 3.2"

  container = {
    image = "${var.backend_image}:${var.backend_image_tag}"
    env = [
      for key, value in var.backend_env_vars : {
        name  = key
        value = value
      }
    ]
  }
}

# Compute Engine for backend service
resource "google_compute_instance" "backend" {
  name = "itemtracker-backend"
  zone = "asia-southeast1-a"
  machine_type = "e2-micro"
  allow_stopping_for_update = true

  boot_disk {
    initialize_params {
      image = module.gce-container.source_image
    }
  }

  network_interface {
    network = "default"
    access_config {
      # ephemeral public IP
    }
  }

  lifecycle {
    # Ensures that new image tags are deployed (?)
    create_before_destroy = true
  }

  metadata = {
    gce-container-declaration = module.gce-container.metadata_value
    google-logging-enabled = "true"
    google-monitoring-enabled = "true"

    # Installs nginx + SSL cert for serving HTTPS requests
    startup-script = <<-EOT
      #!/bin/bash
      apt update
      apt upgrade -y
      apt install certbot python3-certbot-nginx
      certbot --nginx -d ${var.backend_domain}
      EOT
  }

  labels = {
    container-vm = module.gce-container.vm_container_label
    image-tag = var.backend_image_tag
  }
}

# Allow HTTP traffic for backend GCE
resource "google_compute_firewall" "backend" {
  name = "backend-allow-http-traffic"
  network = "default"

  allow {
    ports = ["80", "443"]
    protocol = "tcp"
  }

  source_ranges = ["0.0.0.0/0"]
}

# Cloud DNS for backend service
resource "google_dns_managed_zone" "backend" {
  name = "backend-dns-zone"
  dns_name = "${var.backend_domain}." # DNS name must end with a dot
  description = "DNS zone for the itemtracker backend"
  force_destroy = "true"
}

# Register backend service's IP in DNS
resource "google_dns_record_set" "backend" {
  name = google_dns_managed_zone.backend.dns_name
  managed_zone = google_dns_managed_zone.backend.name
  type = "A"
  ttl = 300

  rrdatas = [ 
    google_compute_instance.backend.network_interface[0].access_config[0].nat_ip
  ]
}

# Cloud Run service for embedder service
resource "google_cloud_run_service" "embedder" {
  name     = "itemtracker-embedder"
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

resource "google_cloud_run_service_iam_member" "embedder_public" {
  location = google_cloud_run_service.embedder.location
  service  = google_cloud_run_service.embedder.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}