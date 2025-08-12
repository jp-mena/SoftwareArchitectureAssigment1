/*
===========================================
    BOOK REVIEWS DATABASE SCHEMA
===========================================

┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│     AUTHORS     │       │      BOOKS      │       │     REVIEWS     │       │      SALES      │
├─────────────────┤       ├─────────────────┤       ├─────────────────┤       ├─────────────────┤
│ id (PK)         │──────>│ author_id (FK)  │<──────│ book_id (FK)    │       │ book_id (FK)    │<─┐
│ name            │       │ id (PK)         │<──────│ id (PK)         │       │ id (PK)         │  │
│ date_of_birth   │       │ name            │       │ review          │       │ year            │  │
│ country_of_origin│      │ summary         │       │ score (1-5)     │       │ sales           │  │
│ description     │       │ date_of_publication│    │ number_of_upvotes│      │ created_at      │  │
│ created_at      │       │ number_of_sales │       │ reviewer_name   │       │ updated_at      │  │
│ updated_at      │       │ created_at      │       │ created_at      │       └─────────────────┘  │
└─────────────────┘       │ updated_at      │       │ updated_at      │                            │
                          └─────────────────┘       └─────────────────┘                            │
                                    │                                                                │
                                    └────────────────────────────────────────────────────────────────┘

RELATIONSHIPS:
==============
• One Author can have many Books (1:N)
• One Book can have many Reviews (1:N)  
• One Book can have many Sales records (1:N)
• Each Sales record is unique per book per year

CONSTRAINTS:
============
• Reviews.score: Must be between 1 and 5
• Sales.year: Must be between 1900 and 2100  
• Sales.sales: Must be >= 0
• Books.number_of_sales: Must be >= 0
• Reviews.number_of_upvotes: Must be >= 0
• Unique constraint on (book_id, year) in Sales table

INDEXES:
========
Authors: name, country_of_origin
Books: name, author_id, date_of_publication, number_of_sales
Reviews: book_id, score, number_of_upvotes, created_at
Sales: book_id, year, sales

FOREIGN KEY ACTIONS:
===================
• Books -> Authors: ON DELETE RESTRICT, ON UPDATE CASCADE
• Reviews -> Books: ON DELETE CASCADE, ON UPDATE CASCADE  
• Sales -> Books: ON DELETE CASCADE, ON UPDATE CASCADE

*/
