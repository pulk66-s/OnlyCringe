if [[ (-z "$1") || (-z "$2") ]]
then
    echo "Usage: ./install.sh <username> <password>"
    exit 1
else
    if [ "$1" = "-h" ]
    then
        echo "USAGE:"
        echo "  ./scripts/create_db.sh [USER] [PWD]"
        echo ""
        echo "USER=db user"
        echo "PWD=db password"
    else
        echo "Connecting with user command: "
        echo "mysql -u $1 -p$2 < ./schemas/create.sql"
        echo "Creating database..."
        sudo mysql -u $1 -p$2 < ./schemas/create.sql
    fi
fi

echo "installing pip3 packages..."
pip3 install -r requirements.txt
