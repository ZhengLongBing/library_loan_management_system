-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE accounts (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    authority VARCHAR(20) NOT NULL DEFAULT 'user',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE books (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(100),
    isbn VARCHAR(20) NOT NULL UNIQUE,
    category VARCHAR(50)
);

CREATE TABLE borrow_records (
    id INT AUTO_INCREMENT PRIMARY KEY ,
    account_id INT NOT NULL,
    book_id INT NOT NULL,
    borrow_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    as_of_date TIMESTAMP NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(id),
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

CREATE TABLE borrow_and_return_records (
    id INT AUTO_INCREMENT PRIMARY KEY ,
    account_id INT NOT NULL,
    book_id INT NOT NULL,
    borrow_date TIMESTAMP NOT NULL,
    return_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (book_id) REFERENCES books(id),
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

#同一isbn号书的总数
CREATE VIEW book_counts AS
SELECT
    title,
    isbn,
    COUNT(*) AS num_books
FROM
    books
GROUP BY
    isbn;


#被借阅同一isbn号书的总数
CREATE VIEW borrow_counts AS
SELECT
    b.title,
    b.isbn,
    COUNT(br.id) AS borrow_count
FROM
    books b
        JOIN borrow_records br ON b.id = br.book_id
GROUP BY
    b.isbn;

CREATE VIEW books_display AS
SELECT
    b.title,
    b.isbn,
    b.author,
    b.category,
    bc.num_books as number,
    (bc.num_books - IFNULL(br.borrow_count, 0)) AS remaining
FROM
    books b
        LEFT JOIN
    (SELECT isbn, COUNT(*) AS num_books FROM books GROUP BY isbn) AS bc ON b.isbn = bc.isbn
        LEFT JOIN
    (SELECT b.isbn, COUNT(br.id) AS borrow_count FROM books b JOIN borrow_records br ON b.id = br.book_id GROUP BY b.isbn) AS br ON b.isbn = br.isbn;

#用户借书的情况
CREATE VIEW user_borrow_details AS
SELECT
    a.username,
    a.id AS account_id,
    b.title,
    b.isbn,
    br.borrow_date,
    br.as_of_date
FROM
    accounts a
        JOIN borrow_records br ON a.id = br.account_id
        JOIN books b ON br.book_id = b.id;


CREATE VIEW remaining_books AS
SELECT
    book_counts.title,
    book_counts.isbn,
    (book_counts.num_books - IFNULL(borrow_counts.borrow_count, 0)) AS remaining_count
FROM
    book_counts
        LEFT JOIN borrow_counts ON book_counts.isbn = borrow_counts.isbn;


#用户还书的情况
CREATE VIEW user_return_details AS
SELECT
    a.username,
    a.id AS account_id,
    b.title,
    b.isbn,
    br.borrow_date,
    br.return_date
FROM
    accounts a
        JOIN borrow_and_return_records br ON a.id = br.account_id
        JOIN books b ON br.book_id = b.id;

-- 插入示例数据到 accounts 表
INSERT INTO accounts (username, password, email) VALUES
    ('user1', 'password1', 'user1@example.com'),
    ('user2', 'password2', 'user2@example.com'),
    ('user3', 'password3', 'user3@example.com');

-- 插入示例数据到 books 表
INSERT INTO books (title, author, isbn, category) VALUES
    ('Book1', 'Author1', 'ISBN1', 'Category1'),
    ('Book2', 'Author2', 'ISBN2', 'Category2'),
    ('Book3', 'Author3', 'ISBN3', 'Category3');

-- 插入示例数据到 borrow_records 表
INSERT INTO borrow_records (book_id, account_id, borrow_date, as_of_date) VALUES
    (1, 1, '2022-01-01', '2022-01-15'),
    (2, 1, '2022-02-01', '2022-02-15'),
    (3, 1, '2022-03-01', '2022-03-15');

-- 插入示例数据到 borrow_and_return_records 表
INSERT INTO borrow_and_return_records (book_id, account_id, borrow_date, return_date) VALUES
    (1, 1, '2022-01-01', '2022-01-20'),
    (2, 1, '2022-02-01', '2022-02-20'),
    (3, 1, '2022-03-01', '2022-03-20');