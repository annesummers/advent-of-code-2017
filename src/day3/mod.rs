use std::num::*;

pub fn run() {

    println!("Steps {}", part1(string_to_int("1")));
    println!("Steps {}", part1(string_to_int("12")));
    println!("Steps {}", part1(string_to_int("23")));
    println!("Steps {}", part1(string_to_int("1024")));
    println!("Steps {}", part1(string_to_int("368078")));
    println!("First largest {}", part2_again_again(string_to_int("368078")));
}

fn string_to_int(string: &str) -> u32 {
    return string.parse::<u32>().unwrap();
}

pub fn part1(data_pos: u32) -> u32 {
    let first_count = 1;
    let mut ring_count = 8;
    let mut total = 8;

    if data_pos == first_count {
        return first_count;
    }

    loop {
        if data_pos <= total + first_count {
            break;
        }

        ring_count = ring_count + 8;
        total += ring_count;
    }

    let side_length = ring_count /4;

    let mut ring_start = total - ring_count + first_count;
    let mut count = 0;
    let mut side_pos = 0;
    while count  < side_length {
        if data_pos == ring_start ||
            data_pos == ring_start + side_length ||
            data_pos == ring_start + side_length * 2 ||
            data_pos == ring_start + side_length * 3 {
            let answer = side_length - count;
            return answer;
        }

        ring_start += 1;
        if side_pos < side_length/2 {
            count += 1;
        } else{
            count -= 1;
        }

        side_pos +=1;
    }

    return 0;
}

fn part2_again_again(input: u32) -> u32 {
    let m: usize= 6;
    let h: usize = 2*m;

    let mut a: Vec<Vec<u32>> = Vec::with_capacity(h);
    for i in 0..h {
        a.insert(i, Vec::with_capacity(h));

        for j in 0..h {
            a[i].insert(j, 0);
        }
    }

    a[m][m] = 1;

    let t = vec![(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)];

    let mut n: i32 = 1;
    let mut g: i32;
    let mut r: i32;
    let mut q: i32;
    let mut d: i32;

    let mut j: i32;
    let mut k: i32;

    loop {
        g = isqrt(n as f64) as i32;
        r = (g + g%2)/2;
        q = 4 * (r as f32).powi(2) as i32;
        d = n as i32 - q;

        if n <= q - 2 * r {
            j = d + 3 * r;
            k = r;
        } else if n <= q {
            j = r;
            k = -d - r;
        } else if n <= q + 2 * r {
            j = r - d;
            k = -r;
        } else {
            j = -r;
            k = d - 3 * r;
        }

        j = j + m as i32;
        k = k + m as i32;

        let mut s= 0;
        let mut c = 0;

        while c < 8 {
            s = s + a[(j + t[c].0) as usize][(k + t[c].1) as usize];
            c += 1;
        }

        let x: usize = j as usize;
        let y: usize = k as usize;

        a[x][y]=s;

        println!("{}, {}, val {}\n", x, y, s);

        if s > input || x == h || y == h {
            return s;
        }

        n +=1;
    }

    return 0;
}

fn isqrt(num: f64) -> i32 {
    return num.sqrt().floor() as i32;
}