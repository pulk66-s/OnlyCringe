import mysql.connector

class Database:

    def __init__(self, host, user, password, database):
        self.host = host
        self.user = user
        self.password = password
        self.database = database
        self.connect(self.host, self.user, self.password, self.database)

    def connect(self, host, user, password, database):
        self.mydb = mysql.connector.connect(
            host=host,
            user=user,
            password=password,
            database=database,
            auth_plugin="mysql_native_password"
        )
        self.cursor = self.mydb.cursor(buffered=True)

    def get(self, req):
        self.cursor.execute(req)
        return self.cursor.fetchall()

    def post(self, req):
        self.cursor.execute(req)
        return self.mydb.commit()
