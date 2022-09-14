from flask import Flask, jsonify, request
from flask_cors import CORS

from Routes.topics_route import topics
from Routes.user_route import users

app = Flask(__name__)
app.register_blueprint(topics)
app.register_blueprint(users)
CORS(app)

@app.route("/api")
def index_route():
    return "Hello World"
