#[macro_use]
extern crate serde;
use candid::{CandidType, Deserialize, Encode, Decode};
use ic_cdk::api::println;
use ic_cdk::update; // Import the update attribute from ic_cdk
use ic_cdk::query; // Import the query attribute from ic_cdk
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Type alias for memory
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Define an error type for handling various errors
#[derive(Debug)]
pub enum Error {
    InvalidOperation { msg: String },
    NotFound { msg: String },
}

// Struct representing a Request
#[derive(CandidType, Deserialize, Serialize, Clone)]
struct Request {
    method: String,
    payload: Vec<u8>, // Store JSON as bytes
}

// Struct representing a Response
#[derive(CandidType, Deserialize, Serialize, Clone)]
struct Response {
    status: u64,      // Use u64 for compatibility with nat
    body: Vec<u8>,    // Store JSON as bytes
}

// Struct representing a Route
#[derive(CandidType, Deserialize, Serialize, Clone)]
struct Route {
    route: String,          // Route name
    request: Request,       // Request details
    expected_response: Response, // Expected response details
}

// Implementing `Storable` for Request
impl Storable for Request {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing `Storable` for Response
impl Storable for Response {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing `Storable` for Route
impl Storable for Route {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing `BoundedStorable` to define storage constraints for Route
impl BoundedStorable for Route {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Memory manager for stable memory and storage map for routes
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ROUTE_STORAGE: RefCell<StableBTreeMap<String, Route, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );
}

// Function to validate Route inputs
fn validate_route(route: &Route) -> Result<(), Error> {
    if route.route.trim().is_empty() {
        return Err(Error::InvalidOperation {
            msg: "Route name cannot be empty.".to_string(),
        });
    }
    Ok(())
}

// Update function to add a new route
#[ic_cdk::update] // Use the correct update attribute
fn add_route(route: Route) -> Result<(), Error> {
    // Validate the route before processing
    validate_route(&route)?;

    ROUTE_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.insert(route.route.clone(), route.clone());
        println!("Route added: {}", route.route); // Log the added route
        Ok(())
    })
}

// Query function to get all routes
#[ic_cdk::query] // Use the correct query attribute
fn get_routes() -> Vec<Route> {
    ROUTE_STORAGE.with(|storage| {
        let storage = storage.borrow();
        storage.values().cloned().collect() // Return a vector of cloned Route structs
    })
}

// Update function to edit an existing route
#[ic_cdk::update] // Use the correct update attribute
fn edit_route(route_name: String, new_request: Request, new_expected_response: Response) -> Result<(), Error> {
    ROUTE_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(route) = storage.get_mut(&route_name) {
            route.request = new_request;
            route.expected_response = new_expected_response;
            println!("Route edited: {}", route_name); // Log the edited route
            Ok(())
        } else {
            Err(Error::NotFound {
                msg: format!("Route '{}' not found.", route_name),
            })
        }
    })
}

// Update function to delete an existing route
#[ic_cdk::update] // Use the correct update attribute
fn delete_route(route_name: String) -> Result<(), Error> {
    ROUTE_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&route_name).is_some() {
            println!("Route deleted: {}", route_name); // Log the deleted route
            Ok(())
        } else {
            Err(Error::NotFound {
                msg: format!("Route '{}' not found.", route_name),
            })
        }
    })
}

// Init function for initializing memory (currently no initialization required)
#[ic_cdk::init] // Use the correct init attribute
fn init() {
    // Memory initialization is handled by thread_local! memory management
}
