![Rust](https://github.com/Johan3F/LogToFile/workflows/Rust/badge.svg?branch=master)

# Simple logger to file
Simple server to log everything that comes in the `/log` endpoint to a file.
No parsing no anything, just dump it into the file.
It opens the file once, and writes on that buffer. Don't expect the file to be re-created or any failure if you delete it during run time

## Run docker
For running this image via docker:
```bash
docker run -d --name logger-server -p 8899:8899 -v $(pwd):/app johanvdwm/logger-server:latest
```

### Environmental variables
| Variable 	| Comments 	|
|----------	|----------	|
|LOG_FILE | It will rename the log file from the defaulted log.log to the given name |
