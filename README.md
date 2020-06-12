# node-manager
Manager tool for sky-mavis-chain node

Usage:

* Initiate the `.env` file
```shell script
cp config/[test|main].env .env
```
* Then open `.env` file and file in remaining variables
* Start services. You might need to log out and log back in after install.
```shell script
./node-manager install
./node-manager start
```

* To stop all services
```shell script
./node-maanger stop
```
* To stop and clean all data, including chain data and oracle data. 
Note this action is irreversible
```shell script
./node-maanger wipe
```
