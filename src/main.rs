pub struct Matcher<T> {
    matcher: Box<dyn Fn(&T) -> bool>, // closure defining the matching condition
    substitution: String,
}

impl<T> Matcher<T> {
    pub fn new<F>(matcher: F, substitution: String) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        Self {
            matcher: Box::new(matcher),
            substitution,
        }
    }
    pub fn apply(&self, item: &T) -> Option<String> {
        if (self.matcher)(item) {
            Some(self.substitution.clone())
        } else {
            None
        }
    }
}
// The new method essentially:

//  Encapsulates the matcher closure with a smart pointer (Box).
//  Returns a new instance of Matcher containing this smart pointer and the substitution

// 'static Ensures Lifetime Compatibility

fn main() {
    let res = Matcher::new(|s: &String| s.starts_with("Hello"), "Greeting".to_string());
    let result = res.apply(&"moo".to_string());
    println!("{:?}", result.unwrap_or("dd".to_string()));

        // Apply the matcher on a `String`
    let result = res.apply(&"Hello world".to_string());
    
    // Print the result
    println!("{:?}", result.unwrap_or("dd".to_string()));
}
