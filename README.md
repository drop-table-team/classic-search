# Classic Search Output Plugin

This plugin provides a classical full text search that allows searching by text and tags.

## Deployment

Use the following command to run `classic-search`:

`ADDRESS="0.0.0.0:8080" MONGO_ADDRESS="mongodb://192.168.0.111:27017" MONGO_DATABASE="data" MONGO_COLLECTION="entries" cargo run`

### Environment Variables

The following evironment variables have to be set:

| Name | Description | Example |
| - | - | - |
| `ADDRESS` | The address that the local webserver is listening on | `0.0.0.0:8080` | 
| `MONGO_ADDRESS` | The address of the MongoDB | `mongodb://192.168.0.111:27017` | 
| `MONGO_DATABASE` | The name of the MongoDB database | `data` | 
| `MONGO_COLLECTION` | The name of the MongoDB collection | `entries` | 



