sudo systemctl stop mysql
add_sql() {
    cat backend/db/schemas/$1 >> docker/db/docker-entrypoint-initdb.d/create.sql
}

echo "cleaning all"
rm -r backend/server/target
rm -r backend/api/target
rm -r backend/docker/back/server
rm -r backend/db/docker-entrypoint-initdb.d/*.sql
rsync -av --exclude=".*" backend/server docker/back
rsync -av --exclude=".*" backend/imgServer docker/img

echo "copy all files to docker"
cp backend/db/schemas/init.sql docker/db/docker-entrypoint-initdb.d/create.sql
add_sql create/user.sql
add_sql create/forum.sql
add_sql create/chat.sql
add_sql create/subs.sql
add_sql create/friend.sql
mkdir docker/front/client
cp -r frontend/src docker/front/client/src
cp -r frontend/public docker/front/client/public
cp -r frontend/package.json docker/front/client/
cp frontend/jsconfig.json docker/front/client/

echo "Go to docker folder"
cd docker
sudo docker-compose build
sudo docker-compose down
sudo docker-compose up