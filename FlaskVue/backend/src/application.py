from flask import Flask, jsonify, request
from flask_cors import CORS

from Services.User import User as UserService
from Utils.JSON import JSON

app = Flask(__name__)
CORS(app)

userService = UserService()

@app.route("/api")
def index_route():
    return "Hello World"

@app.route("/api/user", methods = ["GET", "POST"])
def get_user_route():
    if request.method == "GET":
        return JSON.parse(userService.get())
    elif request.method == "POST":
        res, code = userService.create(request.json)
        return JSON.parse(res), code