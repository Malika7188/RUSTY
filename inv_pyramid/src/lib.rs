// Make the function public so it can be used externally
pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut p = Vec::new();

    // Top half of the pyramid
    for j in 1..=i {
        let spaces = " ".repeat((i - j) as usize);
        let line = format!("{}{}", spaces, v.repeat(j as usize));
        p.push(line);
    }

    // Bottom half of the pyramid (excluding the middle row)
    for j in (1..i).rev() {
        let spaces = " ".repeat((i - j) as usize);
        let line = format!("{}{}", spaces, v.repeat(j as usize));
        p.push(line);
    }

    p
}
