pub fn bwt<T: Clone>(data: &[T], suffix_array: &[usize]) -> Vec<T> {
    let mut bwt = vec![];

    if let Some(last) = data.last() {
        bwt.push(last.clone());
    }

    for &index in suffix_array.iter() {
        if index <= 0 {
            continue;
        }

        bwt.push(data[index - 1].clone());
    }

    bwt
}
