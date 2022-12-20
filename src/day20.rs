use std::collections::LinkedList;
use std::rc::Rc;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;

const INPUT : &str = include_str!("../input/2022/day20.txt");

fn main(){
    let start = Instant::now();
    let encrypted_file = INPUT.parse::<EncryptedFile>().unwrap();
    let part_1 = {
        let mut encrypted_file = encrypted_file.clone();
        encrypted_file.mix(1);
        encrypted_file.get(1000) + encrypted_file.get(2000) + encrypted_file.get(3000)
    };
    println!("Part 1: {}", part_1);
    let part_2 = {
        let mut encrypted_file = encrypted_file.clone();
        encrypted_file.key = 811589153;
        encrypted_file.mix(10);
        encrypted_file.get(1000) + encrypted_file.get(2000) + encrypted_file.get(3000)
    };
    println!("Part 2: {}", part_2);
    println!("Time: {}ms", start.elapsed().as_millis());
}

#[derive(Clone)]
pub struct EncryptedFile{
    data: LinkedList<Rc<i64>>,
    key: i64,
}

impl FromStr for EncryptedFile{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split('\n').filter_map(|line| line.parse::<i64>().ok()).map(Rc::new).collect();
        Ok(EncryptedFile{data, ..EncryptedFile::default()})
    }
}

impl EncryptedFile{
    fn default() -> Self{
        EncryptedFile{data: LinkedList::new(), key: 1}
    }

    fn mix(&mut self, times: i32) {
        let original_order = self.data.iter().cloned().collect_vec();

        for _ in 0..times {
            for value in original_order.iter() {
                let position = self.data.iter().position(|x| Rc::ptr_eq(x, value)).unwrap();
                let new_position = (position as i64 + (value.as_ref() * self.key)).rem_euclid(self.data.len() as i64 - 1) as usize;
                let data = { //remove the value from old position
                    let mut split_list = self.data.split_off(position);
                    let data = split_list.pop_front().unwrap();
                    self.data.append(&mut split_list);
                    data
                };
                { //add it back at the new position
                    let mut split_list = self.data.split_off(new_position);
                    self.data.push_back(data);
                    self.data.append(&mut split_list);
                }
            }
        }
    }

    fn get(&self, index: usize) -> i64 {
        let start = self.data.iter().position(|x| *x.as_ref() == 0).unwrap();
        let index = (start + index).rem_euclid(self.data.len());
        **self.data.iter().nth(index).unwrap() * self.key
    }
}
