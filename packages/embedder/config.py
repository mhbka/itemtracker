import os

class Config:
    SECRET_KEY = os.environ.get("SECRET_KEY", "dev-key-change-in-production")
    
class ProductionConfig(Config):
    DEBUG = False
    TESTING = False