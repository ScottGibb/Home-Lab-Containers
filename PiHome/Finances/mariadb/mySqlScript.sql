CREATE DATABASE fireflyiiiDb;
create user fireflyiiiUser@'%' identified by 'fireflyiiiPassword';
grant all privileges on fireflyiiiDb.* to fireflyiiiUser@'%';
flush privileges;
quit