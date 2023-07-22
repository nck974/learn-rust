#[macro_export] // Allows to be brought into scope
macro_rules! vec { // Declare name without "!"
    ( $( $x:expr ),* ) => { // Arm that matches an expression followed by the return value with =>
        { // $x has the value of the matched expression
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )* // match n number of times containing the variable $x
            temp_vec
        }
    };
}