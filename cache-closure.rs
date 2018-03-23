use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Entry;
use std::fmt::Debug;

struct Cacher<T, P, R>
    where T: Fn(P) -> R {
    function: T,
    cache: HashMap<P, R>
}

impl<T, P, R> Cacher<T, P, R> 
    where P: Eq + Hash + Clone + Debug, R: Debug, T: Fn(P) -> R {

    fn new(function : T) -> Cacher<T, P, R> {
        Cacher {
            function,
            cache: HashMap::new()
        }
    }

    fn value (&mut self, param : P) -> &mut R {
        match self.cache.entry(param.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert((self.function)(param))
        }
    }

    fn clear_cache(&mut self) {
        self.cache = HashMap::new();
    }

    fn print_cache(self) {
        for (key, val) in self.cache.iter() {
            println!("{:?} -> {:?}", key, val);
        }
    }
}

fn main () {
    let mut soma = Cacher::new(|x: &[u32]| {
        println!("call");
        let mut res = 0;

        for val in x.iter() {
            res += val;
        }
        res
    });

    println!("{:?}", soma.value(&[10, 11]));
    println!("{:?}", soma.value(&[10, 12]));
    println!("{:?}", soma.value(&[10, 13]));

    soma.print_cache();
}
