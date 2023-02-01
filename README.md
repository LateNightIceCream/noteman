# noteman
notes manager c:

really just a practice project

## Usage

`noteman -d path/to/notes/root -t path/to/template/dir -i path/to/startup/script`


## Functionality
noteman's functionality is simply to keep a predefined directory structure of notes that looks like this:

- root
  - subject 1
    - topic 1.1
      - ...
    - topic 2.2
      - ...

  - subject 2
    - topic 2.1
      - ...
    - topic 2.2
      - ...
      

and then it lets you choose from them or create new ones using rofi as a GUI.
What is behind the `...` is determined by an additional template directory.

The program flow is as follows:

1. call noteman with notes directory, template directory and optional startup script
2. select from exisiting subject or create new one
3. select from exisiting topic inside subject or create new one
4. copy template files to selected topic directory 
  - they will be renamed according to the topic name
  - [topicname] in the filename will be replaced with the name of the topic with underscores
5. run the startup script inside that directory

### Startup Script
The startup script is called by noteman after selecting the topic. It can e.g. spawn an editor, terminal and PDF viewer.

It will be passed these arguments:

- [1]: topic directory path


## Motivation
I usually take lecture notes and so on using markdown files and mdbook. To access them easily I made this tool.
Also, I need to write lab protocols using LaTeX with a similar approach but different compilation tools.

