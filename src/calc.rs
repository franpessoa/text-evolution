use std::collections::HashMap;

use rand::{thread_rng, Rng};

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Individual {
    target: String,
    content: String,
    parent: Option<Box<Individual>>
}

pub struct Simulation {
    target: String,
    individuals: Vec<Individual>,
    num_clones: u32,
    survivors: u32,
    mut_count: u16,
    gen_count: u32
}

impl Individual {
    fn new(t: String, c: String, parent: Option<Box<Individual>>) -> Self { 
        match parent {
            Some(y) => return Self { target: t, content: c, parent: Some(y) },
            None => return Self {target: t, content: c, parent: None}
        }
    }
    
    fn mutate(&self, count: u8) -> Self {
        let mut content = self.content.clone();
        let all_chars: String = format!(
            "{}{}{}{}",
            "abcdefghijklmnopqrstuvwxyz",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "1234567890",
            "\'\"!@#$%&*()_+-=¹²³£¢¬{[]}§ªº |<>:;,."
        );

        let mut rng = thread_rng();
        
        // Generating and applying random mutations
        for _ in 0..count {
            let mutation = (
                rng.gen_range(0..count), 
                all_chars.chars().nth(
                    rng.gen_range(0..all_chars.chars().count()).try_into().unwrap()
                ).unwrap()
            );
            
            content = content.chars().enumerate().map(|(idx, ch)| {
                if mutation.0 == idx as u8 { return mutation.1 } else {return ch} 
            }).collect::<String>();
        };
        
        return Self::new(self.target.to_owned(), content, Some(Box::new(self.clone())))
    }
    
    fn is_target(&self) -> bool {
        return self.content == self.target;
    }
}


impl Simulation {
    async fn advance_gen(&self) -> Self {
        let mut new_inds: Vec<Individual> = Vec::new();
        
        // Loop through all individuals
        for i in self.individuals.clone().into_iter() {
            for _ in 0..self.num_clones {
                new_inds.push(i.mutate(self.mut_count.try_into().unwrap()))
            }
        }
        
        new_inds.sort_unstable_by(|a, b| {
            distance(&a.content, &a.target).partial_cmp(&distance(&b.content, &b.target)).unwrap()
        });
        
        if self.gen_count < 5 {
            let mut unique_parents = HashMap::new();
            for x in &new_inds {
                unique_parents.entry(x.parent.clone()).or_insert(x.clone());
            };
            
            for x in &new_inds[0..(self.survivors as usize)].to_vec() {
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