#[cfg(not(supports_matches_macro))]
#[macro_export]
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    }
}
