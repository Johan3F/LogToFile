# Simple logger to file
Simple server to log everything that comes in the `/log` endpoint to a file.
No parsing no anything, just dump it into the file.
It opens and closes the file on each message, is not meant for performance.

## Run docker
For running this image via docker:
```bash
docker run -v ./logs/:/app/ -p 8899:8899 -e LOG_FILE=log.log 
```