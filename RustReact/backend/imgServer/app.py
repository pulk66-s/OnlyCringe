from crypt import methods
from flask import Flask, request
from flask_cors import CORS

from Classes.Profile import ProfileImg

app = Flask(__name__)
CORS(app)

profileImg = ProfileImg()

@app.route("/api/ping")
def ping():
    return "pong"

@app.route("/api/image/profile/<uuid(strict=False):uuid>", methods=["GET", "POST", "DELETE"])
def profile_by_uuid(uuid):
    if request.method == "GET":
        return profileImg.get(str(uuid))
    elif request.method == "POST":
        return profileImg.post(str(uuid), request.data)
    elif request.method == "DELETE":
        return profileImg.delete(str(uuid))
