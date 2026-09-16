pub fn entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    // Count how many times each of the 256 possible byte values appear
    let mut counts = [0usize; 256];
    for &byte in data {
        counts[byte as usize] += 1;
    }

    // Sum -p*log2(p) over every value
    let len = data.len() as f64;
    let mut entropy = 0.0;
    for &count in counts.iter() {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    entropy
}

#[cfg(test)]
mod tests {}
