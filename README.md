# Example using `tokio` with `egui`

This example uses `reqwest` to send an HTTP request to [httpbin](https://httpbin.org/). The parsed response contains an increment value (as provided in the request) that is finally sent to the main GUI thread.

## Disclaimer

This is the worst possible way to increment a counter, but it demonstrates awaiting async tasks without blocking the main GUI thread. It is also a very poor use of `tokio`. Awaiting the future in a thread with `pollster` is both easier and just as efficient. But if you _actually_ need `tokio`, this is how to use it with `egui`.
