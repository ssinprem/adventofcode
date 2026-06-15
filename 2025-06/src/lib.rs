
pub mod part1 {
    use std::collections::HashMap;
    pub struct Quiz {
        num: Vec<u64>,
        result: u64,
    }
    
    pub fn solve(file: String) -> u64 {
        let mut quizs = HashMap::<usize, Quiz>::new();
        file.lines().for_each(|line| {
            for (i,str) in line.split_whitespace().enumerate() {
                let quiz: &mut Quiz = quizs.entry(i)
                    .or_insert(Quiz {
                            num: Vec::new(),
                            result: 0,
                        }
                    );
                if let Ok(num) = str.parse::<u64>() {
                    quiz.num.push(num);
                } else if str.starts_with('*') {
                    quiz.result = quiz.num.iter().product();
                } else if str.starts_with('+') {
                    quiz.result = quiz.num.iter().sum();
                }
            }
        });
        quizs.values().map(|q| q.result).sum::<u64>()
    }
}

pub mod part2 {
    use std::collections::HashMap;
    
    pub struct Quiz {
        num: HashMap<usize, u64>,
        result: u64,
        start: usize
    }

    pub fn solve(file: String) -> u64 {
        let mut quizs = HashMap::<usize, Quiz>::new();
        file.lines().for_each(|line| {
            let mut quiz_id = 0;
            let mut pre_char = 'x';
            let mut first = true;
            for (i,char) in line.chars().enumerate() {
                let quiz: &mut Quiz = quizs.entry(quiz_id)
                .or_insert(Quiz {
                            num: HashMap::new(),
                            result: 0,
                            start: i
                        }
                    );
                if char == '*' {
                    quiz.result = quiz.num.values().product();
                    println!("#{:<4} {:?} = {}", quiz_id, quiz.num, quiz.result);
                    quiz_id += 1;
                } else if char == '+' {
                    quiz.result = quiz.num.values().sum();
                    println!("#{:<4} {:?} = {}", quiz_id, quiz.num, quiz.result);
                    quiz_id += 1;
                } else if char == ' ' {
                    if pre_char != ' ' && !first {
                        quiz_id += 1;
                    }
                } else {
                    first = false;
                    let index = i-quiz.start;
                    let num = quiz.num.entry(index).or_insert(0);
                    *num = *num * 10 + char.to_string().parse::<u64>().unwrap();
                }
                pre_char = char;
            }
        });
        quizs.values().map(|q| q.result).sum::<u64>()
    }
}
