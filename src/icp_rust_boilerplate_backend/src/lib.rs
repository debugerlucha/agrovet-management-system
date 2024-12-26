#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Agrovet struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Agrovet {
    id: u64,
    name: String,
    location: String,
    contact: String,
    email: String,
    products: Vec<String>,
    created_at: u64,
}

// Product struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Product {
    id: u64,
    agrovet_id: u64,
    name: String,
    category: String,
    price: u64,
    stock: u64,
    is_available: bool,
}

// Order struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Order {
    id: u64,
    product_id: u64,
    customer_name: String,
    quantity: u64,
    total_price: u64,
    order_date: u64,
    status: String, // "pending", "completed", "cancelled"
}

// Feedback struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Feedback {
    id: u64,
    agrovet_id: u64,
    customer_name: String,
    rating: f32,
    comment: String,
    timestamp: u64,
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateAgrovetPayload {
    name: String,
    location: String,
    contact: String,
    email: String,
    products: Vec<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct UpdateAgrovetPayload {
    id: u64,
    name: Option<String>,
    location: Option<String>,
    contact: Option<String>,
    email: Option<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateProductPayload {
    agrovet_id: u64,
    name: String,
    category: String,
    price: u64,
    stock: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateOrderPayload {
    product_id: u64,
    customer_name: String,
    quantity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateFeedbackPayload {
    agrovet_id: u64,
    customer_name: String,
    rating: f32,
    comment: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable for Agrovet
impl Storable for Agrovet {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Agrovet {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Product
impl Storable for Product {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Product {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Order
impl Storable for Order {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Order {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Feedback
impl Storable for Feedback {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Feedback {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static AGROVETS: RefCell<StableBTreeMap<u64, Agrovet, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    static PRODUCTS: RefCell<StableBTreeMap<u64, Product, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));

    static ORDERS: RefCell<StableBTreeMap<u64, Order, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    static FEEDBACKS: RefCell<StableBTreeMap<u64, Feedback, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));
}


// Create Agrovet
#[ic_cdk::update]
fn create_agrovet(payload: CreateAgrovetPayload) -> Result<Agrovet, Message> {
    if payload.name.is_empty() || payload.contact.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let agrovet_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let agrovet = Agrovet {
        id: agrovet_id,
        name: payload.name,
        location: payload.location,
        contact: payload.contact,
        email: payload.email,
        products: payload.products,
        created_at: time(),
    };

    AGROVETS.with(|agrovets| {
        agrovets.borrow_mut().insert(agrovet_id, agrovet.clone());
    });

    Ok(agrovet)
}

// Get Agrovet by ID
#[ic_cdk::query]
fn get_agrovet_by_id(id: u64) -> Result<Agrovet, Message> {
    AGROVETS.with(|agrovets| match agrovets.borrow().get(&id) {
        Some(agrovet) => Ok(agrovet.clone()),
        None => Err(Message::NotFound("Agrovet not found".to_string())),
    })
}

// List All Agrovets
#[ic_cdk::query]
fn list_all_agrovets() -> Result<Vec<Agrovet>, Message> {
    AGROVETS.with(|agrovets| {
        let all_agrovets: Vec<Agrovet> = agrovets
            .borrow()
            .iter()
            .map(|(_, agrovet)| agrovet.clone())
            .collect();

        if all_agrovets.is_empty() {
            Err(Message::NotFound("No agrovets found".to_string()))
        } else {
            Ok(all_agrovets)
        }
    })
}

// Create Product
#[ic_cdk::update]
fn create_product(payload: CreateProductPayload) -> Result<Product, Message> {
    if payload.name.is_empty() || payload.category.is_empty() || payload.price == 0 {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let agrovet_exists = AGROVETS.with(|agrovets| agrovets.borrow().contains_key(&payload.agrovet_id));
    if !agrovet_exists {
        return Err(Message::NotFound("Agrovet not found".to_string()));
    }

    let product_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let product = Product {
        id: product_id,
        agrovet_id: payload.agrovet_id,
        name: payload.name,
        category: payload.category,
        price: payload.price,
        stock: payload.stock,
        is_available: true,
    };

    PRODUCTS.with(|products| {
        products.borrow_mut().insert(product_id, product.clone());
    });

    Ok(product)
}

// Get Products by Agrovet ID
#[ic_cdk::query]
fn get_products_by_agrovet_id(agrovet_id: u64) -> Result<Vec<Product>, Message> {
    PRODUCTS.with(|products| {
        let agrovet_products: Vec<Product> = products
            .borrow()
            .iter()
            .filter(|(_, product)| product.agrovet_id == agrovet_id)
            .map(|(_, product)| product.clone())
            .collect();

        if agrovet_products.is_empty() {
            Err(Message::NotFound("No products found for this agrovet".to_string()))
        } else {
            Ok(agrovet_products)
        }
    })
}

// Create Order
#[ic_cdk::update]
fn create_order(payload: CreateOrderPayload) -> Result<Order, Message> {
    if payload.customer_name.is_empty() || payload.quantity == 0 {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let product_exists = PRODUCTS.with(|products| products.borrow().contains_key(&payload.product_id));
    if !product_exists {
        return Err(Message::NotFound("Product not found".to_string()));
    }

    let order_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let total_price = PRODUCTS.with(|products| {
        let product = products.borrow().get(&payload.product_id).unwrap();
        product.price * payload.quantity
    });

    let order = Order {
        id: order_id,
        product_id: payload.product_id,
        customer_name: payload.customer_name,
        quantity: payload.quantity,
        total_price,
        order_date: time(),
        status: "pending".to_string(),
    };

    ORDERS.with(|orders| {
        orders.borrow_mut().insert(order_id, order.clone());
    });

    Ok(order)
}

// Get Orders by Agrovet ID
#[ic_cdk::query]
fn get_orders_by_agrovet_id(agrovet_id: u64) -> Result<Vec<Order>, Message> {
    PRODUCTS.with(|products| {
        let product_ids: Vec<u64> = products
            .borrow()
            .iter()
            .filter(|(_, product)| product.agrovet_id == agrovet_id)
            .map(|(id, _)| id)
            .collect();

        ORDERS.with(|orders| {
            let agrovet_orders: Vec<Order> = orders
                .borrow()
                .iter()
                .filter(|(_, order)| product_ids.contains(&order.product_id))
                .map(|(_, order)| order.clone())
                .collect();

            if agrovet_orders.is_empty() {
                Err(Message::NotFound("No orders found for this agrovet".to_string()))
            } else {
                Ok(agrovet_orders)
            }
        })
    })
}

// Create Feedback
#[ic_cdk::update]
fn create_feedback(payload: CreateFeedbackPayload) -> Result<Feedback, Message> {
    if payload.customer_name.is_empty() || payload.rating < 0.0 || payload.rating > 5.0 {
        return Err(Message::InvalidPayload("Invalid feedback data".to_string()));
    }

    let agrovet_exists = AGROVETS.with(|agrovets| agrovets.borrow().contains_key(&payload.agrovet_id));
    if !agrovet_exists {
        return Err(Message::NotFound("Agrovet not found".to_string()));
    }

    let feedback_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let feedback = Feedback {
        id: feedback_id,
        agrovet_id: payload.agrovet_id,
        customer_name: payload.customer_name,
        rating: payload.rating,
        comment: payload.comment,
        timestamp: time(),
    };

    FEEDBACKS.with(|feedbacks| {
        feedbacks.borrow_mut().insert(feedback_id, feedback.clone());
    });

    Ok(feedback)
}

// Get Feedback by Agrovet ID
#[ic_cdk::query]
fn get_feedback_by_agrovet_id(agrovet_id: u64) -> Result<Vec<Feedback>, Message> {
    FEEDBACKS.with(|feedbacks| {
        let agrovet_feedbacks: Vec<Feedback> = feedbacks
            .borrow()
            .iter()
            .filter(|(_, feedback)| feedback.agrovet_id == agrovet_id)
            .map(|(_, feedback)| feedback.clone())
            .collect();

        if agrovet_feedbacks.is_empty() {
            Err(Message::NotFound("No feedback found for this agrovet".to_string()))
        } else {
            Ok(agrovet_feedbacks)
        }
    })
}

// Validate and check empty fields
fn validate_required_fields(fields: Vec<&str>) -> Result<(), Message> {
    for field in fields {
        if field.is_empty() {
            return Err(Message::InvalidPayload("One or more required fields are empty".to_string()));
        }
    }
    Ok(())
}

#[ic_cdk::update]
fn update_agrovet(payload: UpdateAgrovetPayload) -> Result<Agrovet, Message> {
    AGROVETS.with(|agrovets| {
        let mut agrovets_map = agrovets.borrow_mut();
        if let Some(mut agrovet) = agrovets_map.get(&payload.id) {
            // Update fields if provided in the payload
            if let Some(name) = payload.name {
                if !name.is_empty() {
                    agrovet.name = name;
                }
            }
            if let Some(location) = payload.location {
                agrovet.location = location;
            }
            if let Some(contact) = payload.contact {
                agrovet.contact = contact;
            }
            if let Some(email) = payload.email {
                agrovet.email = email;
            }

            // Reinsert the updated agrovet back into the map
            agrovets_map.insert(payload.id, agrovet.clone());
            Ok(agrovet)
        } else {
            Err(Message::NotFound("Agrovet not found".to_string()))
        }
    })
}

#[ic_cdk::query]
fn get_stock_summary() -> Result<Vec<(String, u64, bool)>, Message> {
    PRODUCTS.with(|products| {
        let stock_summary: Vec<(String, u64, bool)> = products
            .borrow()
            .iter()
            .map(|(_, product)| (product.name.clone(), product.stock, product.is_available))
            .collect();

        if stock_summary.is_empty() {
            Err(Message::NotFound("No products found".to_string()))
        } else {
            Ok(stock_summary)
        }
    })
}

fn increment_id_counter() -> u64 {
    ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1).unwrap();
            current_value + 1
        })
}

fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

ic_cdk::export_candid!();
