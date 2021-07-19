# What is this?
smart_job_application is a silly and quickly written mini-project that helps you write targeted job applications based on a generic template.

It was written as an exercise to learn some of Rust's language features.

# How does it work?
Its developed to work with two kinds of "tokens".

    1. #token_name#
    2. #token_name[some string that replaces the token]#

Tokens start and end with a hashtag (#), and can optionally contain a default replacement string.

The application prompts the user to choose a file to process, after which it iterates over all such tokens in the file while asking the user for input for each token.

For the first type of token, the user is asked to write a string to replace the token with.

For the second type of token, the user is asked whether to replace the token with the contained default replacement string (Y/N). If the user inputs no, the token is replaced with an empty string.

Finally the user is prompted to select a location and name for the newly processed file.

# Example
Before processing:
```
Dear #name_of_potential_employer#.

I am writing to apply for the position as #job_position# as advertised on #where_did_you_find_the_job_advertisement#. As requested,
i enclose a complete job application, my certification, my resume and references.

#if_job_is_related_to_backend[The role is very appealing to me as i have a lot of experience working in backend systems.]#
#if_job_is_related_to_frontend[The role is very appealing to me as i am very passionate about frontend development.]#

#if_my_experience_as_electrician_is_relevant[I started my work-life as an electrician, and thus know a lot about electrical stuff as you mention is a requirement in the
advertised position]#

#if_my_experience_as_a_math_teacher_is_relevant[I taught mathemathics at the university of London where i got a lot of experience teaching concepts, which is why i think i fit the 
job description perfectly.]#

I am passionate about programming, very patient in my work and others, and always seek to bring a good mood to the workplace.
With a bachelors degree in computer engineering, and plenty of work experience, i believe i can be a valuable asset for your company #company_name#.

You can reach me any time via email at #email_address# or by cell phone at #cell_phone_number#.
```

During processing the user inputs an answer for each token:
```
What should token name_of_potential_employer be replaced with?
>Mr. Johnson

What should token job_position be replaced with?
>Math-Savvy Backend Developer

What should token where_did_you_find_the_job_advertisement be replaced with?
>Linked.in

Should this token's: 
	if_job_is_related_to_backend
 default replacement text: 
	"The role is very appealing to me as i have a lot of experience working in backend systems."
	be inserted?
Y/N?
>y

Should this token's: 
	if_job_is_related_to_frontend
 default replacement text: 
	"The role is very appealing to me as i am very passionate about frontend development."
	be inserted?
Y/N?
>n

Should this token's: 
	if_my_experience_as_electrician_is_relevant
 default replacement text: 
	"I started my work-life as an electrician, and thus know a lot about electrical stuff as you mention is a requirement in the
advertised position"
	be inserted?
Y/N?
>n

Should this token's: 
	if_my_experience_as_a_math_teacher_is_relevant
 default replacement text: 
	"I taught mathemathics at the university of London where i got a lot of experience teaching concepts, which is why i think i fit the 
job description perfectly."
	be inserted?
Y/N?
>y

What should token company_name be replaced with?
>Johnsons Concultants

What should token email_address be replaced with?
>some@email.com

What should token cell_phone_number be replaced with?
>012 34 567
```

The user then selects a location and filename for the processed job application, which contents is:

```
Dear Mr. Johnson.

I am writing to apply for the position as Math-Savvy Backend Developer as advertised on Linked.in. As requested,
i enclose a complete job application, my certification, my resume and references.

The role is very appealing to me as i have a lot of experience working in backend systems.

I taught mathemathics at the university of London where i got a lot of experience teaching concepts, which is why i think i fit the 
job description perfectly.

I am passionate about programming, very patient in my work and others, and always seek to bring a good mood to the workplace.
With a bachelors degree in computer engineering, and plenty of work experience, i believe i can be a valuable asset for your company Johnsons Concultants.

You can reach me any time via email at some@email.com or by cell phone at 012 34 567.
```

# Dependencies
Dependencies as listed in Cargo.toml is
    
    .. rfd = "0.4.0"
    .. regex = "1.5"

[rfd](https://crates.io/crates/rfd) or Rusty File Dialog is used to prompt the user to select which file to read and where to save the processed file.

Rusty File Dialog uses the systems native API for file dialogs. The development libraries for these systems are required to compile.