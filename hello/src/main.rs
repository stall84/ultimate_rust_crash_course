// Alternatively to the fully qualified path as in (hello::greet()). You can import a function from a library into scope using 'use'
use hello::greet;

fn main() {
    println!("Hello, world!");
    // hello::greet(); // The double colon is the scope operator. You'll use this to access functions or members
    // of libraries. In this case, our current project 'hello' is the 'master' library, with all
    // functions surfaced. Accessible only if they're declared public 'pub'
    greet();
}
