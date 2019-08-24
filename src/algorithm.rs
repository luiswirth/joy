pub mod algorithm {

    pub fn bubble_sort<I, C, T>(iter: I, cond: C)
    where
        I: Iterator<Item = T>,
        C: Fn(T, T) -> bool,
    {
        unimplemented!();
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn unit_test() {
        assert!(true);
    }
}
