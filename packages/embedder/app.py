import os
import torch
from flask import Flask, jsonify, request
from PIL import Image
from transformers import CLIPModel, CLIPProcessor

app = Flask(__name__)
model = CLIPModel.from_pretrained("openai/clip-vit-base-patch32")
processor = CLIPProcessor.from_pretrained("openai/clip-vit-base-patch32")

@app.route("/embed", methods=["POST"])
def create_embeddings():
    texts = request.form.getlist("text")
    images = []

    for file in request.files.getlist("image"):
        try:
            images.append(Image.open(file))
        except Exception as e:
            return jsonify({"error": f"Invalid image file: {str(e)}"}), 400

    if not texts and not images:
        return jsonify({"error": "Requires at least one text or image input"}), 400

    inputs = processor(
        text=texts if texts else None,
        images=images if images else None,
        return_tensors="pt",
        padding=True,
    )

    with torch.no_grad():
        outputs = model(**inputs)

    response = {}
    if texts:
        response["text_embeddings"] = outputs.text_embeds.numpy().tolist()
    if images:
        response["image_embeddings"] = outputs.image_embeds.numpy().tolist()

    return jsonify(response)
