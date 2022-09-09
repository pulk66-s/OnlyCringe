FROM mariadb:latest

COPY config/my.cnf /etc/mysql/ 
COPY docker-entrypoint-initdb.d/* /docker-entrypoint-initdb.d/
