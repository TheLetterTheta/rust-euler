// Nicholas Dolan
// 2019-01-17
// Euler project #9
pub fn go(target: u32) -> u32{
    pythagorean_tripple(target)
}

fn pythagorean_tripple(target: u32) -> u32 {
    let r = (1..( target / 2))
        .flat_map(|i|
                  (i..(target / 2))
                  .map(move |j| (i, j, ((i*i + j*j) as f64).sqrt()))
                  .filter(|j| j.2.fract() == 0.0)
                  .filter(|i| i.0 + i.1 + (i.2 as u32) == target)
        )
        .nth(0)
        .unwrap();
    r.0 * r.1 * (r.2 as u32)
}
