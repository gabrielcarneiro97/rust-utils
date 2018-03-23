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

        match self.map.entry(param.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert((self.function)(param))
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
    println!("{:?}", soma.value(&[10, 11]));
    println!("{:?}", soma.value(&[10, 11, 13, 14]));
        
}