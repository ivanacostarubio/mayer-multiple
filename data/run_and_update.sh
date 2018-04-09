#!/bin/bash
script_full_path=$(dirname "$0")
cd $script_full_path

$script_full_path/mayer_multiple >> $script_full_path/result_mayer_multiple.csv

TODAY= date;
cd $script_full_path
git add ./
git commit -m "`date`"
git push github master
