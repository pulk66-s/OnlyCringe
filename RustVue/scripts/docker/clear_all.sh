sudo docker stop $(sudo docker ps -a -q)
sudo docker rm $(sudo docker ps -a -q)
sudo docker rmi -f $(sudo docker images -a -q)

rm -r docker/back/server
rm -r docker/img/imgServer
rm -r docker/db/docker-entrypoint-initdb.d/*.sql