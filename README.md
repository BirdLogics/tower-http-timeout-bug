# Tower HTTP Timeout Bug

## Description

Demonstrates an issue with [tower-http](https://github.com/tower-rs/tower-http). The `TimeoutLayer` middleware provided by the crate returns a [408 Request Timeout](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/408), which is not properly handled by Firefox. Firefox responds to the status code by repeating the request multiple times and eventually resetting the connection.

See https://github.com/tower-rs/tower-http/issues/300

## Example output

```
Hosting on http://127.0.0.1:3000
2024-10-05T01:05:03.371466Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:06.372723Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3001 ms status=408
2024-10-05T01:05:06.375414Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:09.376837Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3001 ms status=408
2024-10-05T01:05:09.379284Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:12.380401Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3001 ms status=408
2024-10-05T01:05:12.382994Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:15.384978Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3002 ms status=408
2024-10-05T01:05:15.387308Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:18.388012Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3000 ms status=408
2024-10-05T01:05:18.390559Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:21.392279Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3001 ms status=408
2024-10-05T01:05:21.394712Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:24.396856Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3002 ms status=408
2024-10-05T01:05:24.399069Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:27.400591Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3001 ms status=408
2024-10-05T01:05:27.402798Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:30.404358Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3001 ms status=408
2024-10-05T01:05:30.406813Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-10-05T01:05:33.408362Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=3001 ms status=408
```