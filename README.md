# node_api

This crate provides bindings to the [Node-API](https://nodejs.org/docs/latest/api/n-api.html) C API, making it easy to write Node native addons in Rust.

## Example

Write your native addon in Rust:

```rust
node_api::init!(init);

fn init(env: node_api::Env, exports: node_api::Value) -> Result<node_api:Value, String> {
  let exports = export.as_object();
  let key = node_api::String::new(env, "add")?;
  let value = node_api::Function::new(env, "add", add)?;
  exports.set(key, value);
  Ok(exports.value())
}

#[node_api::function]
fn add(a: u64, b: u64) -> Result<u64, String> {
  Ok(a + b)
}
```

Then load it in Node.js.

```javascript
let native = require("./add.node");
assert(native.add(3, 4) === 7);
```

See the examples folder for a complete example.

## Serde integration

To make it easy to move data structures between Rust and Node.js, the node_api crate supports integration with [serde](https://serde.rs).

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Contact {
  name: String,
  email: String,
}

#[node_api::function]
fn contact(env: node_api::Env) -> node_api::Result<Contact> {
  Ok(Contact {
    name: "John Doe".to_owned(),
    email: "john.doe@example.com".to_owned(),
  })
}
```

```javascript
let contact = native.contact();
console.log("Contact name: " + contact.name);
console.log("Contact email: " + contact.email);
```
