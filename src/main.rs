use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;
use std::env::{set_current_dir, current_dir};
use std::fs::{remove_file, remove_dir_all};
use evalexpr::*;


//Implementing a Shell in Rust
//Goal: I wanted to see how Rust interfaces with the kernel. 
//What better way to do so then by writing a shell which implements system calls

//New functionality:
//Multiple built-in commands
//Echo command
//Expression Evaluator
//Basic authentication - little bit of reverse engineering

//Custom implementations for 
//directory traversal
//directory removal
//directory display

fn main() {
    println!("Welcome to tsh2.0.");
    println!("Type help to see more information about built-in commands.");
    loop {
        print!("tsh2.0> ");
        stdout().flush().unwrap();
        let mut cmd = String::new();
        //Read the user input and pass it to our cmd_handler
        stdin().read_line(&mut cmd).unwrap();
        //Check if the code returned by cmd_handler is 0
        //in which case we quit the program. 
        if cmd_handler(cmd) == 0 {
            break;
        }
    } 
}

fn cmd_handler(cmd: String) -> i32 {
    //Obtain an iterator for our command split by whitespace 
    //This will allow us to pass arguments
    let mut parse = cmd.trim().split_whitespace();
    //We want to retrieve the arguments as well as the original command
    let main_cmd = parse.next().unwrap();

    match main_cmd {
        //Custom implementation of rm
        "rm" => {
            let flag = parse.next().unwrap();
            //Check if the -r flag is provided to remove a directory
            match flag {
                //Remove directory with -r flag
                "-r" => {
                    let path = parse.next().unwrap();
                    let success = remove_dir_all(path);
                    match success {
                        Ok(_success) => {
                            println!("Successfully removed directory at {}", path);
                        }
                        Err(_error) => {
                            println!("Unable to identify the provided directory name.");
                        }
                    }
                },
                //If no flag is provided, we remove a file
                _ => {
                    let success = remove_file(flag);
                    match success {
                        Ok(_success) => {
                            println!("Successfully removed file at {}", flag);
                        }
                        Err(_error) => {
                            println!("Unable to identify the provided file name.");
                        }
                    }
                }
            }
        },
        "pwd" => {
            let working_dir = current_dir().unwrap();
            println!("The current working directory is {}", working_dir.display());
        },
        "eval" => {
            //Obtain the complete expression to be evaluated by concatenating
            //the remaining operators/operands within the iterator
            let mut exp = String::new();
            for op in parse {
                exp = format!("{} {}", exp, op);
            }
            let answer = eval(&exp).unwrap();
            println!("Your expression evaluates to: {}", answer);
        },
        "377" => {
            let user_secret = parse.next().unwrap();
            let key = 3;
            let encrypted_key = [76, 80, 106, 112, 64, 108, 108, 111];
            if user_secret.len() != 8 {
                println!("Your input is of the wrong length.");
                return 1;
            }
            //Iterate through the user input and check that the corresponding characters are correct
            //xor the user input with the key
            let mut counter = 0;
            while counter < 8 {
                //Case when a character doesn't match
                if (user_secret.chars().nth(counter).unwrap() as u32 ^ key) != encrypted_key[counter] {
                    println!("Wrong password!");
                    return 1;
                }
                counter += 1;
            }
            //Encrypted prize string
            let prize = "kwwsp9,,vnbpp.`p.044-djwkva-jl,";
            counter = 0;
            println!("Your prize: ");
            //Decrypt the prize
            while counter < prize.len() {
                print!("{}", char::from_u32(prize.chars().nth(counter).unwrap() as u32 ^ key).unwrap());
                counter += 1;
            }
            println!("\nEnjoy!");
        }
        "cd" => {
            //Retrieve the path and set the current directory to the pathname
            let path = parse.next().unwrap();
            let success = set_current_dir(&path);
            match success {
                Ok(_success) => {
                    println!("Successfully changed to directory {}", path);
                }
                Err(_error) => {
                    println!("Unable to identify the provided directory name.");
                }
            }
        },
        "echo" => {
            //Iterate through the rest of the words in the echo string and print them out
            for words in parse {
                print!("{} ", words);
            }
            println!();
        },
        "quit" => {
            println!("Have a nice day!");
            return 0;
        },
        "help" => {
            println!("\nIn general, here is how you use the tsh2.0 command line: ");
            println!("<Main_Command> <Arguments>");
            println!("\nHere is a list of the built-in commands and their definitions: \n");
            println!("quit: Exit the shell.");
            println!("echo <argument>: Output the argument string.");
            println!("help: Display this help table.");
            println!("eval <expression>: Evaluate arithmetic expression.");
            println!("rm <file>: Remove the given file.");
            println!("rm -r <dir>: Remove the directory dir.");
            println!("pwd: Print the working directory.");
            println!("377 <password>: Secret Command... Enter the correct password to access.");
            println!("cd <directory>: Change the working directory.\n");
        },
        //The default case
        _ => {
            //The parse iterator is at the position of the args
            let child_proc = Command::new(main_cmd).args(parse).spawn();

            //We check that the child process doesn't panic
            //If there is an error, we catch it, letting the user know that
            //they have entered an invalid command
            match child_proc {
                Ok(mut child_proc) => {
                    child_proc.wait().unwrap();
                }
                Err(_error) => {
                    println!("You have entered an invalid tsh2.0 command. Please try again or enter help for a list of built-in commands.")
                }
            }
        }
    }
    //We return 1 for all cases except for quitting
    return 1;
}