# Mock API

This project provides a mock API to manage routes, allowing you to add, edit, and retrieve route information. It is built using Rust and runs on the Internet Computer.

## Features

### 1. Add Route

Add a new route to the mock API.

#### Endpoint
add_route(route: String, request: Request, expected_response: Response) -> ()
#### Request Structure

- **route**: A unique identifier for the route (e.g., `"example_route"`).
- **request**: A record containing:
- **method**: The HTTP method (e.g., `"GET"` or `"POST"`).
- **payload**: A blob containing the JSON payload.
- **expected_response**: A record containing:
- **status**: The expected HTTP status code (as `nat64`).
- **body**: A blob containing the expected JSON response.

#### Example

dfx canister call mock_api_backend add_route '( "example_route", record { method = "GET"; payload = blob "" }, record { status = 200; body = blob "{\"message\": \"Success\"}" } )'

2. Edit Route

Edit an existing route in the mock API.

Endpoint
edit_route(route: String, new_request: Request, new_expected_response: Response) -> Result<(), String>

Request Structure

• route: The unique identifier of the route to be edited.
• new_request: A record containing the new request details.
• new_expected_response: A record containing the new expected response details.

Example
dfx canister call mock_api_backend edit_route '( "example_route", record { method = "POST"; payload = blob "{\"key\":\"new_value\"}" }, record { status = 201; body = blob "{\"message\": \"Updated\"}" } )'

3. Get Routes

Retrieve all routes and their details.

Endpoint
get_routes() -> (vec Route)

Response Structure

Returns a vector of records, each containing:

• route: The route identifier.
• request: The request details (method and payload).
• expected_response: The expected response details (status and body).

Example
dfx canister call mock_api_backend get_routes

Usage

1. Build the Canisters:
  dfx build

2. Deploy the Canisters:
  dfx deploy

3. Add/Edit/Retrieve Routes: Use the commands outlined above to interact with the API.
  
Conclusion

This mock API allows for easy management of routes for testing purposes. Feel free to extend its functionality as needed!
