use std::fmt::Error;

/// Converts a value into a sequence of bytes
/// Example: `Swap { qty_1: 1, qty_2: 2 }` → `[0,0,0,1, 0,0,0,2]`
trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

/// Builds a value back from its byte representation
/// Given `[0,0,0,1, 0,0,0,2]`, returns `Swap { qty_1: 1, qty_2: 2 }`
trait Deserialize: Sized {
    fn deserialize(v: Vec<u8>) -> Result<Self, Error>;
}

struct Swap {
    qty_1: u32,
    qty_2: u32,
}

impl Serializable for Swap {
    fn serialize(&self) -> Vec<u8> {
        // Little‑endian encoding for each field
        let mut v = Vec::with_capacity(8);
        v.extend_from_slice(&self.qty_1.to_le_bytes());
        v.extend_from_slice(&self.qty_2.to_le_bytes());
        v
    }
}

impl Deserialize for Swap {
    fn deserialize(v: Vec<u8>) -> Result<Self, Error> {
        // Exactly two u32 values (8 bytes) are required
        if v.len() != 8 {
            return Err(Error);
        }
        let qty_1 = u32::from_le_bytes(v[0..4].try_into().unwrap());
        let qty_2 = u32::from_le_bytes(v[4..8].try_into().unwrap());
        Ok(Swap { qty_1, qty_2 })
    }
}

struct User {
    name: String,
    age: u32,
}

// Enable `println!("{}", user)` by implementing Display
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

// Enable `println!("{:?}", user)` by implementing Debug
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ name: {}, age: {} }}", self.name, self.age)
    }
}

/// Simple `vec!`‑style macro (for demonstration only)
macro_rules! vector {
    ($($x:expr),* $(,)?) => {{
        let mut temp_vec = Vec::new();
        $(temp_vec.push($x);)*
        temp_vec
    }};
}

pub fn macro_print() {
    // Showcase Display & Debug on `User`
    let u = User {
        name: "Prapti".into(),
        age: 22,
    };
    println!("{}", u);
    println!("{:?}", u);

    // Use our custom vector macro
    let v = vector![1, 2, 3, 4, 5];
    println!("Vector: {:?}", v);

    // Demonstrate serialization / deserialization
    let s = Swap { qty_1: 10, qty_2: 20 };
    let bytes = s.serialize();
    println!("Serialized Swap: {:?}", bytes);
    let decoded = Swap::deserialize(bytes).unwrap();
    println!(
        "Deserialized Swap: qty_1 = {}, qty_2 = {}",
        decoded.qty_1, decoded.qty_2
    );
}

// #[get("/user")]           // attribute macro (e.g. Actix or Rocket)
// fn create_user() {
//     sqlx::query!("INSERT INTO USERS VALUE ()");   // procedural macro
// }
