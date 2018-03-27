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

        let s11_10 = soma.value(&[10, 11]);
        let s11_12 = soma.value(&[12, 11]);
        println!("{:?}", s11_10);
        println!("{:?}", s11_12);
        soma.print_cache();
    }
}
