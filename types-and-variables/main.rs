const ONE:u8 = 1;
static TWO:u8 = 2;   // In-memory constant

fn main()
{
    // Int
    let first:u8 = 128;
    println!("Var = {}, size = {} bytes", 
        first, 
        std::mem::size_of_val(&first)
    );

    // Decimal
    let decimal:f32 = 2.5;
    println!("Var = {}", decimal);

    // Bool
    let boolean = false;
    println!("Var = {}, size = {} bytes", 
        boolean,
        std::mem::size_of_val(&boolean)
    );

    // Char (UTF-8)
    let letter:char = 'C';
    println!("Var = {}, size = {} bytes", 
        letter,
        std::mem::size_of_val(&letter)
    );

    // Mutable
    let mut mboolean = false;
    println!("It's {}!", mboolean);
    mboolean = !mboolean;
    println!("Now It's {}!", mboolean);

    // Constants
    const PI:f32 = 3.14;
    println!("PI is a constant and its value is {}", PI);
    println!("One is a global constant and its value is {}", ONE);
    println!("Two is a global in-memory constant and its value is {}", TWO);
}

/*
    Primitive Types

neverExperimental	The ! type, also called “never”.
array	            A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
bool	            The boolean type.
char	            A character type.
f32	                A 32-bit floating point type (specifically, the “binary32” type defined in IEEE 754-2008).
f64	                A 64-bit floating point type (specifically, the “binary64” type defined in IEEE 754-2008).
fn	                Function pointers, like fn(usize) -> bool.
i8	                The 8-bit signed integer type.
i16	                The 16-bit signed integer type.
i32	                The 32-bit signed integer type.
i64	                The 64-bit signed integer type.
i128	            The 128-bit signed integer type.
isize	            The pointer-sized signed integer type.
pointer	            Raw, unsafe pointers, *const T, and *mut T.
reference	        References, &T and &mut T.
slice	            A dynamically-sized view into a contiguous sequence, [T]. Contiguous here means that elements are laid out so that every element is the same distance from its neighbors.
str	                String slices.
tuple	            A finite heterogeneous sequence, (T, U, ..).
u8	                The 8-bit unsigned integer type.
u16	                The 16-bit unsigned integer type.
u32	                The 32-bit unsigned integer type.
u64	                The 64-bit unsigned integer type.
u128	            The 128-bit unsigned integer type.
unit	            The () type, also called “unit”.
usize	            The pointer-sized unsigned integer type.

Source: https://doc.rust-lang.org/std/index.html#primitives
*/