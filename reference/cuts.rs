
    // let mut memory: HashMap<String, String> = HashMap::new();
    // let mut functions: HashMap<String, Position> = HashMap::new();
    // let mut stack = Vec::<Position>::new();
    // let mut current_position = Position{linenumber: 0, indentation: 0};

    
fn count_indentation(text: &String) -> usize {
    text.len() - text.trim_start_matches("\t").len()
}

    assert_eq!(match_if(&String::from("IF a == b")), Some((String::from("a"), String::from("b"))));
    assert_eq!(match_if(&String::from("IF a == ")), None);
    assert_eq!(match_if(&String::from("IF  == b")), None);
    assert_eq!(match_if(&String::from("IFa == b")), None);
    assert_eq!(match_if(&String::from("IFa == b")), None);
    

    assert_eq!(match_def(&String::from("DEF a")), Some(String::from("a")));
    assert_eq!(match_def(&String::from("DEF ")), None);
    assert_eq!(match_def(&String::from("DEFa")), None);
    assert_eq!(match_def(&String::from("DE a")), None);