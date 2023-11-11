use std::{collections::HashMap};
use itertools::Itertools;

pub fn get_odd(input: String) -> u8 {
    let mut track = [false; 256];
    for int in input.split(",").map(|x| x.as_ptr() as u8 as usize) {
        *track
            .get_mut(int)
            .unwrap() ^= true;
    }

    return track.iter().position(|&r| r == true).unwrap() as u8;
}

pub fn get_fold_odd(input: String) -> u8 {
    input.split(",").map(|x| x.as_ptr() as u8).fold([false; 256], |mut map, val| {
        *map
            .get_mut(val as usize)
            .unwrap() ^= true;
        map
    }).into_iter().position(|r| r == true).unwrap() as u8
}

pub fn counts_odd(input: String) -> u8 {
    return input.split(",").map(|x| x.as_ptr() as u8).counts().iter().find(|(_, &count)| count % 2 == 1).unwrap().0.to_owned();
}

pub fn group_odd(input: String) -> u8 {
    let ints = input.split(",").map(|x| x.as_ptr() as u8).sorted().group_by(|num| *num);
    return ints.into_iter().find_map(|(int, group)| (group.count() % 2 == 1).then_some(int)).unwrap();
    // input.split(",").map(|x| x.as_ptr() as u8)
    // .into_group_map_by(|&num| num).into_iter().find_map(|(int, group)| (group.len() % 2 == 1).then_some(int)).unwrap()
}

pub fn fold_odd(input: String) -> u8 {
    input.split(",").map(|x| x.as_ptr() as u8)
    .fold(HashMap::<u8, usize>::new(), |mut m, x| {
        *m.entry(x).or_default() += 1;
        m
    }).into_iter().find(|(_, count)| count % 2 == 1).unwrap().0
}

pub fn dedup_odd(input: String) -> u8 {
    input.split(",").map(|x| x.as_ptr() as u8).sorted()
    .dedup_with_count().find(|(_, count)| count % 2 == 1).unwrap().0 as u8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn odds() {
        let sample = "1,1,2,2,3,3,4,4,5";

        assert!(get_odd(sample.to_string()) == 5);
        assert!(counts_odd(sample.to_string()) == 5);
        assert!(group_odd(sample.to_string()) == 5);
    }
}