#[macro_export]
macro_rules! avec {
    ($($element:expr),*) => {{
        // check that count is const
        const _: usize = $crate::count![@COUNT; $($element),*];

        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::count![@COUNT; $($element),*]);
        $(vs.push($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}

fn main() {
}

fn empty_vec() {
    let vec: Vec<u32> = avec![];
}

fn single() {
    let vec: Vec<u32> = avec![32];
}

fn double() {
    let vec = avec![32, 23];
}