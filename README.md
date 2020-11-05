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
    - Communicate with the Database.
    - Password is hashed with `PBKDF2` hash with secret key.

- Encode and decode Json Web Token (JWT)
    - JWT token is then stored in Cookie with live-time of 3 days.
    - Cookie is then set as `HttpOnly` and `SameSite`, this prevent `XSS` and `CSRF` attack.
    - Cookie then is encrypted by `Actix Identity` itself with secret key.

- Refresh Token
    - Decode Cookie which stored JWT.
    - Checked live time, drop if expired.
    - Signed new JWT and put as new Cookie.

## Chat Module
Responsible for everything that related to chat.

Chat Module is implement using Actor models with mainly with actix StreamHandler.

### Basic concept.
Each Actor has shared state `ChatContext` which encapsulate `room` and `connection` as Nested HashMap.

- `room` is use to determine message between sender and receiver. 
    - Created as '<sender_name>-<receiver_name>' where name is sorted to ensure that both client has the same room.

- `connection` is use to determine client's session, for instance, using `room` as determiner couldn't work because if client with same user using at the same time, the previous client `Addr` will overwrited and cannot listen to `ChatRoom` any more.
For each connection, the `connection` is use to encapsulate `Addr` if more than 1 device on each end is listen.
`connection` is register to an responsible Actor for total connection in a room.
    - When client is disconnect, the session is ended and removed.
    - `u128` is easier and faster to find and compare than using `Addr` struct provided by `Actix`
    - Created as `Time since Unix Epoch`.

Room and Connection is structured as:
```rust
// ? Chat Context Struct
HashMap<                // Chat Context
    String,             // Room
    HashMap<
        u128,           // Connection
        Addr<ChatRoom>  // Chat Address
    >
>
```

Models:
- `ChatRoom`
    - Main Actor, responsible everything for chat.
    - Handle request stream from client.
    - Send Message to others receivers in `ChatRoom`.
    - Send Message to `ChatMessage`.

    - Structure:
        - clients
            - type: `Arc<Mutex<ChatContext>>`
            - Shared Mutatble state.
            - Encapsulate pointer to Chat Address across Actors and threads.

        - room
            - type: `String`
            - Use to determined sent message between each Actor.

        - connection
            - type: `u128`
            - Use to determine client session in case of multiple device client.

        - sender
            - type: `String`
            - name of sender or client who establish connection.

        - database_connection
            - `sqlx::PgPool`
            - Pull from Database connection pull.

- `ChatMessage`
    - Encapsulate all information of chat message.
    - Receive message from `ChatRoom`.
    - Communicate with Database.

    - Structure:
        - type: 
            - type: `String`
            - Determine message's type.

        - sender
            - type: `String`
            - Determine message owner.

        - data
            - type: `String`
            - Message data.

- `ChatContext`
    - Shared mutable context.
    - Store `room` and `connection` for both in Actors and web threads.

```rust
// ? Chat Context Struct
HashMap<                // Chat Context
    String,             // Room
    HashMap<
        u128,           // Connection
        Addr<ChatRoom>  // Chat Address
    >
>
```

- `Chat`
    - Not an Actor.
    - Pull chat history from server as pagination.

### Database
Message is store in `PostgreSQL`<sup>1</sup>
- All message is stored in `message` table<sup>2</sup>

- Database structure is structured as in `migrations` folder.

### Note
1. Chat Message is not suitable for storing in PostgreSQL (in my opinion) and will be moved to MongoDB or other NoSQL.

2. Due to limitation of SQLx, creating dynamic table without defining one in `migrations` is forbid. Also creating nested table in PostgreSQL is not support.

Due to this limitation of both `(1)` and `(2)`, when querying for chat message take `O(log n)` with index which is very slow in scale of real-world chat app.

Since querying the `O(log n)` from the whole table when requested is extremly slow, I want to use NoSQL instead.

Seperated chat message as document and access using document key is faster, since we already have key for each connection.

Querying from specific structure is exactly faster than querying from whole table.
  
Now please let me rest and watch Fubuki livestream.
![fbk]
(https://static.wikia.nocookie.net/virtualyoutuber/images/2/2c/Shirakami_Fubuki_-_Destroyer_Azur_Lane_Chibi.gif/revision/latest/scale-to-width-down/180?cb=20191126133045)