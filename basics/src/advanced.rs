// generic/monomorphic is like a C++ template 
// can be optimized and inlined
fn to_string1<T: ToString> (item: &T) -> String {
    item.to_string()
}

// dynamic/polymorphic is like a C++ base class with virutal methods
// trait object using dynamic dispatch
fn to_string2(item: &dyn ToString) -> String {
    item.to_string()
}


// `From` should be preferred over `Into` probably because of auto-implementation
// `AsRef` is for cheap reference-to-reference conversions like `From<&T> for &U`

// passing a sequence of values to a function should be done using `impl IntoIterator<Item=T>`


// ExtensionTraits can be used to add methods to already existing types
pub trait OptionMutExt<T> {
    fn replace(&mut self, val: T) -> Option<T>;

    fn replace_with<F: FnOnce() -> T>(&mut self, f: F) -> Option<T>;
}

impl<T> OptionMutExt<T> for Option<T> {
    fn replace(&mut self, val: T) -> Option<T> {
        self.replace_with(move || val)
    }

    fn replace_with<F: FnOnce() -> T>(&mut self, f: F) -> Option<T> {
        if self.is_some() {
            let result = self.take();
            *self = Some(f());
            result
        } else {
            None
        }
    }
}