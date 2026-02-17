use std::env;
use std::fs::File;
// use std::path::Path;
use std::collections::HashMap;
// use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use num_traits::pow;

fn  check_args(c: &str) -> bool
{
    for caractere in c.chars()
    {
        if caractere < '0' || caractere > '9'
        {
            return false
        }
    }
    true
}

fn algorithm(c: &str, map: &HashMap<String, String>, _len: usize) 
{
    // 123
    // let  mut i: usize = len;

    if c.len() == 3
    { 
        if let Some(first_char) = c.chars().next() 
        {
            let key = first_char.to_string();
            
            if let Some(value) = map.get(&key) {
                println!("{}", value);
            }
        }

        if map.contains_key(&c.to_string())
        {
            let value = map.get(&c.to_string()).unwrap();
            println!("{}", value);
        }
            // let nbr =  caractere.parse::<i32>().unwrap();
        // println!("{res}");
        // for caractere in c.chars()
        // {
        //     // let nbr: usize = caractere.parse().unwrap();

        //     let  nbr = (caractere.to_string()).parse::<usize>().unwrap();
        //     let  res = nbr * pow(10, i);
            
        //     i -= 1;
        // }

    }

}

fn main()
{
    let args: Vec<String> = env::args().collect();

    // println!("{:?}", args);
    if args.len() != 2 ||!check_args(&args[1])
    {
        println!("Error");
        return ;
    }

    let file = File::open("numbers.dict").expect("Failure open");
    // println!("{:?}", file);
    let file = BufReader::new(file);
    let mut map = HashMap::new();
    // let mut order = HashMap::new();

    for line in file.lines() 
    {
        let l = line.unwrap();
        if let Some((key, val)) = l.split_once(':') 
        {
            map.insert(key.trim().to_string(),val.trim().to_string());
        }
    }   
    if map.contains_key(&args[1])
    {
        let value = map.get(&args[1]).unwrap();
        println!("{}", value);
    }
    else
    {
        let len = args[1].len() - 1;
        let mut _puissance = pow(10, len);
        // println!("{} {}", args[1].len(), puissance);
        _puissance = pow(10, args[1].len() - 2);
        // println!("{} {}", args[1].len(), puissance);
        // len--;
        algorithm(&args[1], &map, len);
    }
}
// check_puissance
// 421

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_add() {
//          assert!algorithm(c, map, len);
//     }

//     #[test]
//     fn test_bad_add() {
//         // This assert would fire and test will fail.
//         // Please note, that private functions can be tested too!
//         assert_eq!(bad_add(1, 2), 3);
//     }
// }