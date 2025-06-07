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
- [x] `print()` (Basic support for string and integer literals, identifiers, and arithmetic expressions)
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

- [ ] `==` (Equal to)
- [ ] `!=` (Not equal to)
- [ ] `>` (Greater than)
- [ ] `<` (Less than)
- [ ] `>=` (Greater than or equal to)
- [ ] `<=` (Less than or equal to)

**3. Assignment Operators:**

- [x] `=` (Assign - for basic types and expressions)
- [ ] `+=` (Add and assign)
- [ ] `-=` (Subtract and assign)
- [ ] `*=` (Multiply and assign)
- [ ] `/=` (Divide and assign)
- [ ] `%=` (Modulus and assign)
- [ ] `**=` (Exponentiate and assign)
- [ ] `//=` (Floor divide and assign)
- [ ] `&=` (Bitwise AND and assign)
- [ ] `|=` (Bitwise OR and assign)
- [ ] `^=` (Bitwise XOR and assign)
- [ ] `>>=` (Bitwise right shift and assign)
- [ ] `<<=` (Bitwise left shift and assign)

**4. Logical Operators:**

- [ ] `and`
- [ ] `or`
- [ ] `not`

**5. Bitwise Operators:**

- [ ] `&` (Bitwise AND)
- [ ] `|` (Bitwise OR)
- [ ] `^` (Bitwise XOR)
- [ ] `~` (Bitwise NOT/Complement)
- [ ] `<<` (Left shift)
- [ ] `>>` (Right shift)

**6. Identity Operators:**

- [ ] `is`
- [ ] `is not`

**7. Membership Operators:**

- [ ] `in`
- [ ] `not in`

## Other "Etc." - Key Concepts and Features

**1. Data Structures (Built-in Types):**

- **Numbers:**
    - [x] `int` (Basic support)
    - [ ] `float`
    - [ ] `complex`
- **Strings:**
    - [x] `str` (Basic support for literals)
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
    - [ ] `bool`
- **NoneType:**
    - [ ] `None`

**2. Control Flow:**

- **Conditional Statements:**
    - [ ] `if`, `elif`, `else`
- **Loops:**
    - [ ] `for`, `while`
- **Loop Control Statements:**
    - [ ] `break`, `continue`, `pass`

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
