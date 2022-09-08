from flask import Flask, jsonify, request
from flask_cors import CORS

from Services.User import User as UserService
from Utils.JSON import JSON
from Utils.Decorator.Json import json_response
from Utils.Decorator.Token import auth_token_gen

app = Flask(__name__)
CORS(app)

userService = UserService()

@app.route("/api")
def index_route():
    return "Hello World"

@app.route("/api/user", methods = ["GET", "POST"])
@json_response
def get_user_route():
    if request.method == "GET":
        return userService.get()
    elif request.method == "POST":
        return userService.create(request.json)

@app.route("/api/user/login", methods = ["POST"])
@json_response
@auth_token_gen
def login_user_route():
    if request.method == "POST":
        return userService.login(request.json)
