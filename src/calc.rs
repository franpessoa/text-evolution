use std::collections::HashMap;
use serde_derive::Serialize;
use rand::{thread_rng, Rng};
use nanoid::nanoid;

pub fn distance(a: &String, b: &String) -> Option<u16> {
    let (mut it_target, mut it_content) = (a.chars().into_iter(), b.chars().into_iter());
    
    if it_target.clone().count() != it_content.clone().count() {
        return None;
    } else {
        let mut sim_count: u16 = 0;
        loop {
            match (it_target.next(), it_content.next()) {
                (Some(x), Some(y)) => if x != y { sim_count += 1 },
                (None, None) => return Some(sim_count),
                _ => return None
            }
        }   
    }
}   

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Individual {
    pub id: String,
    pub content: String,
    pub parent: Option<String>
}

#[derive(Serialize, Clone, Debug)]
pub struct Simulation {
    pub target: String,
    pub individuals: Vec<Individual>,
    pub num_clones: u32,
    pub survivors: u32,
    pub mut_count: u16,
    pub gen_count: u32
}

impl Individual {
    pub fn new(c: String, parent: Option<String>) -> Self { 
        return Self {
            id: nanoid!(5),
            content: c,
            parent
        } 
    }
    
    pub fn mutate(&self, count: u8) -> Self {
        let mut content = self.content.clone();
        
        let all_chars: String = format!(
            "{}{}{}{}",
            "abcdefghijklmnopqrstuvwxyz",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "1234567890",
            "\'\"!@#$%&*()_+-=¹²³£¢¬{[]}§ªº |<>:;,."
        );

        let mut rng = thread_rng();
        let mut mutations = Vec::new();
        
        // Generating and applying random mutations
        for _ in 0..count {
            mutations.push((
                rng.gen_range(0..content.chars().count()), 
                all_chars.chars().nth(
                    rng.gen_range(0..all_chars.chars().count()).try_into().unwrap()
                ).unwrap()
            ));
        };
        
        content = content.chars().enumerate().map(|(idx, ch)| {
            let mut return_char = ch;
            for mutation in &mutations {
                if mutation.0 == idx { return_char =  mutation.1 }
            }
            return return_char
        }).collect::<String>();
        
        return Self::new(content, Some(self.id.clone()))
    }
    
    pub fn is_target(&self, target: &String) -> bool {
        return &self.content == target;
    }
    
    pub fn compare(&self, target: &String) -> Option<u16> {
        return distance(target, &self.content)
    }
}


impl Simulation {
    pub fn advance_gen(&self) -> Self {
        let mut new_inds: Vec<Individual> = Vec::new();
        
        // Loop through all individuals
        for i in self.individuals.clone().into_iter() {
            for _ in 0..self.num_clones {
                new_inds.push(i.mutate(self.mut_count.try_into().unwrap()))
            }
        }
        
        new_inds.sort_unstable_by(|a, b| {
            distance(&a.content, &self.target).partial_cmp(&distance(&b.content, &self.target)).unwrap()
        });
        
        if self.gen_count < 5 {
            let mut unique_parents = HashMap::new();
            for x in &new_inds {
                unique_parents.entry(x.parent.clone()).or_insert(x.clone());
            };
            
            for x in &new_inds[0..(self.survivors as usize)] {
                unique_parents.remove(&x.parent);
            }
            
            if unique_parents.len() > 0 {
                let mut counter: usize = 0;
                
                for i in unique_parents {
                    new_inds[25 + counter] = i.1;
                    counter += 1;
                }
                
                return Self {
                    individuals: new_inds[0..(self.survivors as usize + counter)].to_vec(),
                    target: self.target.to_owned(),
                    num_clones: self.num_clones,
                    survivors: self.survivors,
                    mut_count: self.mut_count,
                    gen_count: self.gen_count + 1
                }
            }
            
        }
        
        return Self {
            individuals: new_inds[0..(self.survivors as usize)].to_vec(),
            target: self.target.to_owned(),
            num_clones: self.num_clones,
            survivors: self.survivors,
            mut_count: self.mut_count,
            gen_count: self.gen_count + 1
        }

    }
}