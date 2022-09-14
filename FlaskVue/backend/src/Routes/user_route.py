from flask import Blueprint, request

from Utils.Decorator.Json import json_response
from Utils.Decorator.Token import auth_token_gen
from Utils.Decorator.Login import login_required

from Services.User import User as UserService

users = Blueprint("users_route", __name__, template_folder='templates')

userService = UserService()

@users.route("/api/user", methods = ["GET"])
@json_response
def get_user_route_get():
    if request.method == "GET":
        return userService.get()

@users.route("/api/user/<string:name>", methods = ["GET"])
@json_response
def get_user_route_by_name(name):
    if request.method == "GET":
        return userService.get(name=name)

@users.route("/api/user/<string:name>", methods = ["PUT", "DELETE"])
@json_response
@login_required
def put_delete_route_by_name(name):
    if request.method == "PUT":
        return userService.update(data=request.json, name=name)
    elif request.method == "DELETE":
        return userService.delete(name=name)

@users.route("/api/user", methods = ["POST"])
@json_response
@auth_token_gen
def get_user_route_post():
    if request.method == "POST":
        return userService.create(request.json)

@users.route("/api/user/login", methods = ["POST"])
@json_response
@auth_token_gen
def login_user_route():
    if request.method == "POST":
        return userService.login(request.json)
