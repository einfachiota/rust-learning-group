# 📙 Guide 

## Types & Variables

### Core Data Types

#### Integer
```rust
fn main() {

    let integer = 42;
    println!("{}", integer);

    let integer2: u32 = "42".parse().expect("Not a number!");
    println!("{}", integer2);

}
```

#### Floating-Point
```rust
fn main() {

    let float = 42.0; // f64
    println!("{}", float);

    let float2: f32 = 4.2; // f32
    println!("{}", float2);
}
```
- Operators
- Scope & Shadowing
- Declaring & Using Constants
- Stacks & Heaps
