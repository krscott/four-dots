#!bash

cd _export

for d in *; do
    butler push "$d" "okayscott/four-dots:$d"
done
