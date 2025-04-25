variable "gcs_bucket" {
  description = "The bucket storing the Terraform state"
  type = string
}

variable "project_id" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP Region to deploy services"
  type        = string
  default     = "us-central1"
}

# Backend service variables
variable "backend_service_name" {
  description = "Name for the backend Cloud Run service"
  type        = string
  default     = "itemtracker-backend"
}

variable "backend_image" {
    description = "Image for the backend"
    type = string
}

variable "backend_image_tag" {
  description = "Tag for the backend Docker image"
  type        = string
  default     = "latest"
}

variable "backend_domain" {
  description = "Domain for the backend service"
  type        = string
  default     = ""
}

variable "backend_cpu" {
  description = "CPU allocation for backend service"
  type        = string
  default     = "1000m" # 1 vCPU
}

variable "backend_memory" {
  description = "Memory allocation for backend service"
  type        = string
  default     = "512Mi" # 512MB
}

variable "backend_env_vars" {
  description = "Environment variables for backend service"
  type        = map(string)
  default     = {}
}

# Embedder service variables
variable "embedder_service_name" {
  description = "Name for the embedder Cloud Run service"
  type        = string
  default     = "itemtracker-embedder"
}

variable "embedder_image" {
  description = "Image for the embedder"
  type        = string
}

variable "embedder_image_tag" {
  description = "Tag for the embedder Docker image"
  type        = string
  default     = "latest"
}

variable "embedder_domain" {
  description = "Domain for the embedder service"
  type        = string
  default     = ""
}

variable "embedder_cpu" {
  description = "CPU allocation for embedder service"
  type        = string
  default     = "1000m" # 1 vCPU
}

variable "embedder_memory" {
  description = "Memory allocation for embedder service"
  type        = string
  default     = "1Gi" # 1GB - may need more for embedding
}

variable "embedder_env_vars" {
  description = "Environment variables for embedder service"
  type        = map(string)
  default     = {}
}