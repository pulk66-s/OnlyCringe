sudo rm -rf /var/www/onlycringe/frontend
sudo cp -r ~/frontend /var/www/onlycringe
cd /var/www/onlycringe/frontend
sudo npm install
sudo systemctl restart onlycringeFrontend.service