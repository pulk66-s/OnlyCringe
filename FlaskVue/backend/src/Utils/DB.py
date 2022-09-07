import mysql.connector
from dotenv import load_dotenv
import os

class DB:

    def __init__(self, dictionary=True, buffered=True):
        load_dotenv()
        self.dictionary = dictionary
        self.buffered = buffered
        self.__connect()

    def __connect(self):
        self.mydb = mysql.connector.connect(
            host=os.getenv("DB_HOST"),
            user=os.getenv("DB_USER"),
            password=os.getenv("DB_PASSWORD"),
            database=os.getenv("DB_NAME"),
            auth_plugin="mysql_native_password"
        )
        self.cx = self.mydb.cursor(buffered=self.buffered, dictionary=self.dictionary)

    def get(self, req):
        self.__connect()
        self.cx.execute(req)
        return self.cx.fetchall()
    
    def post(self, req):
        try:
            self.__connect()
            self.cx.execute(req)
            self.mydb.commit()
            return True
        except:
            return False
