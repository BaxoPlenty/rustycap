[![Crates.io](https://img.shields.io/crates/v/rustycap.svg)](https://crates.io/crates/rustycap) [![Documentation](https://docs.rs/rustycap/badge.svg)](https://docs.rs/rustycap)

# rustycap

The official [CapBypass](https://capbypass.com/) wrapper for Rust.

# Features

| Name    | Description                                                                     |
| ------- | ------------------------------------------------------------------------------- |
| `image` | Enables the use of an image to base64 encoding library for classification tasks |

# Examples

## Initialization of `Solver`

```rust
use rustycap::Solver;

let solver = Solver::new("CAPBYPASS_KEY");
```

## Retrieve Balance

```rust
let balance = solver.get_balance().await.expect("Unable to retrieve balance");
let credits = balance.credits;
```

## Creating a task and waiting for it

```rust
let data = json!({
    "blob": "test",
});
let task = FunCaptchaTask::new(
        "https://example.com/",
        "PUBLIC_KEY",
        "host:port:user:pass"
    ).data(&data).subdomain("roblox-api.arkoselabs.com");
let solution = solver.create_and_wait(task).await?;

println!("Received token: {}", solution);
```

# TaskInfo enum

| Field                      | Description                                                     |
| -------------------------- | --------------------------------------------------------------- |
| `TaskInfo::Processing`     | The task is being processed                                     |
| `TaskInfo::DoesNotExist`   | The task was unable to be found                                 |
| `TaskInfo::Failed(String)` | The task failed and the `String` contains the error description |
| `TaskInfo::Done(String)`   | The task was successful. The `String` is the solution.          |
