from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives.asymmetric import ec
import base64
import jwt
import time
import uuid

# Generates a private key. Kept separate as it is more computationally expensive, so only should be called once per scraping session.
def generate_private_key():
    return ec.generate_private_key(ec.SECP256R1(), default_backend())

# Generates a valid Dpop token, required for accessing Mercari APIs.
def generate_dpop(private_key: ec.EllipticCurvePrivateKey, accessed_url: str, http_method: str):
    public_key = private_key.public_key()

    public_numbers = public_key.public_numbers()
    x = public_numbers.x
    y = public_numbers.y

    x_b64 = base64.urlsafe_b64encode(x.to_bytes(32, 'big')).decode('utf-8').rstrip('=')
    y_b64 = base64.urlsafe_b64encode(y.to_bytes(32, 'big')).decode('utf-8').rstrip('=')

    jwk = { # Essentially the public key, used by server to decrypt signature
        "crv": "P-256",
        "kty": "EC",
        "x": x_b64,
        "y": y_b64
    }

    payload = {
        "iat": int(time.time()),  # Issued at time
        "jti": str(uuid.uuid4()),  # Unique identifier for the token
        "htu": accessed_url,
        "htm": http_method,
        "uuid": str(uuid.uuid4()),  # Another unique identifier (thought this would depend on URL, but guess not)
    }

    header = {
        "typ": "dpop+jwt",
        "alg": "ES256",
        "jwk": jwk 
    }

    jwt_token = jwt.encode(payload, private_key, algorithm='ES256', headers=header)
    return jwt_token