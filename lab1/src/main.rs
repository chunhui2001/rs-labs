fn extend_slice(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

fn main() {
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [1.2, 0.34];

    extend_slice(&mut wave, &head); // 用另一个向量扩展 wave
    extend_slice(&mut wave, &tail); // 用数组扩展 wave

    wave.push(1.0);

    // extend_slice(&mut wave, &wave)
    println!("{:?}, {}, {}", wave, wave.len(), wave.capacity());
}
