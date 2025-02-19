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
    # Extract text inputs
    texts = request.form.getlist("text")
    images = []
    
    # Extract image inputs
    for file in request.files.getlist("image"):
        try:
            images.append(Image.open(file))
        except Exception as e:
            return jsonify({"error": f"Invalid image file: {str(e)}"}), 400
    
    # Ensure at least one text or image is provided
    if not texts and not images:
        return jsonify({"error": "Requires at least one text or image input"}), 400
    
    # Prepare inputs for CLIP
    inputs = processor(text=texts if texts else None, 
                       images=images if images else None, 
                       return_tensors="pt", 
                       padding=True)
    
    # Generate embeddings
    with torch.no_grad():
        outputs = model(**inputs)
    
    # Extract embeddings
    response = {}
    if texts:
        response["text_embeddings"] = outputs.text_embeds.numpy().tolist()
    if images:
        response["image_embeddings"] = outputs.image_embeds.numpy().tolist()
    
    return jsonify(response)

if __name__ == '__main__':
    app.run(debug=True)
