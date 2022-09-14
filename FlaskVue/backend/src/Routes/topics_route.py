from flask import Blueprint, request

from Services.Topic import Topic as TopicService
from Utils.Decorator.Json import json_response

topics = Blueprint('topics', __name__, template_folder='templates')

topicService = TopicService()

@topics.route("/api/topics", methods = ["GET", "POST"])
@json_response
def get_topics_route():
    if request.method == "GET":
        return topicService.get()
    elif request.method == "POST":
        return topicService.create(request.json)