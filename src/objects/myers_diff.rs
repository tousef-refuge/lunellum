// DO EDITS IN REVERSE ORDER
use crate::crypto::is_binary;
use super::file_edit::FileEdit;

// "gun to your head tell me how this works" js slide it in dawg
pub fn myers_diff(a: &Vec<u8>, b: &Vec<u8>) -> Vec<FileEdit> {
    // damn it was that simple
    if a.is_empty() {
        if b.is_empty() { return Vec::new(); }
        return vec![FileEdit::InsertData { pos: 0, data: b.clone(), }];
    }

    if b.is_empty() {
        return vec![FileEdit::DeleteData { pos: 0, data: a.clone(), }];
    }

    // as much as i dont wanna do it this makes binary files
    // not tweak out from myers_diff, maybe ill change it later
    if is_binary(a) || is_binary(b) {
        return vec![FileEdit::InsertFile { data: b.clone(), }];
    }

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
                trace.push(v.clone());
                return build_edits(&a, &b, &trace, d, offset);
            }
        }
    }

    unreachable!()
}

fn build_edits(a: &[u8], b: &[u8], trace: &[Vec<isize>], d: usize, offset: isize) -> Vec<FileEdit> {
    let mut x = a.len() as isize;
    let mut y = b.len() as isize;

    let mut raw = Vec::new();

    for depth in (1..=d).rev() {
        let v = &trace[depth];
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
            raw.push(RawEdit::Delete {
                pos: x as usize,
                byte: a[x as usize],
            });
        }

        x = prev_x;
        y = prev_y;
    }

    raw.reverse();
    merge_edits(raw)
}

fn merge_edits(raw: Vec<RawEdit>) -> Vec<FileEdit> {
    let mut edits = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        match raw[i] {
            RawEdit::Insert { pos, .. } => {
                let mut data = Vec::new();
                while i < raw.len() {
                    match raw[i] {
                        RawEdit::Insert { pos: p, byte } if p == pos => {
                            data.push(byte);
                            i += 1;
                        }
                        _ => break,
                    }
                }
                edits.push(FileEdit::InsertData { pos, data });
            }
            RawEdit::Delete { pos, .. } => {
                let mut data = Vec::new();
                while i < raw.len() {
                    match raw[i] {
                        RawEdit::Delete { pos: p, byte }
                        if p == pos + data.len() => {
                            data.push(byte);
                            i += 1;
                        }
                        _ => break,
                    }
                }
                edits.push(FileEdit::DeleteData { pos, data });
            }
        }
    }

    edits.sort();
    edits
}

#[derive(Debug)]
enum RawEdit {
    Insert { pos: usize, byte: u8 },
    Delete { pos: usize, byte: u8 },
}
