#! /bin/bash

cd greenrock-web-ui
bun run build
cd ..

docker buildx build --platform linux/amd64 -t pinsky/greenrock:latest .

docker push pinsky/greenrock:latest