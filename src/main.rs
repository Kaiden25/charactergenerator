extern crate rand;

use std::io;
use rand::Rng;
//use std::fs::File;
//use std::io::prelude::*;

#[derive(Debug)]
struct Race {
    name: String,
    strength_modifier: i8,
    dexterity_modifier: i8,
    constitution_modifier: i8,
    comprehension_modifier: i8,
    willpower_modifier: i8,
    presence_modifier: i8,
}

#[derive(Debug)]
struct Character {
    race: Race,
    name: String,
    strength: u8,
    dexterity: u8,
    constitution: u8,
    comprehension: u8,
    willpower: u8,
    presence: u8,
}

fn main() {
    println!("Own System Character Genarator");

    loop {
        println!("What is your race?");
        println!("1 for Human");
        println!("2 for Beast");
        println!("3 for Devil");
        println!("4 for Spirit");
        println!("\nTo terminate the progrem input a letter");

        let mut race_choise = String::new();

        io::stdin().read_line(&mut race_choise)
            .expect("Failed to read line");

        let race_choise: u32 = match race_choise.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

    if race_choise == 1 {
        let race = human();
        println!("{}", race);
/*
        let mut file = File::create("char.txt");
        file.write_all(race.as_bytes());
        break;*/
    } else if race_choise == 2 {
        let race = beast();
        println!("{}", race);
        //break;
    } else if race_choise == 3 {
        let race = devil();
        println!("{}", race);
        //break;
    } else if race_choise == 4 {
        let race = spirit();
        println!("{}", race);
        //break;
    }
    }
}


// Race Human
fn human() -> String {
    let race_grade = rand::thread_rng().gen_range(1, 21);
    let race_tier = rand::thread_rng().gen_range(1, 21);
    let mut race_overview = String::new();
    
    if race_tier <= 16 {
        race_overview.push_str("Average ");
        if race_grade <= 16 {
            race_overview.push_str("Normal ");
            let b_stats = average_normal_human();
            race_overview.push_str("Human");
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Divine Body ");
            let b_stats = average_divine_body_human();
            race_overview.push_str("Human");
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("Bloodline ");
            let b_stats = average_bloodline_human();
            race_overview.push_str("Human");
            race_overview.push_str(&b_stats);
        }
    } else if race_tier <= 17 && race_tier <= 19 {
        race_overview.push_str("Superior ");
        if race_grade <= 16 {
            race_overview.push_str("Normal ");
            let b_stats = superior_normal_human();
            race_overview.push_str("Human");
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Divine Body ");
            let b_stats = superior_divine_body_human();
            race_overview.push_str("Human");
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("Bloodline ");
            let b_stats = superior_bloodline_human();
            race_overview.push_str("Human");
            race_overview.push_str(&b_stats);
        }
    } else if race_tier == 20 {
        race_overview.push_str("Extreme ");
        if race_grade <= 16 {
            race_overview.push_str("Normal ");
            let b_stats = extreme_normal_human();
            race_overview.push_str("Human");
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Divine Body ");
            let b_stats = extreme_divine_body_human();
            race_overview.push_str("Human");
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("Bloodline ");
            let b_stats = extreme_bloodline_human();
            race_overview.push_str("Human");
            race_overview.push_str(&b_stats);
        }
    }
    
    return race_overview;}
//Human Attributes
fn average_normal_human() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_normal_human() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2 + 1;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2 + 1;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2 + 1;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 + 1;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2 + 1;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2 + 1;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_normal_human() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let str3 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2 + str3 + 2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dex3 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2 + dex3 + 2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let con3 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2 + con3 + 2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comp3 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 + comp3 + 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let will3 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2 + will3 + 2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let pres3 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2 + pres3 + 2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    base_stats
}

fn average_divine_body_human() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    base_stats    
}

fn superior_divine_body_human() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2 + 1;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2 + 1;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2 + 1;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 + 1;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2 + 1;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2 + 1;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_divine_body_human() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let str3 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2 + str3 + 2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dex3 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2 + dex3 + 2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let con3 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2 + con3 + 2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comp3 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 + comp3 + 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let will3 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2 + will3 + 2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let pres3 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2 + pres3 + 2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn average_bloodline_human() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_bloodline_human() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2 + 1;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2 + 1;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2 + 1;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 + 1;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2 + 1;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2 + 1;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_bloodline_human() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let str3 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2 + str3 + 2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dex3 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2 + dex3 + 2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let con3 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2 + con3 + 2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comp3 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 + comp3 + 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let will3 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2 + will3 + 2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let pres3 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2 + pres3 + 2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
/*
// Human Devine Body 
fn divine_body() -> String {
    let body = rand::thread_rng().gen_range(1, 21);
    let mut body_overview = String::new();
    return body_overview;
}
// Human Bloodline
*/

// Race Beast
fn beast() -> String {
    let race_grade = rand::thread_rng().gen_range(1, 21);
    let race_tier = rand::thread_rng().gen_range(1, 21);
    let mut race_overview = String::new();
    
    if race_tier <= 16 {
        race_overview.push_str("Average ");
        if race_grade <= 16 {
            race_overview.push_str("Normal ");
            let b_stats = average_normal_beast();
            race_overview.push_str("Beast");
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Monstrous ");
            let b_stats = average_monstrous_beast();
            race_overview.push_str("Beast");
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("Godbeast ");
            let b_stats = average_godbeast();
            race_overview.push_str(&b_stats);
        }
    } else if race_tier <= 17 && race_tier <= 19 {
        race_overview.push_str("Superior ");
        if race_grade <= 16 {
            race_overview.push_str("Normal ");
            let b_stats = superior_normal_beast();
            race_overview.push_str("Beast");
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Monstrous ");
            let b_stats = superior_monstrous_beast();
            race_overview.push_str("Beast");
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("Godbeast ");
            let b_stats = superior_godbeast();
            race_overview.push_str(&b_stats);
        }
    } else if race_tier == 20 {
        race_overview.push_str("Extreme ");
        if race_grade <= 16 {
            race_overview.push_str("Normal ");
            let b_stats = extreme_normal_beast();
            race_overview.push_str("Beast");
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Monstrous ");
            let b_stats = extreme_monstrous_beast();
            race_overview.push_str("Beast");
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("Godbeast ");
            let b_stats = extreme_godbeast();
            race_overview.push_str(&b_stats);
        }
    }
    race_overview.push_str("Beast");
    return race_overview;}
// Beast Attributes
/*
Strength        2d4     +2
Dexterity       2d4     +1
Constitution    2d4     +2
Comprehension   2d4     -2
Willpower       2d4
Presence        2d4
*/
fn average_normal_beast() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2 + 2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2 + 1;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2 + 2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 -2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_normal_beast() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2 + 3;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2 + 2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2 + 3;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 - 1;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_normal_beast() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let str3 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2 + str3 + 4;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dex3 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2 + dex3 + 3;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let con3 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2 + con3 + 4;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comp3 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 + comp3;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let will3 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2 + will3;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let pres3 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2 + pres3;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn average_monstrous_beast() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2 + 2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2 + 1;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2 + 2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 -2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_monstrous_beast() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2 + 3;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2 + 2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2 + 3;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 - 1;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_monstrous_beast() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let str3 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2 + str3 + 4;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dex3 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2 + dex3 + 3;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let con3 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2 + con3 + 4;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comp3 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 + comp3;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let will3 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2 + will3;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let pres3 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2 + pres3;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn average_godbeast() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2 + 2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2 + 1;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2 + 2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 - 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_godbeast() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2 + 3;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2 + 2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2 + 3;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 - 1;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_godbeast() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let str3 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2 + str3 + 4;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dex3 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2 + dex3 + 3;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let con3 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2 + con3 + 4;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comp3 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 + comp3;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let will3 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2 + will3;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let pres3 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2 + pres3;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}

// Race Devil
fn devil() -> String {
    let race_grade = rand::thread_rng().gen_range(1, 21);
    let race_tier = rand::thread_rng().gen_range(1, 21);
    let mut race_overview = String::new();
    
    if race_tier <= 16 {
        race_overview.push_str("Average ");
        if race_grade <= 16 {
            race_overview.push_str("Soldier ");
            let b_stats = average_soldier_devil();
            race_overview.push_str("Devil");
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Lord ");
            let b_stats = average_lord_devil();
            race_overview.push_str("Devil");
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("Overlord ");
            let b_stats = average_overlord_devil();
            race_overview.push_str("Devil");
            race_overview.push_str(&b_stats);
        }
    } else if race_tier <= 17 && race_tier <= 19 {
        race_overview.push_str("Superior ");
        if race_grade <= 16 {
            race_overview.push_str("Soldier ");
            let b_stats = superior_soldier_devil();
            race_overview.push_str("Devil");
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Lord ");
            let b_stats = superior_lord_devil();
            race_overview.push_str("Devil");
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("Overlord ");
            let b_stats = superior_overlord_devil();
            race_overview.push_str("Devil");
            race_overview.push_str(&b_stats);
        }
    } else if race_tier == 20 {
        race_overview.push_str("Extreme ");
        if race_grade <= 16 {
            race_overview.push_str("Soldier ");
            let b_stats = extreme_soldier_devil();
            race_overview.push_str("Devil");
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Lord ");
            let b_stats = extreme_lord_devil();
            race_overview.push_str("Devil");
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("Overlord ");
            let b_stats = extreme_overlord_devil();
            race_overview.push_str("Devil");
            race_overview.push_str(&b_stats);
        }
    }
    return race_overview;}
// Devil Attributes
/*
Strength        2d4
Dexterity       2d4
Constitution    2d4     -2
Comprehension   2d4     +1
Willpower       2d4     +2
Presence        2d4     +2
*/
fn average_soldier_devil() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2 - 2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 + 1;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2 + 2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2 + 2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_soldier_devil() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2 - 1;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 + 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2 + 3;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2 + 3;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_soldier_devil() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let str3 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2 + str3;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dex3 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2 + dex3;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let con3 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2 + con3;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comp3 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 + comp3 + 3;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let will3 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2 + will3 + 4;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let pres3 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2 + pres3 + 4;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn average_lord_devil() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2 - 2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 + 1;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2 + 2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2 + 2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_lord_devil() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2 - 1;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 + 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2 + 3;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2 + 3;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_lord_devil() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let str3 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2 + str3;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dex3 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2 + dex3;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let con3 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2 + con3;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comp3 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 + comp3 + 3;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let will3 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2 + will3 + 4;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let pres3 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2 + pres3 + 4;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn average_overlord_devil() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2 - 2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 + 1;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2 + 2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2 + 2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_overlord_devil() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2 - 1;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 + 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2 + 3;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2 + 3;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_overlord_devil() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let str3 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2 + str3;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dex3 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2 + dex3;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let con3 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2 + con3;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comp3 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 + comp3 + 3;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let will3 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2 + will3 + 4;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let pres3 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2 + pres3 + 4;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}

// Race Spirit
fn spirit() -> String {
    let race_grade = rand::thread_rng().gen_range(1, 21);
    let race_tier = rand::thread_rng().gen_range(1, 21);
    let mut race_overview = String::new();
    
    if race_tier <= 16 {
        race_overview.push_str("Average ");
        if race_grade <= 16 {
            race_overview.push_str("Minor Spirit");
            let b_stats = average_minor_spirit();
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Major Spirit");
            let b_stats = average_major_spirit();
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("True Spirit");
            let b_stats = average_true_spirit();
            race_overview.push_str(&b_stats);
        }
    } else if race_tier <= 17 && race_tier <= 19 {
        race_overview.push_str("Superior ");
        if race_grade <= 16 {
            race_overview.push_str("Minor Spirit");
            let b_stats = superior_minor_spirit();
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Major Spirit");
            let b_stats = superior_major_spirit();
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("True Spirit");
            let b_stats = superior_true_spirit();
            race_overview.push_str(&b_stats);
        }
    } else if race_tier == 20 {
        race_overview.push_str("Extreme ");
        if race_grade <= 16 {
            race_overview.push_str("Minor Spirit");
            let b_stats = extreme_minor_spirit();
            race_overview.push_str(&b_stats);
        } else if race_grade >= 17 && race_grade <= 19 {
            race_overview.push_str("Major Spirit");
            let b_stats = extreme_major_spirit();
            race_overview.push_str(&b_stats);
        } else if race_grade == 20 {
            race_overview.push_str("True Spirit");
            let b_stats = extreme_true_spirit();
            race_overview.push_str(&b_stats);
        }
    }
    return race_overview;}
//Spirit Attributes
/*
Strength        2d4     -2
Dexterity       2d4
Constitution    2d4
Comprehension   2d4     +2
Willpower       2d4     +1
Presence        2d4     +1
*/
fn average_minor_spirit() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");

    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2 - 2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 + 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2 + 1;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2 + 1;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_minor_spirit() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2 - 1;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 + 3;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2 + 2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2 + 2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_minor_spirit() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 5);
    let str2 = rand::thread_rng().gen_range(1, 5);
    let str3 = rand::thread_rng().gen_range(1, 5);
    let strength = str1 + str2 + str3;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 5);
    let dex2 = rand::thread_rng().gen_range(1, 5);
    let dex3 = rand::thread_rng().gen_range(1, 5);
    let dexterity = dex1 + dex2 + dex3;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 5);
    let con2 = rand::thread_rng().gen_range(1, 5);
    let con3 = rand::thread_rng().gen_range(1, 5);
    let constitution = con1 + con2 + con3;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 5);
    let comp2 = rand::thread_rng().gen_range(1, 5);
    let comp3 = rand::thread_rng().gen_range(1, 5);
    let comprehension = comp1 + comp2 + comp3 + 4;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 5);
    let will2 = rand::thread_rng().gen_range(1, 5);
    let will3 = rand::thread_rng().gen_range(1, 5);
    let willpower = will1 + will2 + will3 + 3;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 5);
    let pres2 = rand::thread_rng().gen_range(1, 5);
    let pres3 = rand::thread_rng().gen_range(1, 5);
    let presence = pres1 + pres2 + pres3 + 3;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn average_major_spirit() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2 - 2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 + 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2 + 1;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2 + 1;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_major_spirit() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2 - 1;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 + 3;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2 + 2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2 + 2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_major_spirit() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 7);
    let str2 = rand::thread_rng().gen_range(1, 7);
    let str3 = rand::thread_rng().gen_range(1, 7);
    let strength = str1 + str2 + str3;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 7);
    let dex2 = rand::thread_rng().gen_range(1, 7);
    let dex3 = rand::thread_rng().gen_range(1, 7);
    let dexterity = dex1 + dex2 + dex3;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 7);
    let con2 = rand::thread_rng().gen_range(1, 7);
    let con3 = rand::thread_rng().gen_range(1, 7);
    let constitution = con1 + con2 + con3;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 7);
    let comp2 = rand::thread_rng().gen_range(1, 7);
    let comp3 = rand::thread_rng().gen_range(1, 7);
    let comprehension = comp1 + comp2 + comp3 + 4;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 7);
    let will2 = rand::thread_rng().gen_range(1, 7);
    let will3 = rand::thread_rng().gen_range(1, 7);
    let willpower = will1 + will2 + will3 + 3;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 7);
    let pres2 = rand::thread_rng().gen_range(1, 7);
    let pres3 = rand::thread_rng().gen_range(1, 7);
    let presence = pres1 + pres2 + pres3 + 3;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn average_true_spirit() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2 - 2;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 + 2;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2 + 1;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2 + 1;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn superior_true_spirit() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");
    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2 - 1;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 + 3;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2 + 2;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2 + 2;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}
fn extreme_true_spirit() -> String {
    let mut base_stats = String::new();
    base_stats.push_str("\n ");
    base_stats.push_str("\n ");

    let str1 = rand::thread_rng().gen_range(1, 9);
    let str2 = rand::thread_rng().gen_range(1, 9);
    let str3 = rand::thread_rng().gen_range(1, 9);
    let strength = str1 + str2 + str3;
    base_stats.push_str("Strengt: ");
    let s:String = strength.to_string();
    base_stats.push_str(&s);
    base_stats.push_str("\n ");
    
    let dex1 = rand::thread_rng().gen_range(1, 9);
    let dex2 = rand::thread_rng().gen_range(1, 9);
    let dex3 = rand::thread_rng().gen_range(1, 9);
    let dexterity = dex1 + dex2 + dex3;
    base_stats.push_str("Dexterity: ");
    let d:String = dexterity.to_string();
    base_stats.push_str(&d);
    base_stats.push_str("\n ");

    let con1 = rand::thread_rng().gen_range(1, 9);
    let con2 = rand::thread_rng().gen_range(1, 9);
    let con3 = rand::thread_rng().gen_range(1, 9);
    let constitution = con1 + con2 + con3;
    base_stats.push_str("Constitution: ");
    let c:String = constitution.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let comp1 = rand::thread_rng().gen_range(1, 9);
    let comp2 = rand::thread_rng().gen_range(1, 9);
    let comp3 = rand::thread_rng().gen_range(1, 9);
    let comprehension = comp1 + comp2 + comp3 + 4;
    base_stats.push_str("Comprehension: ");
    let c:String = comprehension.to_string();
    base_stats.push_str(&c);
    base_stats.push_str("\n ");

    let will1 = rand::thread_rng().gen_range(1, 9);
    let will2 = rand::thread_rng().gen_range(1, 9);
    let will3 = rand::thread_rng().gen_range(1, 9);
    let willpower = will1 + will2 + will3 + 3;
    base_stats.push_str("Willpower: ");
    let w:String = willpower.to_string();
    base_stats.push_str(&w);
    base_stats.push_str("\n ");

    let pres1 = rand::thread_rng().gen_range(1, 9);
    let pres2 = rand::thread_rng().gen_range(1, 9);
    let pres3 = rand::thread_rng().gen_range(1, 9);
    let presence = pres1 + pres2 + pres3 + 3;
    base_stats.push_str("Presence: ");
    let p:String = presence.to_string();
    base_stats.push_str(&p);
    base_stats.push_str("\n ");

    let d_stats = derived_attributes(strength, dexterity, constitution, comprehension, willpower, presence);
    base_stats.push_str(&d_stats);

    return base_stats;}

//Derived Attributes
fn derived_attributes(strength : u8, dexterity : u8, constitution : u8, comprehension : u8, willpower : u8, presence : u8) -> String {
    let mut derived_stats = String::new();
    derived_stats.push_str("\n ");

    //Armor Class : Armor Class is the defense value that armor grants the character. ==> TBD
    derived_stats.push_str("Armor Class: TBD \n ");

    //Fortitude : Fortitude is derived from a combination of Willpower and Constitution ==> Con + Will
    let fortitude = constitution + willpower;
    let f:String = fortitude.to_string();
    derived_stats.push_str("Fortitude: ");
    derived_stats.push_str(&f);
    derived_stats.push_str("\n ");

    //Agility : Agility is derived from a combination of Dexterity and Constitution ==> Dex + Con
    let agility = dexterity + constitution;
    let a:String = agility.to_string();
    derived_stats.push_str("Agility: ");
    derived_stats.push_str(&a);
    derived_stats.push_str("\n ");

    //Initiative : Initiative is derived from Dexterity and Strength/Willpower ==> Dex + (higher of Str/Will)
    let mut initiative = 0;
    if strength >= willpower {
        initiative = dexterity + strength;
    } else if strength < willpower {
        initiative = dexterity + willpower;
    }
    let i:String = initiative.to_string();
    derived_stats.push_str("Initiative: ");
    derived_stats.push_str(&i);
    derived_stats.push_str("\n ");

    //Enlightenment : Comprehension + Willpower ==> Comp + Will
    let enlightenment = dexterity + constitution;
    let e:String = enlightenment.to_string();
    derived_stats.push_str("Enlightenment: ");
    derived_stats.push_str(&e);
    derived_stats.push_str("\n ");

    //Renown : This attribute always starts as 0. It increases as the story progresses depending on the characters choices. ==> 0
    derived_stats.push_str("Renown: 0 \n ");

    //Vitality : Fortitude is derived from a combination of Strength and Constitution times two ((Str+Con)*2) ==> (Str + Con)*2
    let vitality = (strength + constitution)*2;
    let v:String = vitality.to_string();
    derived_stats.push_str("Vitality: ");
    derived_stats.push_str(&v);
    derived_stats.push_str("\n ");

    //Awareness : Comprehension + Presence ==> Comp + Pres
    let awareness = comprehension + presence;
    let a:String = awareness.to_string();
    derived_stats.push_str("Awareness: ");
    derived_stats.push_str(&a);
    derived_stats.push_str("\n ");

    return derived_stats;}