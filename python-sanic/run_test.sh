#!bin/bash

clear
docker-compose up -d db
sleep 15
docker-compose up -d api
sleep 15
docker-compose up wrk >> wrk.txt
sleep 15
docker-compose up wrk >> wrk.txt
sleep 15
docker-compose up wrk >> wrk.txt
docker-compose down
echo "Perf test is now complete."
