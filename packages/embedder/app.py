# app.py
from flask import Flask, request, jsonify
import torch
import torch.nn.functional as F
from PIL import Image
from transformers import CLIPProcessor, CLIPModel

app = Flask(__name__)

# Load CLIP model and processor
model = CLIPModel.from_pretrained("openai/clip-vit-base-patch32")
processor = CLIPProcessor.from_pretrained("openai/clip-vit-base-patch32")

def cosine_similarity(a, b):
    """Compute cosine similarity between two vectors."""
    a_tensor = torch.tensor(a)
    b_tensor = torch.tensor(b)
    return float(F.cosine_similarity(a_tensor.unsqueeze(0), b_tensor.unsqueeze(0)))

@app.route('/embed', methods=['POST'])
def create_embeddings():
    # Validate inputs
    if (len(request.files) != 2 or 
        'text1' not in request.form or 
        'text2' not in request.form):
        return jsonify({"error": "Requires 2 images and 2 texts"}), 400

    # Process images
    images = [
        Image.open(request.files[f'image{i}']) 
        for i in range(1, 3)
    ]

    # Get texts
    texts = [
        request.form[f'text{i}'] 
        for i in range(1, 3)
    ]

    # Prepare inputs
    inputs = processor(text=texts, images=images, return_tensors="pt", padding=True)

    # Generate embeddings
    with torch.no_grad():
        outputs = model(**inputs)
    
    # Extract embeddings
    text_embeds = outputs.text_embeds.numpy().tolist()
    image_embeds = outputs.image_embeds.numpy().tolist()

    # Compute similarities
    text_similarity = cosine_similarity(text_embeds[0], text_embeds[1])
    image_similarity = cosine_similarity(image_embeds[0], image_embeds[1])

    return jsonify({
        "text1_embedding": text_embeds[0],
        "text2_embedding": text_embeds[1],
        "image1_embedding": image_embeds[0],
        "image2_embedding": image_embeds[1],
        "text_similarity": text_similarity,
        "image_similarity": image_similarity
    })

if __name__ == '__main__':
    app.run(debug=True)