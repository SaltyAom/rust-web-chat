# Chat backend
##### Work in Progress
Implement chat web backend in Rust because Rust is fast and I can.

## Approach
Mainly using 2 approach.
- Model View Controller Structure (MVC)
- Functional Programming

## MVC
Using MVC structure, to encapsulate logic and express the code as natural language.

### Structure
Code structure is expressed as the following:
- main.rs
    - Main module entry
- (Module Name)
    - Module to pull in `main.rs`

    - mod
        - Entry of the module
    - controller (View)
        - Main control of the module
        - Responsible to express the logic
        - Control http endpoint
    - model
        - Model of the structure
    - services
        - Encapsulate business logic as natural language
    - constant
        - Encapsulate constant as natural language
- libs
    - Shared Services

### Functional Programming
Basically, I use function with name to do thing and guess what it do by name if I forget it.
Don't look at me like that, I'm too lazy to explain it ya know.

## User Module
Responsible for everything that related to authentication including:
- Sign in / Sign up / Sign out
- Encode and decode Json Web Token (JWT)
- Refresh Token

## Chat Module
##### Work in Progress
Responsible for everything that related to chat.

Now please let me rest and watch Hololive.
![fbk](https://static.wikia.nocookie.net/virtualyoutuber/images/2/2c/Shirakami_Fubuki_-_Destroyer_Azur_Lane_Chibi.gif/revision/latest/scale-to-width-down/180?cb=20191126133045)