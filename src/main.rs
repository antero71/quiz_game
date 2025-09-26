#![allow(unused)]

use std::io::{self, BufRead, Write};
use std::fs;
use std::io::prelude::*;
use serde::Deserialize;

// We need read quiz questions from a file
// We need to store the quiz questions and answers in a struct
// We need to ask the questions to the user
// We need to check the answers and keep score
// We need to display the score at the end

#[derive(Debug, Deserialize)]
struct Question {
    #[serde(rename = "question")]
    text: String,
    options: Vec<String>,
    correct: String,
}

fn main() {
    let questions = read_quiz_file("src/quiz_rust.yaml");
    let mut score = 0;

    for q in &questions {
        if ask_question(q) {
            score += 1;
        }
    }
    let percentage = (score as f32 / questions.len() as f32) * 100.0;
    println!("You got {} out of {} correct ({:.2})%.", score, questions.len(),percentage);
}

/// Reads quiz questions from a YAML file and returns a vector of Question structs.
/// 
/// # Arguments
/// * `filename` - The path to the YAML file containing the quiz questions.
/// 
/// # Panics
/// Panics if the file cannot be read or parsed.
fn read_quiz_file(filename: &str) -> Vec<Question> {
    let file_content = fs::read_to_string(filename).expect("Could not read file");
    let questions: Vec<Question> = serde_yaml::from_str(&file_content).expect("YAML parse error");
    questions
}

/// Asks a single quiz question to the user, reads the answer, and checks if it is correct.
/// 
/// # Arguments
/// * `q` - A reference to the Question to be asked.
/// 
/// # Returns
/// * `true` if the user's answer is correct, `false` otherwise.
fn ask_question(q: &Question) -> bool {
    println!("{}", q.text);
    for (i, opt) in q.options.iter().enumerate() {
        let letter = (b'A' + i as u8) as char;
        println!("{}: {}", letter, opt);
    }
    print!("Your answer: ");
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();

    if answer.trim().eq_ignore_ascii_case(&q.correct) {
        println!("Correct!\n");
        true
    } else {
        println!("Wrong! The correct answer is {}.\n", q.correct);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Luo testikysymykset YAML-muodossa
    fn sample_yaml() -> &'static str {
        r#"
- question: What is Rust?
  options:
    - A programming language
    - A kind of cheese
    - A car brand
    - A planet
  correct: A

- question: What does 'let' do in Rust?
  options:
    - Declares a variable
    - Prints text
    - Imports a crate
    - Ends a program
  correct: A
"#
    }

    #[test]
    fn test_read_quiz_file_from_str() {
        let questions: Vec<Question> = serde_yaml::from_str(sample_yaml()).unwrap();
        assert_eq!(questions.len(), 2);
        assert_eq!(questions[0].text, "What is Rust?");
        assert_eq!(questions[0].options[2], "A car brand");
        assert_eq!(questions[1].correct, "A");
    }

    #[test]
    fn test_answer_checking() {
        let q = Question {
            text: "What is Rust?".to_string(),
            options: vec![
                "A programming language".to_string(),
                "A kind of cheese".to_string(),
                "A car brand".to_string(),
                "A planet".to_string(),
            ],
            correct: "A".to_string(),
        };
        // Simuloi oikea ja väärä vastaus
        assert!(check_answer(&q, "A"));
        assert!(check_answer(&q, "a"));
        assert!(!check_answer(&q, "B"));
    }

    // Apufunktio testaukseen, koska ask_question lukee stdinistä
    fn check_answer(q: &Question, user_input: &str) -> bool {
        user_input.trim().eq_ignore_ascii_case(&q.correct)
    }
}