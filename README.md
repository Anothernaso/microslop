### **Putting the "Slop" in "MicroSlop" since 2026.**

Turn your text into beautifully chaotic, glitchy, Wandoze-level slop.

Because why type neatly when you can hallucinate?

#### **Examples:**
```rust
use microslop::prelude::*;

fn main() {
    println!("{}", "MicroSlop Wandoze".to_slop().hallucinate());
}
```
or

```c
#include "microslop.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    char *slop = MICROSLOP_to_slop("Hello, World!");

    char *hall = MICROSLOP_to_hallucination(slop);
    free(slop);

    printf("%s\n", hall);
    free(hall);

    return 0;
}
```
