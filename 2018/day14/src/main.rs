fn main() {
    let input = 077201;
    let inputs = &[0,7,7,2,0,1];
    let mut state = vec![3,7];
    let mut first = 0;
    let mut second = 1;
    for i in 0.. {
        let sum = state[first] + state[second];
        if sum / 10 > 0 {
            state.push(sum / 10);
        }
        state.push(sum % 10);
        first = (1 + state[first] as usize + first) % state.len();
        second = (1 + state[second] as usize + second) % state.len();
        if state.len() > inputs.len() {
            if state.get((state.len() - inputs.len())..).unwrap() == inputs {
                println!("{}", state.len() - inputs.len());
                break;
            }
            if state.get((state.len() - inputs.len()-1)..(state.len()-1)).unwrap() == inputs {
                println!("{}", state.len() - inputs.len()-1);
                break;
            }
        }
    }
    for i in input..input + 10 {
        print!("{}", state[i]);
    }
    println!("");
}
