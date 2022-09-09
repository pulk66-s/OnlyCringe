#!/bin/bash
sudo systemctl start mysql

cd server/app;
cargo run;