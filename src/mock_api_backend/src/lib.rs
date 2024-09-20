use once_cell::sync::Lazy; // Import Lazy from once_cell
use std::collections::HashMap;
use std::sync::Mutex; // Import Mutex
use candid::{CandidType, Deserialize};
use serde::{Serialize}; // Ensure Serialize is included
use ic_cdk::update; // Import the update attribute
use ic_cdk::query; // Import the query attribute

#[derive(CandidType, Deserialize, Serialize, Clone)] // Add Clone here
struct Request {
    method: String,
    payload: Vec<u8>, // Store JSON as bytes
}

#[derive(CandidType, Deserialize, Serialize, Clone)] // Add Clone here
struct Response {
    status: u64,      // Use u64 for compatibility with nat
    body: Vec<u8>,    // Store JSON as bytes
}

#[derive(CandidType, Serialize, Clone)] // Ensure Clone is here
struct Route {
    route: String,          // Route name
    request: Request,       // Request details
    expected_response: Response, // Expected response details
}

struct Api {
    routes: HashMap<String, Route>, // Store routes in a HashMap
}

impl Api {
    fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
}

// Use Lazy to initialize API with a closure
static API: Lazy<Mutex<Api>> = Lazy::new(|| Mutex::new(Api::new()));

#[update]
fn add_route(route: String, request: Request, expected_response: Response) {
    let mut api = API.lock().unwrap(); // Lock the mutex
    let route_clone = route.clone(); // Clone the route for logging
    api.routes.insert(route_clone.clone(), Route { route: route_clone, request, expected_response });
    ic_cdk::println!("Route added: {}", route); // Log the added route
}

#[query]
fn get_routes() -> Vec<Route> {
    let api = API.lock().unwrap(); // Lock the mutex
    api.routes.values().cloned().collect() // Return a vector of cloned Route structs
}

#[update]
fn edit_route(route: String, new_request: Request, new_expected_response: Response) -> Result<(), String> {
    let mut api = API.lock().unwrap(); // Lock the mutex
    if api.routes.contains_key(&route) {
        let route_clone = route.clone(); // Clone the route for logging
        api.routes.insert(route_clone.clone(), Route { route: route_clone.clone(), request: new_request, expected_response: new_expected_response });
        ic_cdk::println!("Route edited: {}", route_clone); // Log the edited route
        Ok(())
    } else {
        Err("Route not found".to_string())
    }
}