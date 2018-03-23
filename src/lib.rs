mod cache_closure;


#[cfg(test)]
mod tests {
    use super::cache_closure::Cacher;
    
    #[test]
    fn cacher() {
        let mut soma = Cacher::new(|x: &[u32]| {
            let mut res = 0;

            for val in x.iter() {
                res += val;
            }
            res
        });
        assert_eq!(soma.value(&[10, 11]), 21);
        soma.print_cache();
    }
}
