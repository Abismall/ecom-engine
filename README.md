


### Send update to stream

You can add a discount using a `POST` request to the `/discount` endpoint.

```sh
curl -X POST http://localhost:3030/add_update \
     -H "Content-Type: application/json" \
     -d '{
           "NewProduct": {
             "name": "New Product",
             "in_stock": true,
             "size": "L",
             "color": "Blue",
             "weight": 700,
             "weight_unit": "g",
             "width": 150,
             "height": 250,
             "category_id": 1,
             "brand_id": 2,
             "price": 2999,
             "tax_rate": 15
           }
         }'
```

### Add a Discount

You can add a discount using a `POST` request to the `/discount` endpoint.

```sh
curl -X POST http://127.0.0.1:8000/discount \
-H "Content-Type: application/json" \
-d '{
    "name": "Customer Owner Days",
    "discount_type": "percentage",
    "value": 15,
    "start_date": "2024-06-17T04:54:00",
    "end_date": "2024-06-29T04:54:00",
    "min_quantity": 1
}'

```
