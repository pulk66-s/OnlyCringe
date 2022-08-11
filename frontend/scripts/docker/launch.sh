mkdir docker/front/client
cp -r src docker/front/client/src
cp -r public docker/front/client/public
cp -r package.json docker/front/client/
cp jsconfig.json docker/front/client/

cd docker
sudo docker-compose build
sudo docker-compose down
sudo docker-compose up