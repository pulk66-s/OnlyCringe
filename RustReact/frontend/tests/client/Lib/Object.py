from Lib.API import API
from Lib.Database import Database

class Object:

    def __init__(self):
        self.database = Database("localhost", "OCUser", "ççOCUs3r_p@ssw0rd__", "OnlyCringes")
        self.api = API("http://localhost:8901/api")
