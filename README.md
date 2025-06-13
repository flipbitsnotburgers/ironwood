# Ironwood

Efficient S-expression evaluation engine with string interning and memoization.

## Overview

Ironwood provides fast evaluation of large numbers of S-expressions through:
- **String interning** for efficient storage and comparison
- **Optimized data structures** for minimal memory overhead
- **Memoization** to avoid re-evaluating identical sub-expressions
- **S-expression syntax** for natural, composable expressions

## Example Usage (Planned)

```rust
use ironwood::Engine;

let mut engine = Engine::new();

// Define domains
engine.add_symbol_domain("status".to_string(), false);
engine.add_integer_domain("age".to_string(), false, 0, 150);

// Add expressions
engine.insert(1, "(and (= status \"active\") (>= age 18))");
engine.insert(2, "(or (= status \"premium\") (>= age 65))");

// Evaluate against events
let event = engine.create_event()
    .set_symbol("status", "active")
    .set_integer("age", 25)
    .build();

let matches = engine.evaluate(&event);
// Returns: [1] (first expression matches)
```

## S-Expression Syntax (Planned)

```scheme
; Comparison operations
(< x 10)
(>= age 18)
(= status "active")

; Boolean operations
(and (> x 5) (< x 10))
(or (= type "A") (= type "B"))
(not (= disabled true))

; Set operations
(in x [1 2 3 4 5])
(not-in category ["spam" "test"])

; List operations
(one-of tags ["important" "urgent"])
(all-of permissions ["read" "write"])
(none-of flags ["deprecated" "hidden"])

; Null/empty checks
(null? optional-field)
(empty? items)
```

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)