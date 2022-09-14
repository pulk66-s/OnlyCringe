from Utils.DB import DB
from Data.Topic import Topic as TopicModel

class Topic:

    def __init__(self):
        self.db = DB()

    def __parse_from_dict(self, res):
        return TopicModel(
            uuid=res["uuid"] if "uuid" in res else "",
            name=res["name"] if "name" in res else "",
            author_uuid=res["author_uuid"] if "author_uuid" in res else ""
        )

    def __check_topic(self, topic):
        fields = ["name", "author_uuid"]
        for field in fields:
            if topic.__dict__[field] == "":
                return False
        return True

    def parse(self, data):
        if type(data) == dict:
            return self.__parse_from_dict(data)
        elif type(data) == list and len(data) > 0 and type(data[0]) == dict:
            return [self.__parse_from_dict(i) for i in data]
        elif type(data) == list:
            return []
        else:
            return {}

    def get(self, name = None):
        if name is not None:
            req = f"select * from Topics where name='{name}'"
            return self.parse(self.db.get(req))
        req = "select * from Topics"
        res = self.db.get(req)
        return self.parse(res)

    def create(self, data):
        topic = self.parse(data)
        if not self.__check_topic(topic):
            raise Exception("Invalid topic, 400")
        req = f"insert into Topics (name, author_uuid) values ('{topic.name}', '{topic.author_uuid}')"
        res = self.db.post(req)
        if res is True:
            return self.get(name=topic.name)
        else:
            raise Exception("Topic name already exist, 400")