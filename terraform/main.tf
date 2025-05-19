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

resource "google_project_service" "compute_api" {
  service            = "compute.googleapis.com"
  disable_on_destroy = false
}

# For deploying container to GCE
module "gce-container" {
  source  = "terraform-google-modules/container-vm/google"
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
  name = "itemtracker-backend-${replace(var.backend_image_tag, ".", "-")}"
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
    # Ensures that new image tags are deployed
    create_before_destroy = true
  }

  metadata = {
    gce-container-declaration = module.gce-container.metadata_value
    google-logging-enabled    = "true"
    google-monitoring-enabled = "true"
  }

  labels = {
    container-vm = module.gce-container.vm_container_label
    image-tag    = var.backend_image_tag
  }

  depends_on = [google_project_service.compute_api]
}

# Create an instance group for the backend service
resource "google_compute_instance_group" "backend" {
  name      = "itemtracker-backend-group"
  zone      = "asia-southeast1-a"
  instances = [google_compute_instance.backend.id]

  named_port {
    name = "http"
    port = "80"
  }
}

# Reserve a static IP address for the load balancer
resource "google_compute_global_address" "backend_lb_ip" {
  name = "itemtracker-backend-lb-ip"
}

# Create the managed SSL certificate
resource "google_compute_managed_ssl_certificate" "backend" {
  name = "itemtracker-backend-ssl-cert"

  managed {
    domains = [var.backend_domain]
  }
}

# Create health check for the backend service
resource "google_compute_health_check" "backend" {
  name = "itemtracker-backend-health-check"

  http_health_check {
    port = 80
    request_path = "/health"  # Adjust based on your health check endpoint
  }
}

# Create backend service
resource "google_compute_backend_service" "backend" {
  name        = "itemtracker-backend-service"
  port_name   = "http"
  protocol    = "HTTP"
  timeout_sec = 10
  health_checks = [google_compute_health_check.backend.id]

  backend {
    group = google_compute_instance_group.backend.id
  }
}

# Create URL map
resource "google_compute_url_map" "backend" {
  name            = "itemtracker-backend-url-map"
  default_service = google_compute_backend_service.backend.id
}

# Create HTTPS target proxy
resource "google_compute_target_https_proxy" "backend" {
  name             = "itemtracker-backend-https-proxy"
  url_map          = google_compute_url_map.backend.id
  ssl_certificates = [google_compute_managed_ssl_certificate.backend.id]
}

# Create forwarding rule for HTTPS
resource "google_compute_global_forwarding_rule" "backend_https" {
  name       = "itemtracker-backend-https-rule"
  target     = google_compute_target_https_proxy.backend.id
  port_range = "443"
  ip_address = google_compute_global_address.backend_lb_ip.address
}

# Create HTTP target proxy (for redirect to HTTPS)
resource "google_compute_target_http_proxy" "backend" {
  name    = "itemtracker-backend-http-proxy"
  url_map = google_compute_url_map.backend_redirect.id
}

# Create URL map for HTTP to HTTPS redirect
resource "google_compute_url_map" "backend_redirect" {
  name = "itemtracker-backend-redirect"

  default_url_redirect {
    https_redirect         = true
    redirect_response_code = "MOVED_PERMANENTLY_DEFAULT"
    strip_query            = false
  }
}

# Create forwarding rule for HTTP to HTTPS redirect
resource "google_compute_global_forwarding_rule" "backend_http" {
  name       = "itemtracker-backend-http-rule"
  target     = google_compute_target_http_proxy.backend.id
  port_range = "80"
  ip_address = google_compute_global_address.backend_lb_ip.address
}

# Allow HTTP/HTTPS traffic for backend GCE
resource "google_compute_firewall" "backend" {
  name    = "backend-allow-http-traffic"
  network = "default"

  allow {
    ports    = ["80", "443"]
    protocol = "tcp"
  }

  source_ranges = ["0.0.0.0/0"]
}

# Allow health check traffic for the backend
resource "google_compute_firewall" "health_check" {
  name    = "allow-health-check"
  network = "default"

  allow {
    protocol = "tcp"
    ports    = ["80"]
  }

  # Allow traffic from GCP health check systems
  source_ranges = ["35.191.0.0/16", "130.211.0.0/22"]
}

# Cloud DNS for backend service
resource "google_dns_managed_zone" "backend" {
  name          = "backend-dns-zone"
  dns_name      = "${var.backend_domain}." # DNS name must end with a dot
  description   = "DNS zone for the itemtracker backend"
  force_destroy = "true"
}

# Register backend service's IP in DNS
resource "google_dns_record_set" "backend" {
  name         = google_dns_managed_zone.backend.dns_name
  managed_zone = google_dns_managed_zone.backend.name
  type         = "A"
  ttl          = 300

  rrdatas = [google_compute_global_address.backend_lb_ip.address]
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