from flask import Flask, jsonify
from flask_cors import CORS

from Services.User import User as UserService

app = Flask(__name__)
CORS(app)

userService = UserService()

@app.route("/api")
def index_route():
    return "Hello World"

@app.route("/api/user")
def get_user_route():
    return jsonify(userService.get())