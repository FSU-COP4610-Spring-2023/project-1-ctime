Project Members -
- Nicholas Ratzlaff
- Matthew Christoffel
- Anthony Knapik

Division of Labor -
Parsing - Matthew Christoffel, Nick Ratzlaff, Anthony Knapik
Environment Variable - Matthew Christoffel, Nick Ratzlaff, Anthony Knapik
Prompt - Matthew Christoffel, Nick Ratzlaff, Anthony Knapik
Tilde Expansion - Nick Ratzlaff, Anthony Knapik
$Path Search - Anthony Knapik, Matthew Christofel
External Command Execution - Matthew Christoffel,Nick Ratzlaff
I/O Redirection - Nick Ratzlaff, Anthony Knapik
Piping - Anthony Knapik, Matthew Christoffel
Background Processing - Matthew Christoffel, Nick Ratzlaff
Built-in Functions - Matthew Christoffel, Nick Ratzlaff, Anthony Knapik
Extra Credit - Matthew Christoffel, Nick Ratzlaff, Anthony Knapik

Updated Work Division -
Parsing - Nicholas Ratzlaff, Anthony Knapik
Environment Variables - Nicholas Ratzlaff
Prompt - Nicholas Ratzlaff
Tilde Expansion - Nicholas Ratzlaff
$Path Search - Anthony Knapik
External Command Execution - Anthony Knapik
I/O Redirection - Nicholas Ratzlaff
Piping - Nicholas Ratzlaff
Background Processing - Matthew Christoffel
Built-in Functions - exit, jobs - Matthew Christoffel - cd, echo - Anthony Knapik
Extra Credit - 

File Listing -
- backgroundExecute.rs
-- file with functions to help run background execution of commands within the shell.
- commandSplit.rs
-- contains function to take in user command and split them into seperate Vectors
   for execution on piping commands.
- direc.rs
-- includes helper function to find current working directory and a function to 
   change directory when the cd command is called. 
- envVar.rs
-- contains function to replace environment Variables with their string equivalents 
   in main.rs.
- execution.rs
-- file containng helper functions to convert String Vectors into CString Vectors
   and function to fill an argument vector if needed. Main functionality is the
   execute function which calls execv and fork processes when needed for piping
   and io redirection. 
- IORedirection.rs
-- file containing functions to write to files, and read files for io redirection.
   called in main.rs when io redirection is detected in a user command.
- prompt.rs
-- contains function to print the command line prompt including the curent user, 
   machine, and working directory. 
- psearch.rs
-- file with function to path search a command call and detects if it is valid 
   or is located in a directroy path on the machine.
- tilde.rs
-- file that implements tilde expansion to the home directory of the current machine
- echoFunc.rs
-- after main.rs checks for echo command, function here checks for inputs follwing 
   echo to be printed out to the user
- main.rs
-- calls functions from all of the afformentioned files to run the shell. Checks 
   for key characters to determine what commands are being run an executing.
- makefile (results in executable called shell)
-- makefile to run the project.

Makefile Specifications
- Running the makefile involves typing "make" into the terminal to create the "shell"
  executable and then tping that into the command line. The project should run from
  there. The project is in Rust, so you may have to set the path variable with
  path = ($path /home/majors/your_linprog_username/.cargo/bin) if on linprog. From
  there, simply typing "shell" in the command prompt should start the project. If
  in a different machine, typing "./shell" after running typing "make" may be 
  required.  

Bugs -
Matthew -  
Background Processing - For some reason the wait check in main is checking on a process more than once,
causing an ECHILD error. To my knowledge thats the only reason that error will appear. 
Very rare that this happens. Does not work correctly with piping. The initial command fails to exit.
Output for piping is nearly correct, however outputs incorrectly if multiple jobs are active at the 
same time as a background piped process.
Exit function will take any input, not just commands. Assuming appropriate input, works fine.
Anthony -
cd Command - At points, the cd command will tell the user that a certain destination directory does
not exist when it in fact does. It will still direct the user to said directory. unsure of why this 
happens.

Extra Credit -
Can run another shell from within the running program [1]
