use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Cacher<T, P, R>
    where T: Fn(P) -> R {
    function: T,
    cache: HashMap<P, Rc<R>>
}

impl<T, P, R> Cacher<T, P, R> 
    where P: Eq + Hash + Clone + Debug, 
        R: Debug + Clone, 
        T: Fn(P) -> R {

    pub fn new(function : T) -> Cacher<T, P, R> {
        Cacher {
            function,
            cache: HashMap::new()
        }
    }

    pub fn value (&mut self, param : P) -> Rc<R> {
        match self.cache.entry(param.clone()) {
            Entry::Occupied(o) => Rc::clone(&o.into_mut()),
            Entry::Vacant(v) => Rc::clone(&v.insert(Rc::new((self.function)(param))))
        }
    }

    pub fn clear_cache(&mut self) {
        self.cache = HashMap::new();
    }

    pub fn print_cache(self) {
        for (key, val) in self.cache.iter() {
            println!("{:?} -> {:?}", key, val);
        }
    }
}

