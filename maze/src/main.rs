fn solve(maze: &[[u8; 5]; 5]) -> bool { // 5x5 をいれてboolがでてくる
    let mut tried = [[false; 5]; 5];
    if maze[0][0] == 1 || maze[4][4] == 1 { //  if start or end is corridor
        return false; // とけへん
    }
    let mut to_try = vec![(0, 0)]; // 0,0 で始まるベクトル 
    while !to_try.is_empty() {
        let (i, j) = to_try.pop().unwrap(); // take last element, unwrap はスペースをとるだけ 
        if (i, j) ==  (4, 4) { // if at end
            return true; // とける
        }
        let next = [ (i as i32 - 1, j as i32), (i as i32 + 1, j as i32), (i as i32, j as i32 - 1), (i as i32, j as i32 + 1), ]; // 上、下、右、左, TODO: 一番最初とか端やったら調べんくていい場合も
        for (ni, nj) in next {
            let ni = ni as usize; // usizeに変える
            let nj = nj as usize;
            if ni < 5 && nj < 5 && maze[ni][nj] == 0 && !tried[ni][nj] { 
                tried[ni][nj] = true; // tried
                to_try.push((ni, nj)); // to_tryにたす
            }
        }
    } false
}

fn main() {
    let maze = [
        [0, 0, 1, 1, 1],
        [1, 0, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 0, 1],
        [1, 1, 1, 0, 0],
    ];

    println!("{}", solve(&maze));
}
