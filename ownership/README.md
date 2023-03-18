### Ownership in rust

Making memory management efficient so that we mostly don't have to know about stack and heap while programming.

#### Ownership Rule
1. Each variable has an owner
2. There can only be one owner at a time
3. Once variable moves out of scope, the value is dropped. 