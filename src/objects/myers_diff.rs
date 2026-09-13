use super::file_edit::FileEdit;

// "gun to your head tell me how this works" js slide it in dawg
pub fn myers_diff(a: &Vec<u8>, b: &Vec<u8>) -> Vec<FileEdit> {
    let n = a.len();
    let m = b.len();
    let max = n + m;

    let mut trace = Vec::with_capacity(max + 1);
    let offset = max as isize;
    let mut v = vec![0isize; 2 * max + 1];

    for d in 0..=max {
        trace.push(v.clone());

        for k in (-(d as isize)..=(d as isize)).step_by(2) {
            let k_index = (k + offset) as usize;

            let mut x = if k == -(d as isize)
                || (k != d as isize
                && v[(k - 1 + offset) as usize]
                < v[(k + 1 + offset) as usize])
            { v[k_index + 1] } else { v[k_index - 1] + 1 };
            let mut y = x - k;
            
            while x < n as isize && y < m as isize && a[x as usize] == b[y as usize] { x += 1; y += 1; }

            v[k_index] = x;

            if x >= n as isize && y >= m as isize {
                return build_edits(&a, &b, &trace, d, offset);
            }
        }
    }

    unreachable!()
}

fn build_edits(a: &[u8], b: &[u8], trace: &[Vec<isize>], d: usize, offset: isize) -> Vec<FileEdit> {
    let mut edits = Vec::new();

    let mut x = a.len() as isize;
    let mut y = b.len() as isize;

    let mut raw = Vec::new();

    for depth in (1..=d).rev() {
        let v = &trace[depth - 1];

        let k = x - y;
        let prev_k = if k == -(depth as isize)
            || (k != depth as isize
            && v[(k - 1 + offset) as usize]
            < v[(k + 1 + offset) as usize])
        { k + 1 } else { k - 1 };

        let prev_x = v[(prev_k + offset) as usize];
        let prev_y = prev_x - prev_k;

        while x > prev_x && y > prev_y { x -= 1; y -= 1; }

        if x == prev_x {
            y -= 1;

            raw.push(RawEdit::Insert {
                pos: x as usize,
                byte: b[y as usize],
            });
        } else {
            x -= 1;

            raw.push(RawEdit::Remove {
                pos: x as usize,
                byte: a[x as usize],
            });
        }
    }

    raw.reverse();

    let mut i = 0;
    while i < raw.len() {
        match raw[i] {
            RawEdit::Insert { pos, .. } => {
                let mut data = Vec::new();
                let start = pos;

                while i < raw.len() {
                    match raw[i] {
                        RawEdit::Insert { pos: p, byte: byte } if p == start => {
                            data.push(byte);
                            i += 1;
                        }

                        RawEdit::Insert { pos: p, byte: byte } if p == start + data.len() => {
                            data.push(byte);
                            i += 1;
                        }

                        _ => break,
                    }
                }

                edits.push(FileEdit::Insert { pos: start, data, });
            }

            RawEdit::Remove { pos, .. } => {
                let mut data = Vec::new();
                let start = pos;

                while i < raw.len() {
                    match raw[i] {
                        RawEdit::Remove { pos: p, byte: byte } if p == start + data.len() => {
                            data.push(byte);
                            i += 1;
                        }

                        _ => break,
                    }
                }

                edits.push(FileEdit::Delete { pos: start, data, });
            }
        }
    }

    edits.sort();
    edits
}

#[derive(Debug)]
enum RawEdit {
    Insert { pos: usize, byte: u8 },
    Remove { pos: usize, byte: u8 },
}
