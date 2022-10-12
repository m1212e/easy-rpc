// https://stackoverflow.com/a/69324393/11988368
/**
 * Simple type conversion which asserts that the caller ensures that this type
 * conversion is safe to make.
 */
#[macro_export]
macro_rules! cast {
    ($target: expr, $pat: path) => {
        {
            if let $pat(a) = $target { // #1
                a
            } else {
                panic!(
                    "mismatch variant when cast to {}", 
                    stringify!($pat)); // #2
            }
        }
    };
}