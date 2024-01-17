pub fn for_loop(v: &mut Vec<i32>) {
    for x in v {
        *x += 1
    }
}

pub fn iter(v: &mut Vec<i32>) {
    let _ = v.iter_mut().map(|x| *x + 1);
}
