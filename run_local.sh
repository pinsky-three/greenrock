#! /bin/bash

cargo run -p greenrock-engine --release &

cd greenrock-web-ui && VITE_API_BASE=http://localhost:4200 bun run dev &

wait
