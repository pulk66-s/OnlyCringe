#!/bin/bash

sudo systemctl start mysql

# $1 = dbUser
# $2 = dbPassword

createSchema() {
    mysql -u $1 -p$2 < $3
}

if [[ (-z "$1") || (-z "$2") ]]
then
    echo "missing db username parameter or db password"
else
    if [ "$1" = "-h" ]
    then
        echo "USAGE:"
        echo "  ./scripts/create_db.sh [USER] [PWD]"
        echo ""
        echo "USER=db user"
        echo "PWD=db password"
    else
        echo "delete all databases"
        createSchema $1 $2 ./db/schemas/delete_all.sql
        echo "initialization of all databases"
        createSchema $1 $2 ./db/schemas/init.sql
        echo "creation of user tables"
        createSchema $1 $2 ./db/schemas/create/user.sql
        echo "creation of forum tables"
        createSchema $1 $2 ./db/schemas/create/forum.sql
        echo "creation of chat tables"
        createSchema $1 $2 ./db/schemas/create/chat.sql
        echo "creation of subs table"
        createSchema $1 $2 ./db/schemas/create/subs.sql
        echo "creation of friend table"
        createSchema $1 $2 ./db/schemas/create/friend.sql
    fi
fi
