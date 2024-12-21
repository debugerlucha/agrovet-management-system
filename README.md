# Agrovet Management System

The Agrovet Management System is a decentralized application built on the Internet Computer for managing agrovets efficiently. It provides features for managing products, orders, feedback, and more, using stable storage for secure data handling.

## Features

- **Agrovet Management**: Create, retrieve, and list agrovets with key details.
- **Product Management**: Add, retrieve, and manage products for each agrovet.
- **Order Management**: Place, track, and manage product orders.
- **Feedback Management**: Collect and view customer feedback for agrovets.
- **Data Integrity**: Persistent and reliable storage for all entities using stable structures.

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/warrenshiv/agrovet-management-system.git
   cd agrovet-management-system
   ```

2. # Starts the replica, running in the background

   ```bash
   dfx start --clean --background
   ```

3. Deploys your canisters to the replica and generates your candid interface

   ```bash
   dfx deploy
   ```bash

## API Endpoints

### Agrovet Management

- **Create Agrovet**: `create_agrovet(payload: CreateAgrovetPayload) -> Result<Agrovet, Message>`
- **Get Agrovet by ID**: `get_agrovet_by_id(id: u64) -> Result<Agrovet, Message>`
- **List All Agrovets**: `list_all_agrovets() -> Result<Vec<Agrovet>, Message>`

### Product Management

- **Create Product**: `create_product(payload: CreateProductPayload) -> Result<Product, Message>`
- **Get Products by Agrovet ID**: `get_products_by_agrovet_id(agrovet_id: u64) -> Result<Vec<Product>, Message>`

### Order Management

- **Create Order**: `create_order(payload: CreateOrderPayload) -> Result<Order, Message>`
- **Get Orders by Agrovet ID**: `get_orders_by_agrovet_id(agrovet_id: u64) -> Result<Vec<Order>, Message>`

### Feedback Management

- **Create Feedback**: `create_feedback(payload: CreateFeedbackPayload) -> Result<Feedback, Message>`
- **Get Feedback by Agrovet ID**: `get_feedback_by_agrovet_id(agrovet_id: u64) -> Result<Vec<Feedback>, Message>`

## Data Models

### Agrovet

```rust
struct Agrovet {
    id: u64,
    name: String,
    location: String,
    contact: String,
    email: String,
    products: Vec<String>,
    created_at: u64,
}
```

### Product

```rust
struct Product {
    id: u64,
    agrovet_id: u64,
    name: String,
    category: String,
    price: u64,
    stock: u64,
    is_available: bool,
}
```

### Order

```rust
struct Order {
    id: u64,
    product_id: u64,
    customer_name: String,
    quantity: u64,
    total_price: u64,
    order_date: u64,
    status: String, // "pending", "completed", "cancelled"
}
```

### Feedback

```rust
struct Feedback {
    id: u64,
    agrovet_id: u64,
    customer_name: String,
    rating: f32,
    comment: String,
    timestamp: u64,
}
```

## Contribution

Contributions are welcome! Feel free to fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.



## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown targetz
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```


## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```