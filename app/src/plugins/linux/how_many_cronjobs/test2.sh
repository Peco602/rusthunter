#!/bin/bash

for USER in `cat /etc/passwd | awk -F ":" '{print $1}'`
do
  echo "this crontab for user : $USER"
  crontab -u $USER -l 2>&1
done >> list_all_cron