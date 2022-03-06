#! /bin/sh
# for now this will only call pandoc on the front.md file
pandoc front.md -f markdown -t html -o front.html
