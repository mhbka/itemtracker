from flask import Flask, request, jsonify
import torch
from PIL import Image
from transformers import CLIPProcessor, CLIPModel

app = Flask(__name__)

# Load CLIP model and processor
model = CLIPModel.from_pretrained("openai/clip-vit-base-patch32")
processor = CLIPProcessor.from_pretrained("openai/clip-vit-base-patch32")

@app.route('/embed', methods=['POST'])
def create_embeddings():
    # Check if image and text are provided
    if 'image' not in request.files or 'text' not in request.form:
        return jsonify({"error": "Missing image or text"}), 400

    # Process image
    image_file = request.files['image']
    image = Image.open(image_file)

    # Get text
    text = request.form['text']

    # Prepare inputs
    inputs = processor(text=[text], images=image, return_tensors="pt", padding=True)

    # Generate embeddings
    with torch.no_grad():
        outputs = model(**inputs)
    
    # Extract embeddings
    text_embeds = outputs.text_embeds.numpy().tolist()
    image_embeds = outputs.image_embeds.numpy().tolist()

    return jsonify({
        "text_embedding": text_embeds[0],
        "image_embedding": image_embeds[0]
    })

if __name__ == '__main__':
    app.run(debug=True)