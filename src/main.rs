use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn main() -> std::io::Result<()> {
    //let content = std::fs::read_to_string("input").expect("could not read file!");

    let mut f = std::fs::File::open("input")?;

    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    let bogdan = String::from_utf8(buffer).unwrap();

    let lmao = bogdan.split("\n\n");
    let mut total = 0;

    for x in lmao
    {
        let id = x.replace("\n", " ");
        let mut entries = HashMap::<&str,&str>::new();

        for y in id.split(" ")
        {
            let z: Vec<&str> = y.split(":").collect();
            entries.insert(z[0], z[1]);
        }

        if entries.contains_key("byr")
        {
            let value = entries.get("byr").unwrap();
            let year = value.parse::<i32>().unwrap();
            if ! (year >= 1920 && year <= 2002)
            {
                continue;
            }
        }
        else
        {
            continue;
        }

        if entries.contains_key("iyr")
        {
            let value = entries.get("iyr").unwrap();
            let year = value.parse::<i32>().unwrap();
            if ! (year >= 2010 && year <= 2020)
            {
                continue;
            }
        }
        else
        {
            continue;
        }

        if entries.contains_key("eyr")
        {
            let value = entries.get("eyr").unwrap();
            let year = value.parse::<i32>().unwrap();
            if ! (year >= 2020 && year <= 2030)
            {
                continue;
            }
        }
        else
        {
            continue;
        }

        if entries.contains_key("hgt")
        {
            let value = entries.get("hgt").unwrap();
            if value.contains("cm")
            {
                let height = &value[0..3].parse::<i32>();
                match height {
                    Ok(_f) => {

                    },
                    Err(_e)=> {
                        continue;
                    }
                }
                if !(height.as_ref().unwrap() >= &150 && height.as_ref().unwrap() <= &193)
                {
                    continue;
                }
            }
            else if value.contains("in")
            {
                
                let height = &value[0..2].parse::<i32>();
                match height {
                    Ok(_f) => {

                    },
                    Err(_e)=> {
                        continue;
                    }
                }
                if !(height.as_ref().unwrap() >= &59 && height.as_ref().unwrap() <= &76)
                {
                    continue;
                }
            }
            else
            {
                continue;
            }
        }
        else
        {
            continue;
        }

        if entries.contains_key("hcl")
        {
            let value = entries.get("hcl").unwrap();
            let re = Regex::new("#([0-9]|[a-f]){6}").unwrap();
            if !re.is_match(value)
            {
                continue;
            }
        }
        else
        {
            continue;
        }

        if entries.contains_key("ecl")
        {
            let value = entries.get("ecl").unwrap();
            let re = Regex::new("((amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth))").unwrap();
            if !re.is_match(value)
            {
                continue;
            }
        }
        else
        {
            continue;
        }

        if entries.contains_key("pid")
        {
            let value = entries.get("pid").unwrap();
            let re = Regex::new("\\d{9}").unwrap();
            if !re.is_match(value) || value.len() != 9
            {
                continue;
            }
        }
        else
        {
            continue;
        }
        
        total += 1;
        // println!("{:?}", entries);
    }
    println!("{}", total);

    Ok(())
}
