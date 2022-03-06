#! /bin/sh
# builds the website and moves the result to ./pages

trunk clean
trunk build

mv dist/* ../pages/
