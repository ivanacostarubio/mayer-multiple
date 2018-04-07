#!/bin/bash


script_full_path=$(dirname "$0")

./$script_full_path/mayer_multiple

TODAY= date;

git add data/

git commit -m "`date`"

git push github data
