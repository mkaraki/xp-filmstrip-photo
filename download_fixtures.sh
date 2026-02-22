#!/bin/bash

# Base directory
ROOT_DIR="./fixtures"

# Create directories
mkdir -p "$ROOT_DIR/Nature"
mkdir -p "$ROOT_DIR/Architecture/City"
mkdir -p "$ROOT_DIR/Travel/Asia/Tokyo"
mkdir -p "$ROOT_DIR/Food/Desserts"
mkdir -p "$ROOT_DIR/Food/Dinner"
mkdir -p "$ROOT_DIR/Food/Snacks"

# Function to download images
# Usage: download_images <directory> <count>
download_images() {
  local dir="$1"
  local count="$2"
  echo "Downloading $count images into $dir..."
  for ((i=1; i<=count; i++)); do
    # Using picsum.photos for reliable random images
    # Adding a random seed to ensure uniqueness
    curl -s -L "https://picsum.photos/1080/720?random=$RANDOM" -o "$dir/photo_$i.jpg"
    sleep 3
    echo -n "."
  done
  echo " Done."
}

# Download according to plan (Total 100)
# Note: picsum.photos doesn't support category queries directly in simple random URL, 
# but it provides high-quality random photography.
download_images "$ROOT_DIR" 5
download_images "$ROOT_DIR/Nature" 10
download_images "$ROOT_DIR/Architecture" 5
download_images "$ROOT_DIR/Architecture/City" 5
download_images "$ROOT_DIR/Travel/Asia" 5
download_images "$ROOT_DIR/Travel/Asia/Tokyo" 10
download_images "$ROOT_DIR/Food/Desserts" 50
download_images "$ROOT_DIR/Food/Dinner" 5
download_images "$ROOT_DIR/Food/Snacks" 5

echo "Finished downloading 100 photos."
