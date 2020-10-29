#! /bin/bash
clear

echo -e "\n\nRust - Actix Web\n" 
wrk http://127.0.0.1:8080/signin -d 300 -t 6 -c 400 -s benchmark.lua 

echo -e "\n\n"
