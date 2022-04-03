import PIL
import numpy as np
import torch
import base64
from flask import Flask, jsonify, request
from io import BytesIO

import process

app = Flask(__name__)

(G, device, label) = process.init_device()


@app.route("/", methods=['GET'])
def hello_world():
    req = request.args
    t = torch.from_numpy(np.random.RandomState(int(req['seed'])).randn(1, G.z_dim)).to(device)
    img = G(t, label, truncation_psi=1, noise_mode='const')
    img = (img.permute(0, 2, 3, 1) * 127.5 + 128).clamp(0, 255).to(torch.uint8)
    i = PIL.Image.fromarray(img[0].cpu().numpy(), 'RGB')

    buffered = BytesIO()
    i.save(buffered, format="JPEG")
    img_str = base64.b64encode(buffered.getvalue())

    response = jsonify({'image': img_str.decode("utf-8") })
    response.headers.add('Access-Control-Allow-Origin', '*')

    return response
