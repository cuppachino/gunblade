pub fn partition_results<T, E>(results: Vec<Result<T, E>>) -> (Vec<T>, Vec<E>) {
    let mut ok = Vec::new();
    let mut err = Vec::new();
    for result in results {
        match result {
            Ok(t) => ok.push(t),
            Err(e) => err.push(e),
        }
    }
    (ok, err)
}
