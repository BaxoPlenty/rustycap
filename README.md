# rustycap

The official [CapBypass](https://capbypass.com/) wrapper for Rust.

# Examples
## Initialization of `Solver`

```rust
use rustycap::Solver;

let solver = Solver::new("CAPBYPASS_KEY");
```

## Retreiving Balance

```rust
let balance = solver.get_balance().await.expect("Unable to retreive balance");
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
| Field | Description |
|-------|-------------|
| `TaskInfo::Processing` | The task is being processed |
| `TaskInfo::DoesNotExist` | The task was unable to be found |
| `TaskInfo::Failed` | The task failed |
| `TaskInfo::Done(String)` | The task was successful. The `String` is the solution. |