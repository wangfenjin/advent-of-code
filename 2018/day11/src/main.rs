fn main() {
    let mut state = [[0i32; 301]; 301];
    let input = 3628;
    for i in 1..301 {
        for j in 1..301 {
            let rackid = i as i32 + 10;
            state[i][j] = ((rackid * (j as i32) + input) * rackid / 100) % 10 - 5;
        }
    }

    let mut result = -1000;
    let mut ri = 1;
    let mut rj = 1;
    let mut rs = 1;
    let mut current = [[0i32; 301]; 301];
    for size in 1..301 {
        let mut next = [[0i32; 301]; 301];
        for i in 1..(301 - size) {
            for j in 1..(301 - size) {
                let mut h = 0;
                let mut w = 0;
                for m in j..(size+j) {
                    h += state[i+size-1][m];
                }
                for n in i..(size+i-1) {
                    w += state[n][j+size-1];
                }
                next[i][j] = current[i][j]+h+w;
                if next[i][j] > result {
                    result = next[i][j];
                    ri = i;
                    rj = j;
                    rs = size;
                }
            }
        }
        current = next;
    }
    println!("{} - {},{},{}", result, ri, rj, rs);
}
