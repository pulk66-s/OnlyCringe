echo "DELETING of old frontend folder"
sudo rm -rf /var/www/onlycringe/frontend
echo "copying folder from home to /var/www/onlycringe"
sudo cp -r ~/frontend /var/www/onlycringe
echo "changing dir"
cd /var/www/onlycringe/frontend
echo "installing"
sudo npm install
echo "restart onlycringeFrontend.service"
sudo systemctl restart onlycringeFrontend.service
echo "DONE, status:"
sudo systemctl status onlycringeFrontend.service
