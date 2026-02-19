use std::env;
use std::fs::File;
// use std::path::Path;
use std::collections::HashMap;
// use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
// use num_traits::pow;



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
    if c.len() == 3
    { 
        let mut chars = c.chars();

        if let Some(first_char) = chars.next() 
        {
            if first_char.to_string() != "1" && first_char.to_string() != "0"
            {
                let key = first_char.to_string();
                
                if  let Some(value) = map.get(&key) 
                {
                    print!("{} ", value);
                }
            }
        }

        let result: String = c.chars().skip(1).take(2).collect();
        let results: String = c.chars().skip(2).take(1).collect();

        if map.contains_key(&result.to_string())
        {
            let value = map.get(&result.to_string()).unwrap();
            print!("cent ");
            print!("{} ", value);
        }
        else if map.contains_key(&results.to_string())
        {
            let value = map.get(&results.to_string()).unwrap();
            print!("cent ");
            print!("{} ", value);
        }
        else 
        {
            println!("cents");
        }
    }
    else 
    {
        if map.contains_key(&c.to_string())
        {
            let value = map.get(&c.to_string()).unwrap();
            print!("{} ", value);
        }
    }
}


fn main()
{
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() != 2 || !check_args(&args[1])
    {
        println!("Error");
        return ;
    }

    let file = File::open("numbers.dict").expect("Failure open");
    // println!("{:?}", file);
    let file = BufReader::new(file);
    let mut map = HashMap::new();
let puissance = HashMap::from([
    (3, "mille"),
    (6, "million"),
    (9, "milliard"),
    (12, "billion"),
    (15, "billiard"),
    (18, "trillion"),
    (21, "trilliard"),
    (24, "quadrillion"),
    (27, "quadrilliard"),
    (30, "quintillion"),
    (33, "quintilliard"),
    (36, "sextillion"),
]);
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
        let vec = split_in_threes(&args[1]); // faire un fonction qui va decouper en 3 et renvoyer un vecteur
        let mut len = (vec.len() - 1) * 3;
        let v_iter = vec.iter();
        for i in v_iter
        {
            algorithm(&i, &map, len);
            if puissance.contains_key(&len)
            {
                let value = puissance.get(&len).unwrap();
                print!("{} ", value);
            }
            if len != 0
                {len -= 3;}
        }
    }
}



fn split_in_threes(num: &str) -> Vec<String> 
{
    let mut result = Vec::new();
    let mut s = num;

    while !s.is_empty() 
    {
        let split_point = if s.len() > 3 { s.len() - 3 } else { 0 };
        result.push(s[split_point..].to_string());
        s = &s[..split_point];
    }

    result.reverse();
    result
}


// */
 
    // 123456789
    // 9 - 3 = 6
    // 789 to vec
    // 123456
    // 6 - 3
    // 456 to vec

    // 123
    // 789 456 123

    // 123 456 789
//  */
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