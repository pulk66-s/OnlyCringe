sudo systemctl stop mysql
add_sql() {
    cat db/schemas/$1 >> docker/db/docker-entrypoint-initdb.d/create.sql
}

echo "cleaning all"
rm -r server/target
rm -r api/target
rm -r docker/back/server
rm -r db/docker-entrypoint-initdb.d/*.sql
rsync -av --exclude=".*" server docker/back
rsync -av --exclude=".*" imgServer docker/img

echo "copy all files to docker"
cp db/schemas/init.sql docker/db/docker-entrypoint-initdb.d/create.sql
add_sql create/user.sql
add_sql create/forum.sql
add_sql create/chat.sql
add_sql create/subs.sql
add_sql create/friend.sql

echo "Go to docker folder"
cd docker
sudo docker-compose build
sudo docker-compose down
sudo docker-compose up