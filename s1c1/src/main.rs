use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;
#[macro_use]
extern crate lazy_static;

fn main() {
    println!("Hello, world!");
}


//s1c1
fn hex2base64(input: &str) -> String {
    let hex = hex::decode(input).unwrap();
    let base64 = base64::encode(&hex);
    base64
}

fn xor(input1: &str, input2: &str) -> String {
    let hex1 = hex::decode(input1).unwrap();
    let hex2 = hex::decode(input2).unwrap();
    let xored = hex1.iter().zip(hex2.iter()).map(|(a, b)| a ^ b).collect::<Vec<u8>>();
    hex::encode(&xored)
}

lazy_static!{
    static ref LETTER_FREQUENCY: HashMap<char, u64> = vec![
        //Frequency of letters in english language, 
        //in percent of texts in english language
        (' ',1293934),
        ('(',628),
        (',',83174),
        ('0',299),
        ('4',93),
        ('8',40),
        ('<',468),
        ('@',8),
        ('D',15683),
        ('H',18462),
        ('L',23858),
        ('P',11939),
        ('T',39800),
        ('X',606),
        ('`',1),
        ('d',133779),
        ('h',218406),
        ('l',146161),
        ('p',46525),
        ('t',289975),
        ('x',4688),
        ('|',33),
        ('#',1),
        ('\'',31069),
        ('/',5),
        ('3',330),
        ('7',41),
        (';',17199),
        ('?',10476),
        ('C',21497),
        ('G',11164),
        ('K',6196),
        ('O',33209),
        ('S',34011),
        ('W',16496),
        ('[',2085),
        ('_',71),
        ('c',66688),
        ('g',57035),
        ('k',29212),
        ('o',281391),
        ('s',214978),
        ('w',72894),
        ('\n',124456),
        ('"',470),
        ('&',21),
        ('*',63),
        ('.',78025),
        ('2',366),
        ('6',63),
        (':',1827),
        ('>',441),
        ('B',15413),
        ('F',11713),
        ('J',2067),
        ('N',27338),
        ('R',28970),
        ('V',3580),
        ('Z',532),
        ('b',46543),
        ('f',68803),
        ('j',2712),
        ('n',215924),
        ('r',208894),
        ('v',33989),
        ('z',1099),
        ('~',1),
        ('!',8844),
        ('%',1),
        (')',629),
        ('-',8074),
        ('1',928),
        ('5',82),
        ('9',948),
        ('=',1),
        ('A',44486),
        ('E',42583),
        ('I',55806),
        ('M',15872),
        ('Q',1178),
        ('U',14129),
        ('Y',9099),
        (']',2077),
        ('a',244664),
        ('e',404621),
        ('i',198184),
        ('m',95580),
        ('q',2404),
        ('u',114818),
        ('y',85271),
        ('}',2),
    ].into_iter().collect();
}

fn calculate_score(input: &str) -> u64 {
    let mut score = 0;
    for c in input.chars() {
        if let Some(v) = LETTER_FREQUENCY.get(&c.to_lowercase().next().unwrap()) {
            score += v;
        }
    }
    score
}

fn find_highest_scored_xored(input1: &str) -> (Option<String>, u64, char){
    let hex = hex::decode(input1).unwrap();
    let mut highest = (None, 0 as u64, ' ');
    for i in 0..255 {
        let xored = std::str::from_utf8(&hex.iter().map(|x| x ^ i).collect::<Vec<u8>>()).unwrap_or_default().to_owned();
        let score = calculate_score(&xored);
        //println!("{}, score: {} ", &xored, score);
        if score > highest.1 {
            highest = (Some(xored), score, std::char::from_u32(i as u32).unwrap());
        }
    }
    //println!("{}, score: {} ", highest.0.unwrap(), highest.1);
    highest
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_s1c1() {
        assert_eq!(hex2base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn test_s1c2() {
        assert_eq!(xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"), "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn test_s1c3() {
        assert_eq!(find_highest_scored_xored("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").0.unwrap(), "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn test_s1c4() {
        let file = File::open("s1c4.txt").unwrap();
        let results = BufReader::new(file)
            .lines()
            .map(|x| x.unwrap_or_default())
            .map(|x| find_highest_scored_xored(&x))
            .filter(|x| x.0.is_some())
            .map(|x| (x.0.unwrap(), x.1, x.2))
            .max_by(|x, y| x.1.cmp(&y.1))
            .unwrap();
            
        let result_str = &results.0;

        assert_eq!(result_str, "Now that the party is jumping\n");
    }

}