#!/bin/bash

echo "Testing image upload debug..."

# First, login to get a token
echo "1. Logging in..."
LOGIN_RESPONSE=$(curl -s -X POST http://127.0.0.1:8000/api/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}')

TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.token // empty')

if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
  echo "❌ Login failed. Response: $LOGIN_RESPONSE"
  exit 1
fi

echo "✅ Login successful. Token: ${TOKEN:0:20}..."

# Test upload with a simple test file
echo "2. Testing image upload..."
echo "test file content" > /tmp/test_image.txt

UPLOAD_RESPONSE=$(curl -s -X POST http://127.0.0.1:8000/api/recipes/27/images \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/octet-stream" \
  -H "X-Filename: test_image.txt" \
  --data-binary @/tmp/test_image.txt)

echo "Upload response: $UPLOAD_RESPONSE"

# Check if files were created
echo "3. Checking uploaded files..."
find /Users/johannes/code/recipes/backend/uploads -type f -name "*" 2>/dev/null || echo "No files found"

# Test getting images
echo "4. Testing get images..."
IMAGES_RESPONSE=$(curl -s -X GET http://127.0.0.1:8000/api/recipes/27/images \
  -H "Authorization: Bearer $TOKEN")

echo "Images response: $IMAGES_RESPONSE"

# Clean up
rm -f /tmp/test_image.txt
