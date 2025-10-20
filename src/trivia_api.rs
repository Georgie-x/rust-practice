use serde_json::{Value, from_value};
use reqwest;
use std::io::{self, Write};
use serde::Deserialize;
use rand::seq::SliceRandom;
use html_escape::decode_html_entities;

#[derive(Debug, Deserialize)]
struct QuizQuestion {
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

#[derive(Debug)]
struct Question {
    question_text: String,
    correct_answer: String,
    options: Vec<String>,
}

fn process_question(raw_question: QuizQuestion) -> Question {
    let question_text = decode_html_entities(&raw_question.question).into_owned();
    let correct_answer = decode_html_entities(&raw_question.correct_answer).into_owned();

    let mut options: Vec<String> = raw_question
        .incorrect_answers
        .into_iter()
        .map(|s| decode_html_entities(&s).into_owned())
        .collect();

    options.push(correct_answer.clone());
    options.shuffle(&mut rand::thread_rng());

    Question {
        question_text,
        correct_answer,
        options,
    }
}

async fn fetch_quiz_data() -> Result<Vec<QuizQuestion>, String> {
    let url = "https://opentdb.com/api.php?amount=10&difficulty=medium&type=multiple";

    let response = match reqwest::get(url).await {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to connect to API: {}", e)),
    };

    let json_data: Value = match response.json().await {
        Ok(j) => j,
        Err(e) => return Err(format!("Failed to parse API response to JSON: {}", e)),
    };

    let results_array = match json_data.get("response_code") {
        Some(code) if code == &Value::from(0) => {
            json_data.get("results").cloned().unwrap_or(Value::Array(vec![]))
        }
        _ => {
            let code = json_data.get("response_code").unwrap_or(&Value::Null);
            return Err(format!("API returned an error code: {}", code));
        }
    };

    match from_value::<Vec<QuizQuestion>>(results_array) {
        Ok(questions) => Ok(questions),
        Err(e) => Err(format!("Failed to deserialize quiz questions: {}", e)),
    }
}

pub async fn run_trivia_quiz_cli() {
    let raw_questions = match fetch_quiz_data().await {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Error starting quiz: {}", e);
            return;
        }
    };

    let questions: Vec<Question> = raw_questions.into_iter().map(process_question).collect();

    if questions.is_empty() {
        eprintln!("Error: No quiz questions were retrieved or processed.");
        return;
    }

    let mut score = 0;
    let total_questions = questions.len();

    for (i, question) in questions.iter().enumerate() {
        println!("\n--- Question {}/{} ---", i + 1, total_questions);
        println!("{}", question.question_text);

        for (j, option) in question.options.iter().enumerate() {
            println!("{}. {}", j + 1, option);
        }

        let user_choice_index: usize = loop {
            print!("Enter your choice number (1-{}): ", question.options.len());
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(n) if n >= 1 && n <= question.options.len() => break n - 1,
                _ => println!("Invalid input. Please enter a number between 1 and {}.", question.options.len()),
            }
        };

        let user_answer = &question.options[user_choice_index];

        if user_answer == &question.correct_answer {
            println!("Correct! ✅");
            score += 1;
        } else {
            println!("Incorrect. The correct answer was: {}", question.correct_answer);
        }
    }

    println!("\n--- Quiz Finished 🎉 ---");
    println!("Your final score: {}/{}", score, total_questions);
}