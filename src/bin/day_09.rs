type DiskImage = Vec<Option<usize>>;

const INPUT: &str = include_str!("../../resources/day_09.txt");

fn main() {
    solve_pt1();
    solve_pt2();
}

fn solve_pt1() {
    let mut disk_image: DiskImage = create_disk_image();
    let mut empty: usize = 0;
    let mut file: usize = disk_image.len() - 1;

    loop {
        while disk_image[empty].is_some() {
            empty += 1;
        }
        while disk_image[file].is_none() {
            file -= 1;
        }
        if file <= empty {
            break;
        }
        disk_image.swap(empty, file);
    }
    let checksum: usize = calculate_checksum(&disk_image);
    println!("Checksum: {}", checksum);
}

fn solve_pt2() {
    let mut disk_image: DiskImage = create_disk_image();

    let max_id: usize = {
        let mut f_end: usize = disk_image.len() - 1;
        while disk_image[f_end].is_none() {
            f_end -= 1;
        }
        disk_image[f_end].unwrap()
    };

    let mut file_end: usize = disk_image.len() - 1;
    for i in (1..=max_id).rev() {
        while disk_image[file_end].is_none() || disk_image[file_end].unwrap() != i {
            file_end -= 1;
        }
        let mut file_size: usize = 0;
        while let Some(value) = disk_image[file_end - file_size] {
            if value != i {
                break
            }
            file_size += 1;
        }

        let mut empty_start: usize = 0;
        let mut go_next: bool = false;
        loop {
            for chunk in &disk_image[empty_start..] {
                if chunk.is_some() {
                    empty_start += 1;
                } else {
                    break;
                }
            }
            let mut empty_size: usize = 0;
            for chunk in &disk_image[empty_start + empty_size..] {
                if chunk.is_none() {
                    empty_size += 1;
                } else {
                    break;
                }
            }
            if file_end - file_size + 1 <= empty_start + empty_size - 1 {
                go_next = true;
                break
            }
            if file_size <= empty_size {
                break;
            }
            empty_start += empty_size;
        }
        if go_next {
            file_end -= file_size;
            continue;
        }

        for i in 0..file_size {
            disk_image.swap(empty_start + i, file_end - i);
        }
    }

    let checksum: usize = calculate_checksum(&disk_image);
    println!("Checksum: {}", checksum);
}

fn create_disk_image() -> DiskImage {
    let mut disk_image: DiskImage = Vec::new();
    let mut curr_id: usize = 0;
    INPUT
        .chars()
        .into_iter()
        .enumerate()
        .for_each(|(count, num)| {
            if count % 2 == 0 {
                disk_image.append(&mut vec![Some(curr_id); num.to_digit(10).unwrap() as usize]);
                curr_id += 1;
            } else {
                disk_image.append(&mut vec![None; num.to_string().parse::<usize>().unwrap()]);
            }
        });
    disk_image
}

fn calculate_checksum(disk_image: &DiskImage) -> usize {
    disk_image
        .iter()
        .enumerate()
        .fold(0, |acc: usize, (x1, x2): (usize, &Option<usize>)| {
            if let Some(val) = x2 {
                return acc + x1 * val
            }
            acc
        })
}
