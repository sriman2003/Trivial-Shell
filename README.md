# TSH
### Presentation Link
Here is a link: https://drive.google.com/file/d/1yPnnKoZpzhcvjQ-Of8OE45mPSr4U4kCC/view
### Objective
Implement a shell in Rust.
### Implementation
I wanted to see how Rust interfaces with the kernel. And what better way to do so then by writing a shell which implements system calls.

Rust has several well-defined modules which make it easy to interact with process management. We can use pre-defined methods to create and wait for new child processes. Thus, my main focus in this project turned to producing the built-in functionality. 

In addition to the standard commands, I designed my own built in commands. I also reproduced some well-known bash commands.

New functionality and custom implementations:  
quit: Exit the shell.  
echo (argument): Output the argument string.  
help: Display this help table.   
eval (expression): Evaluate arithmetic expression.  
rm (file): Remove the given file.  
rm -r (directory): Remove the directory dir.  
pwd: Print the working directory.  
377 (password): Secret Command... Enter the correct password to access.   
cd (directory): Change the working directory.  

As listed above, tsh2.0 supports directory traversal, removal, and display. Additionally, the eval command evaluates arithmetic expressions, echo prints whatever argument is given, and quit exits the shell program.

The 377 built-in command is a pretty neat password reversing challenge. The password encryption is an xor with a key of 3. I thought it would be a good idea to implement this to help myself understand how to iterate over strings and convert among different data types.

### Documentation
The comments throughout the main.rs file describe my steps to implementing each part of tsh2.0. The video/presentation/demonstration should further detail the functionality of tsh2.0 as well as how I implemented it.

### Running TSH2.0
After cloning the repository onto your local machine, navigate to the main directory. Use the command **cargo run** in the command line. This should retrive the required dependencies and run the shell. That's it!
