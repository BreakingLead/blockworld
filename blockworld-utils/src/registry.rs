pub trait Registry<K, V> {
    fn get_object(key: K) -> V;
    fn put_object(key: K, value: V);
}
