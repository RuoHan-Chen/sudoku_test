![The Academic Honesty Policy posted on Brightspace is applicable to all work you do in CS 3270/5270](https://my.vanderbilt.edu/tairas/files/2024/08/academic-honor-policy-fall-2024-2.png)

# CS 3270: Programming Languages
## Project 4

## Goal

Gain experience in quickly learning a new programming language.

## GitHub notes

Please see [GitHub notes](github-notes.md) for more information on cloning, committing, and pushing repositories.

## Due dates
Due date|Description
----:|-----------
Friday (11/15)|Complete [Google form](https://bit.ly/vu-cs3270-fall24) to specify choice of programming language and if working individually or as a team – **the sooner you complete the form and start on the project, the more time you have to work on it**.
Thursday (12/05)|Final due date – **no late submissions accepted**.


## Assignment

Create a program that solves Sudoku in a language that is new to you.

At this point in the class, you have programmed a Sudoku solver in C++, Racket, and Prolog. You are now to write a Sudoku solver in a language that is new to you. The languages you may choose from are:

* Elixir
* Erlang
* F#
* Go
* Haskell
* Rust
* Scala\*\*

\*\* Using Java packages **prohibited**

In all cases, you can only use constructs based on the default installation of the language framework (i.e., do not use any special libraries that must be separately installed from the default installation).

## Language selection and working individually or as a team

You must complete the [Google form](https://bit.ly/vu-cs3270-fall24) by **Friday (11/15)** with information on what language you choose to do  this project in and whether you will be working individually or as a team. Failure to complete the form by the deadline will incur a late penalty.

A team can consist of up to three students. Members of a team can be from different sections. Only one member of the team needs to complete the form (i.e., only one submission per team).  

After you complete the form, a repository will be generated for you. Note that the generation of the repository is not an automatic process. When the repository is ready, you will receive an email notification.

**Regardless if you are working individually or as a team, you must complete the form above by the deadline.**

## Program features

Your program should work just as the programs in the previous projects:

* **Read** puzzle from a file.
* **Print** the initial puzzle and solved puzzle (and a no solution message if unsolvable).
* Use the language's testing framework to **run tests**.
* Configure GitHub Actions to run the tests on **online**.

### Reading from a file

Your program should **prompt** the user for a file name and **load** the puzzle from that file. Checking that a file with the given file name actually exists is not required. Include the three puzzles from the previous projects (i.e., `sudoku-test1.txt`, `sudoku-test2.txt`, and `sudoku-impossible.txt`).

Note that the previous projects imported a puzzle in two different ways. In the Project 1 (C++/Java), the contents of the text file were read one token at a time and stored in some data structure. In Projects 2 (Racket) and 3 (Prolog), the text file contained the actual code for the data structure in textual or serialized form (i.e., a list of lists) and built-in functions were utilized to import the textual version of the list of lists into the program. You may implement the reading of the text files either way.

### Printing the puzzle

Just as in previous projects, the **initial** puzzle and the **solved** puzzle should be **printed** with borders just as in the previous projects. If the puzzle is not solvable, the message **"This puzzle has no solution"** message should be printed. Timing information is not necessary.

### Testing the solver

All the languages listed above have a **testing framework** that can be used to confirm that the solver worked correctly. If you look at the tests from the previous projects, the general process is to load the unsolved puzzle and the solved puzzle from the text files. Then run the solver and compare (or assert) that the result generated by the solver is equal to the solved puzzle.

The testing must be done within the testing framework of the language, which allows for the test cases to be invoked via a special command separate from the running of the program.

When testing your solved puzzle, you must compare the solved puzzle with the solution is some way. You can't just assert that some function that is tasked to solve the puzzle returns `true` and that signifies that the puzzle was solved correctly.

You don't need to perform testing of the printout of the puzzle (such as was done in Project 1). Also, you don't need to perform testing of whether the functions that check if a value can be placed in a position on the puzzle is correct (such as was done in Project 2).

### Testing on GitHub.com

Your Project 4 GitHub repository should be configured so that the test cases can also be run on GitHub.com. This requires configuring a **GitHub Actions** workflow. [A description of GitHub Actions workflow setup in the previous projects](github-actions.md).

If you setup the workflow correctly, once you push to GitHub.com and all the tests pass, a green checkmark :heavy_check_mark: will be displayed beside the commit information.

## Additional notes on implementation

### All in one file

Some languages support including test cases in the same file as the code the test cases are checking. You may place all you code in one file as long as it is possible to run your program and run the test cases separately. The code in the file should be well-structured. Note that it should be also possible to separate the test case code and the main program in separate files as was done in the previous projects.

### Main driver

You are expected to implement a driver that runs your program. It should prompt for a text file name and then then try to solve the puzzle loaded from the text file, providing an output of with a result. Hence, we expect to be able to run your program similar to the previous projects. In addition, we expect to be able to run the test cases you've generated.

## Project submission

For submission, the individual or team repository for Project 4 on GitHub should contain the following files:

* **All source code files** – Make sure to add block comments to the top of all source files and include an academic honesty statement. Placing the files in a `src` folder is optional.

* **Puzzle text files** – Include the two solvable puzzles and one unsolvable puzzle from past projects. Placing the files in a `txt` folder is optional.

* Update the **project4.md** file and include all required information outlined in the file, which include:
  * **Names** of individual or team members.
  * **Instructions** on how to access an appropriate **compiler/interpreter** for your solver. If you found a website with appropriate installation instructions, you can just include the website's URL. You don't need to provide installation instructions for all operating systems. You can just provide instructions for the operating system that you used for the project.
  * **Instructions** on **how to use** your program. As appropriate, include instructions on compiling, linking, and running your solver. Also, how to run the test cases.
  * **Answer** the questions listed in the file.

### Markdown markup language

The **project4.md** file is a *Markdown* file, which allows you to use simple syntax to format the text in the file. You can open and edit **project4.md** in your favorite text editor. You are not required to *beautify* your text in this file, but at the very least, the text should not be messy and/or disorganized. [More information on the Markdown syntax](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax).

## Participation form for team projects

**Important:** For those who are on a team, each team member must complete the [Google form](https://docs.google.com/forms/d/e/1FAIpQLSdGC0NDAyeWwAhI8Gtuo2SGW2nnfZeZo2Umb2GyID3nBg8CSw/viewform?usp=sf_link) by the project due date on **Thursday (12/05)** discussing how much each team member contributed to the project. Those working individually do not need to complete the form.

Your submission to the form stating your personal opinion will not be shared with other team members. You should state a percentage of contribution by each team member (should total 100%) and you should provide additional commentary describing each team member's contribution to the overall effort. Optionally, you can provide information on any special circumstances you want me to be aware of. Note that we will not look at how active each team member is on the GitHub repository (e.g., committing code). Team participation will be based on the form above.

Even if different team members work on different parts of the project, each team member should have a general understanding of all the aspects of the project.

## Style guidelines

* **Structure:** You should not place the entire solution in one function/method. Make sure you modularize your code (i.e., place code for sub-tasks in different functions/methods).

* **Line lengths:** No lines should exceed 100 columns.

* **Indentation:** Use proper and consistent indentation.

* **Whitespace:** Functions/methods should be separated by at least one empty line.

* **Comments:** Use Javadoc-style comments to document all functions/methods. For your reference, below is an example of Javadoc-style comments for a method.

   ```
  /**
   * Returns an Image object that can then be painted on the screen.
   *
   * @param  url  An absolute URL giving the base location of the image.
   * @param  name The location of the image, relative to the url argument.
   * @return      The image at the specified URL.
   */
  ```
  You do not need to include Javadoc-style comments on functions/methods that are in any test files. That is, only provide Javadoc-style comments on functions/methods containing implementation code. However, header comments containing your information and the Honor Code statement must be placed on both implementation and test files.

## Grading

* Falsifying a green checkmark associated to a GitHub Actions workflow will receive a severe points deduction. That is, if you repository displays a green checkmark, but the workflow did not include appropriately testing the solver, points will be deducted.

* Points will be deducted if the **project4.md** file is incomplete.

* Points may be deducted if the **project4.md** file is messy and/or disorganized.

* If you are encountering difficulties and need to change the language that you initially chose, you must contact the instructor about the change. Note that your score will be reduced by around 5%.

* Not completing the language selection and working individually or as a team form by the deadline will be deducted around 5%.

* A team member who does not complete the team participation form by the deadline will be deducted around 5% from that team member's score.

* For teams, each team member will receive the same grade unless a team member did not contribute much to the final project (in which case their score may be reduced by around 10%).

## Graduate students

You are required to select a second language and create a Sudoku solver with that language, too. For the second language, you **are not** allowed to join a team. That is, for one of your languages you can join a team, but for the other you must work individually. Of course, you may work on both individually, too. Please email your second language selection directly to the instructor. The due date for this second language is the same as the due date of the first language, which is on **Thursday (12/05)**.

## Academic honesty

For this project, you are allowed to use the Internet to assist you in completing the project. You may get assistance from a Large Language Model (LLM). For example, you can ask ChatGPT to translate your solution in the previous project to the language you are working on in this project.

However, you must use these tools by yourself (if you are working individually) or within your team (if you are working as a team). That is, **you cannot ask somebody outside of your team** to assist you with using these tools. And as per usual, you cannot assist any student in future classes. Also, you are prohibited from **searching for or asking an LLM to find** ***"the answer to Project 4 in CS 3270"*** or something along that line.

**IMPORTANT: In this assignment, you are allowed to work with other students (in your team) and get assistance from a large language model or access information on the internet. As you know from previous assignments in CS 3270, this is not always the case. Hence, IT IS YOUR RESPONSIBILITY TO UNDERSTAND WHAT IS ALLOWED AND WHAT IS NOT ALLOWED WHEN COMPLETING ASSESSMENTS IN ANY CLASS THAT YOU ENROLL IN.**

## Final notes

* Ensure that your name, VUnetID, email address, and the honor code statement appear in the header comments of files containing code (both implementation and test files).

* You must push your repository to GitHub.com before the deadline as no late submissions will be accepted.

* CS 3270 TAs are not expected to know the languages in Project 4, so they are not obligated to provide assistance.
