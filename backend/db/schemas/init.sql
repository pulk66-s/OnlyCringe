create database if not exists OnlyCringes;
CREATE USER 'OCUser'@'%' IDENTIFIED BY 'ççOCUs3r_p@ssw0rd__';
GRANT ALL PRIVILEGES ON OnlyCringes.* TO 'OCUser'@'%';
