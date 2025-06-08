# E++ Language Feature Progress

This document tracks the implementation progress of Python syntax and features in the E++ language.

## Python Built-in Functions

- [ ] `abs()`
- [ ] `all()`
- [ ] `any()`
- [ ] `ascii()`
- [ ] `bin()`
- [ ] `bool()`
- [ ] `breakpoint()`
- [ ] `bytearray()`
- [ ] `bytes()`
- [ ] `callable()`
- [ ] `chr()`
- [ ] `classmethod()`
- [ ] `compile()`
- [ ] `complex()`
- [ ] `delattr()`
- [ ] `dict()`
- [ ] `dir()`
- [ ] `divmod()`
- [ ] `enumerate()`
- [ ] `eval()`
- [ ] `exec()`
- [ ] `filter()`
- [ ] `float()`
- [ ] `format()`
- [ ] `frozenset()`
- [ ] `getattr()`
- [ ] `globals()`
- [ ] `hasattr()`
- [ ] `hash()`
- [ ] `help()`
- [ ] `hex()`
- [ ] `id()`
- [ ] `input()`
- [ ] `int()`
- [ ] `isinstance()`
- [ ] `issubclass()`
- [ ] `iter()`
- [ ] `len()`
- [ ] `list()`
- [ ] `locals()`
- [ ] `map()`
- [ ] `max()`
- [ ] `memoryview()`
- [ ] `min()`
- [ ] `next()`
- [ ] `object()`
- [ ] `oct()`
- [ ] `open()`
- [ ] `ord()`
- [x] `pow()` (Implemented as part of `**` operator)
- [x] `print()` (Basic support for string and integer literals, identifiers, and arithmetic/logical/bitwise expressions)
- [ ] `property()`
- [ ] `range()`
- [ ] `repr()`
- [ ] `reversed()`
- [ ] `round()`
- [ ] `set()`
- [ ] `setattr()`
- [ ] `slice()`
- [ ] `sorted()`
- [ ] `staticmethod()`
- [ ] `str()`
- [ ] `sum()`
- [ ] `super()`
- [ ] `tuple()`
- [ ] `type()`
- [ ] `vars()`
- [ ] `zip()`
- [ ] `__import__()`

## Python Operators

**1. Arithmetic Operators:**

- [x] `+` (Addition)
- [x] `-` (Subtraction)
- [x] `*` (Multiplication)
- [x] `/` (Division)
- [x] `%` (Modulus)
- [x] `**` (Exponentiation)
- [x] `//` (Floor Division)

**2. Comparison (Relational) Operators:**

- [x] `==` (Equal to)
- [x] `!=` (Not equal to)
- [x] `>` (Greater than)
- [x] `<` (Less than)
- [x] `>=` (Greater than or equal to)
- [x] `<=` (Less than or equal to)

**3. Assignment Operators:**

- [x] `=` (Assign - for basic types and expressions)
- [x] `+=` (Add and assign)
- [x] `-=` (Subtract and assign)
- [x] `*=` (Multiply and assign)
- [x] `/=` (Divide and assign)
- [x] `%=` (Modulus and assign)
- [x] `**=` (Exponentiate and assign)
- [x] `//=` (Floor divide and assign)
- [x] `&=` (Bitwise AND and assign)
- [x] `|=` (Bitwise OR and assign)
- [x] `^=` (Bitwise XOR and assign)
- [x] `>>=` (Bitwise right shift and assign)
- [x] `<<=` (Bitwise left shift and assign)

**4. Logical Operators:**

- [x] `and`
- [x] `or`
- [x] `not`

**5. Bitwise Operators:**

- [x] `&` (Bitwise AND)
- [x] `|` (Bitwise OR)
- [x] `^` (Bitwise XOR)
- [x] `~` (Bitwise NOT/Complement)
- [x] `<<` (Left shift)
- [x] `>>` (Right shift)

**6. Identity Operators:**

- [x] `is` (Placeholder: C++ value equality for primitives)
- [x] `is not` (Placeholder: C++ value inequality for primitives)

**7. Membership Operators:**

- [x] `in` (Placeholder: C++ string.find for string operands)
- [x] `not in` (Placeholder: C++ string.find for string operands)

## Other "Etc." - Key Concepts and Features

**1. Data Structures (Built-in Types):**

- **Numbers:**
    - [x] `int` (Basic support, `long long` in C++)
    - [x] `float` (Supported: float literals, arithmetic, assignment, print)
    - [ ] `complex`
- **Strings:**
    - [x] `str` (Basic support for literals and `std::string` in C++)
- **Lists:**
    - [ ] `list`
- **Tuples:**
    - [ ] `tuple`
- **Dictionaries:**
    - [ ] `dict`
- **Sets:**
    - [ ] `set`
- **Frozensets:**
    - [ ] `frozenset`
- **Booleans:**
    - [x] `bool` (Handled as `int` 0 or 1 in C++, `bool` in C++ for logical ops)
- **NoneType:**
    - [ ] `None`

**2. Control Flow:**

- **Conditional Statements:**
    - [x] `if`, `elif`, `else`
- **Loops:**
    - [x] `while` (Full support: parsing, AST, codegen, variable scoping, tested June 7, 2025)
    - [ ] `for`
- **Loop Control Statements:**
    - [ ] `break`
    - [ ] `continue`
    - [ ] `pass`

**3. Functions (Defining Your Own):**

- [ ] `def` keyword
- [ ] `return` statement
- [ ] Arguments, parameters
- [ ] `lambda`
- [ ] Decorators

**4. Classes and Objects (Object-Oriented Programming):**

- [ ] `class` keyword
- [ ] `__init__`
- [ ] `self`
- [ ] Attributes and Methods
- [ ] Inheritance
- [ ] Polymorphism
- [ ] Encapsulation

**5. Modules and Packages:**

- [ ] `import`
- [ ] `from ... import ...`
- [ ] `as`
- [ ] Standard Library access
- [ ] Third-party packages

**6. Exception Handling:**

- [ ] `try`, `except`, `else`, `finally`
- [ ] `raise`

**7. File I/O:**

- [ ] `open()`
- [ ] `read()`, `write()`, `close()`
- [ ] `with open(...) as f:`

**8. Comprehensions:**

- [ ] List comprehensions
- [ ] Dictionary comprehensions
- [ ] Set comprehensions
- [ ] Generator expressions

**9. Generators and Iterators:**

- [ ] `yield`
- [ ] Iterables, iterators

**10. Context Managers:**

- [ ] `with` statement
- [ ] `__enter__`, `__exit__`
