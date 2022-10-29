pub trait Crud<T: Clone> {
    fn create(v: &mut Vec<T>, e: T, should_read: bool) -> Option<T> {
        if should_read {
            let e_clone = e.clone();
            v.push(e);
            Some(e_clone)
        }
        else {
            v.push(e);
            None
        }
    }
    fn read(v: &Vec<T>, mut predicate: impl FnMut(&T) -> bool) -> Option<T> {
        for e in v.iter() {
            if predicate(e) {
                return Some(e.clone())
            }
        }
        None
    }
    fn update(v: &mut Vec<T>, mut predicate: impl FnMut(&T) -> bool, mut update: impl FnMut(&mut T), should_read: bool) -> Option<T> {
        for e in v.iter_mut() {
            if predicate(e) {
                update(e);
                if should_read {
                    return Some(e.clone())
                }
                else {
                    return None
                }
            }
        }
        None
    }
    // May want to return Option<T>, may want delete_all and this with Option<T>?
    fn delete(v: &mut Vec<T>, predicate: impl FnMut(&T) -> bool) -> usize {
        let initial_size = v.len();
        v.retain(predicate);
        initial_size - v.len()
    }
}
