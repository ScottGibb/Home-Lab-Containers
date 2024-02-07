#!/bin/sh
/etc/init.d/mariadb start
mysql -u root --password="" -e "source /mysql_scripts/mySqlScript.sql"
/etc/init.d/mariadb start && tail -f /dev/null
