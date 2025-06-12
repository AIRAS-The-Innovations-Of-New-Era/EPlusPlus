# E++ Language Feature Progress

This document tracks the implementation progress of Python syntax and features in the E++ language.

## Recent Major Updates

**Built-in Functions Implementation (Latest):**
- ✅ Implemented 20+ Python built-in functions including `abs()`, `max()`, `min()`, `len()`, `chr()`, `ord()`, `hex()`, `bin()`, `oct()`, `input()`, `sum()`, `all()`, `any()`, `reversed()`, `round()`, and type conversion functions (`int()`, `float()`, `bool()`, `str()`)
- ✅ Implemented `ascii()`, `bytearray()`, `bytes()`, `callable()`.
- ✅ Enhanced `print()` with multi-argument support and comprehensive type overloads
- ✅ Added `pow()` function with optional modulus parameter
- ✅ Fixed template ambiguity issues in C++ code generation

**Grammar and Parser Enhancements:**
- ✅ Added support for single-quoted string literals (`'char'`) alongside double-quoted strings
- ✅ Implemented unary minus (`-`) and unary plus (`+`) operators
- ✅ Fixed list literal parsing to properly handle expressions like `[1, 2, 3]`
- ✅ Added comprehensive function call argument parsing with keyword argument support
- ✅ Added parser and AST support for basic class definitions, including single inheritance.

**Code Generation Improvements:**
- ✅ Implemented variadic template system for multi-argument functions
- ✅ Added 50+ lines of C++ helper functions for built-in operations
- ✅ Resolved C++ compilation issues with container operations and type conversions
- ✅ Implemented C++ code generation for basic class structures (structs with static members/methods) and single inheritance.

## Python Built-in Functions

- [x] `abs()` (Implemented: absolute value for int, float, complex)
- [x] `all()` (Implemented: returns True if all elements are truthy)
- [x] `any()` (Implemented: returns True if any element is truthy)
- [x] `ascii()` (Implemented: ASCII representation of objects)
- [x] `bin()` (Implemented: binary representation of integers)
- [x] `bool()` (Implemented: boolean conversion for all types)
- [ ] `breakpoint()`
- [x] `bytearray()` (Implemented: mutable sequence of bytes)
- [x] `bytes()` (Implemented: immutable sequence of bytes)
- [x] `callable()` (Implemented: checks if an object is callable)
- [x] `chr()` (Implemented: character from ASCII code)
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
- [x] `float()` (Implemented: float conversion from int, float, string)
- [ ] `format()`
- [ ] `frozenset()`
- [ ] `getattr()`
- [ ] `globals()`
- [ ] `hasattr()`
- [ ] `hash()`
- [ ] `help()`
- [x] `hex()` (Implemented: hexadecimal representation of integers)
- [ ] `id()`
- [x] `input()` (Implemented: string input from user with optional prompt)
- [x] `int()` (Implemented: integer conversion from int, float, string)
- [ ] `isinstance()`
- [ ] `issubclass()`
- [ ] `iter()`
- [x] `len()` (Implemented: length of strings, lists, dicts, sets, tuples)
- [ ] `list()`
- [ ] `locals()`
- [ ] `map()`
- [x] `max()` (Implemented: maximum of multiple arguments or iterable)
- [ ] `memoryview()`
- [x] `min()` (Implemented: minimum of multiple arguments or iterable)
- [ ] `next()`
- [ ] `object()`
- [x] `oct()` (Implemented: octal representation of integers)
- [ ] `open()`
- [x] `ord()` (Implemented: ASCII code from character)
- [x] `pow()` (Implemented: power function with optional modulus)
- [x] `print()` (Implemented: multi-argument printing with type overloads)
- [ ] `property()`
- [ ] `range()`
- [ ] `repr()`
- [x] `reversed()` (Implemented: reverse iteration for lists and strings)
- [x] `round()` (Implemented: rounding with optional precision)
- [ ] `set()`
- [ ] `setattr()`
- [ ] `slice()`
- [ ] `sorted()`
- [ ] `staticmethod()`
- [x] `str()` (Implemented: string conversion for all types)
- [x] `sum()` (Implemented: sum of iterable with optional start value)
- [ ] `super()`
- [ ] `tuple()`
- [x] `type()` (Implemented: basic type information, placeholder)
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

**6. Unary Operators:**

- [x] `+` (Unary plus)
- [x] `-` (Unary minus/negation)

**7. Identity Operators:**

- [x] `is` (Placeholder: C++ value equality for primitives)
- [x] `is not` (Placeholder: C++ value inequality for primitives)

**8. Membership Operators:**

- [x] `in` (Placeholder: C++ string.find for string operands)
- [x] `not in` (Placeholder: C++ string.find for string operands)

## Other "Etc." - Key Concepts and Features

**1. Data Structures (Built-in Types):**

- **Numbers:**
    - [x] `int` (Basic support, `long long` in C++)
    - [x] `float` (Supported: float literals, arithmetic, assignment, print)
    - [x] `complex` (Implemented: basic support for complex numbers, arithmetic, assignment, print)
- **Strings:**
    - [x] `str` (Enhanced support: double-quoted and single-quoted literals, comprehensive built-in functions)
- **Lists:**
    - [x] `list` (Implemented: basic support for list literals, indexing, assignment, and iteration)
- **Tuples:**
    - [x] `tuple` (Implemented: basic support for tuple literals, unpacking, and assignment)
- **Dictionaries:**
    - [x] `dict` (Implemented: basic support for dict literals, key access, assignment, and iteration)
- **Sets:**
    - [x] `set` (Implemented: basic support for set literals, membership, and iteration)
- **Frozensets:**
    - [x] `frozenset` (Implemented: basic support for frozenset literals and membership)
- **Booleans:**
    - [x] `bool` (Handled as `int` 0 or 1 in C++, `bool` in C++ for logical ops)
- **NoneType:**
    - [x] `None` (Implemented: basic support for None/null value, assignment, and comparison)

**2. Control Flow:**

- **Conditional Statements:**
    - [x] `if`, `elif`, `else`
- **Loops:**
    - [x] `while` (Full support: parsing, AST, codegen, variable scoping, tested June 7, 2025)
    - [x] `for`
- **Loop Control Statements:**
    - [x] `break`
    - [x] `continue`
    - [x] `pass`

**3. Functions (Defining Your Own):**

- [x] `def` keyword
- [x] `return` statement
- [x] Arguments, parameters
- [x] `lambda`
- [x] Decorators

**4. Classes and Objects (Object-Oriented Programming):**

- [x] `class` keyword (basic structure, definition, and C++ codegen for static members/methods)
- [~] Attributes and Methods (static-like attributes and methods within class definitions only; no instance members or `self` yet)
- [~] Inheritance (basic single inheritance syntax and C++ generation for static context; no polymorphism or complex MRO)
- [ ] `__init__` (constructor)
- [ ] `self` (instance context)
- [ ] Instantiation (creating objects from classes)
- [ ] Instance member access (e.g., `obj.attr`, `obj.method()`)
- [ ] Polymorphism (dynamic dispatch based on object type)
- [ ] Encapsulation (public/private concepts are not yet defined)

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
