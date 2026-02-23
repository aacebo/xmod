pub trait Context<K, T: 'static> {


    fn get(&self) -> &impl Accessor<T> {
        self.key(std::any::type_name::<T>())
    }

    fn get_mut(&mut self) -> &mut impl Accessor<T> {
        self.key_mut(std::any::type_name::<T>())
    }
}

pub trait Fork<T: 'static>: Context<T> {
    fn fork(&self) -> impl Context<T>;
}

pub trait Cancel<T: 'static>: Context<T> {
    fn cancel();
}

///
/// ## Accessor<T>
/// represents a Context field/property
/// 
pub trait Accessor<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
    fn set(&mut self, value: T);
}

// pub trait Provider {
//     type Target: 

//     fn create(&self) -> impl Context<T>;
// }