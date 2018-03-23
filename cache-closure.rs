use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Entry;

struct Cacher< T, P, R>
    where T: Fn(P) -> R {
    function: T,
    map: HashMap<P, R>
}

impl<T, P, R> Cacher<T, P, R> 
    where P: Eq + Hash + Clone, T: Fn(P) -> R {

    fn new(function : T) -> Cacher<T, P, R> {
        Cacher {
            function,
            map: HashMap::new()
        }
    }
    fn value (&mut self, param : P) -> &mut R {

        let p = param.clone();

        match self.map.entry(param) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert((self.function)(p))
        }
    }
}

fn main () {
    let mut cache = Cacher::new(|x: (u32, u32)| {
        println!("call");
        x.0 + x.1
    });
    println!("{:?}", cache.value((10, 11)));
    println!("{:?}", cache.value((11, 11)));
    println!("{:?}", cache.value((10, 11)));
        
}