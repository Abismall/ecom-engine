#!/bin/bash

# Function to generate a random product
generate_random_product() {
  local index=$1
  local sizes=("S" "M" "L" "XL")
  local colors=("Red" "Blue" "Green" "Yellow" "Black" "White")
  local size=${sizes[$RANDOM % ${#sizes[@]}]}
  local color=${colors[$RANDOM % ${#colors[@]}]}
  local in_stock=(1==1)
  local weight=$((RANDOM % 1000 + 500)) # Random weight between 500 and 1500 grams
  local width=$((RANDOM % 200 + 100)) # Random width between 100 and 300 mm
  local height=$((RANDOM % 300 + 200)) # Random height between 200 and 500 mm
  local category_id=$((RANDOM % 3 + 1)) # Random category_id between 1 and 10
  local brand_id=$((RANDOM % 3 + 1)) # Random brand_id between 1 and 5
  local price=$((RANDOM % 5000 + 1000)) # Random price between 1000 and 6000
  local tax_rate=$((RANDOM % 10 + 5)) # Random tax_rate between 5% and 15%

  # Generate the JSON product
  echo '{
    "NewProduct": {
      "name": "Product '"$index"'",
      "in_stock": '"true"',
      "size": "'"$size"'",
      "color": "'"$color"'",
      "weight": '"$weight"',
      "weight_unit": "g",
      "width": '"$width"',
      "height": '"$height"',
      "category_id": '"$category_id"',
      "brand_id": '"$brand_id"',
      "price": '"$price"',
      "tax_rate": '"$tax_rate"'
    }
  }'
}

# Function to send product updates
send_product_updates() {
  local numberOfProducts=$1
  local url="http://localhost:3030/add_update"

  for ((i=1; i<=numberOfProducts; i++)); do
    local product_json=$(generate_random_product $i)
    echo "Sending update for Product $i..."
    curl -X POST "$url" -H "Content-Type: application/json" -d "$product_json"
    echo -e "\n"
  done
}

# Get the number of products to generate from command line arguments
if [[ -n $1 && $1 =~ ^[0-9]+$ && $1 -gt 0 ]]; then
  send_product_updates $1
else
  echo "Please provide a valid number of products to generate."
  exit 1
fi
