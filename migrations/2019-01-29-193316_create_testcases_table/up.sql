-- Your SQL goes here
CREATE TABLE Testcase(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL, 
    assg CHAR(5) NOT NULL,
    ques CHAR(5) NOT NULL,
    content TEXT NOT NULL
);