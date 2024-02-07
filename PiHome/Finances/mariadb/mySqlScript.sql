CREATE DATABASE fireflyiiiDb;
create user firefly@'%' identified by 'secret_firefly_password';
grant all privileges on fireflyiiiDb.* to firefly@'%';
flush privileges;
quit