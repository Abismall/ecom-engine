




### Generate Docker Files

Run the Python script to generate the necessary Docker files.

```sh
python ./docker/generate_docker_files.py
```

### Build and Run the Docker Containers

Use Docker Compose to build and run the containers defined in the `compose.json` file.

```sh
docker-compose -f ./docker/compose.json up --build
```

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

## Additional Commands

Here are some additional commands that might be useful during development:

### Stop the Docker Containers

To stop the running Docker containers:

```sh
docker-compose -f ./docker/compose.json down
```

### View Docker Logs

To view the logs of the running containers:

```sh
docker-compose -f ./docker/compose.json logs
```

### Rebuild the Containers

If you make changes to the Docker configuration or the code, you might need to rebuild the containers:

```sh
docker-compose -f ./docker/compose.json up --build
```

### Remove Docker Images

To remove all Docker images related to the project:

```sh
docker-compose -f ./docker/compose.json down --rmi all
```

### Prune Docker System

To prune the Docker system and remove all unused containers, networks, images, and optionally, volumes:

```sh
docker system prune -a
```

