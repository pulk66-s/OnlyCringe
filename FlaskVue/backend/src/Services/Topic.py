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

    def parse(self, data):
        if type(data) == dict:
            return self.__parse_from_dict(data)
        elif type(data) == list and len(data) > 0 and type(data[0]) == dict:
            return [self.__parse_from_dict(i) for i in data]
        elif type(data) == list:
            return []
        else:
            return {}

    def get(self):
        req = "select * from Topics"
        res = self.db.get(req)
        return self.parse(res)