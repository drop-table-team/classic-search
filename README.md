# Classic Search Output Plugin

This plugin provides a classical full text search that allows searching by text and tags.

## Deployment

Use the following command to run `classic-search`:

`ADDRESS="0.0.0.0:8080" MODULE_NAME="classic-search" BACKEND_ADDRESS="http://192.168.0.105:8080" cargo run`

### Environment Variables

The following evironment variables have to be set:

| Name | Description | Example |
| - | - | - |
| `ADDRESS` | The address that the local webserver is listening on | `0.0.0.0:8080` | 
| `MODULE_NAME` | The name of the module (in this case `classic-search`) | `classic-search` |
| `BACKEND_ADDRESS` | The address of the backend | ` 192.168.2.70:80` | 



