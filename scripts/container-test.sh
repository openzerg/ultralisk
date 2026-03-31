#!/bin/bash
set -e

echo "=== Building ultralisk container ==="
cd /home/admin/ultralisk
nix develop --command bash -c "cargo build --release"

echo "=== Building container image ==="
IMAGE=$(nix develop --command bash -c "devenv container build ultralisk 2>&1" | tail -1)
echo "Image: $IMAGE"

echo "=== Starting container ==="
podman rm -f ultralisk-test 2>/dev/null || true
podman run -d --name ultralisk-test --privileged \
  -p 15319:15319 \
  -v /home/admin/openzerg:/openzerg:ro \
  nix:$IMAGE

echo "=== Waiting for server ==="
sleep 5

echo "=== Checking health ==="
curl -s http://localhost:15319/openzerg.Agent/CheckHealth \
  -X POST -H "Content-Type: application/json" -d '{}'

echo ""
echo "=== Running E2E tests ==="
cd /home/admin/openzerg/tests
ENDPOINT=http://localhost:15319 bun test e2e.test.ts

echo "=== Cleaning up ==="
podman rm -f ultralisk-test