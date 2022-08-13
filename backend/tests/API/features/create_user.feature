Feature: Creating User tests

    Scenario: Create simple users
        Given Cleaning all user database
        When Creating users
            | name          | email                 | password  | role  | description   |
            | John Doe      | john@gmail.com        | 123456    |       |               |
            | Jane Doe      | jane@gmail.com        | 123456    | ADMIN |               |
            | DescUser      | descuser@gmail.com    | 123456    |       | lorem ipsum   |
        Then The users should be created
            | name          | email                 | role  | description   |
            | John Doe      | john@gmail.com        |       |               |
            | Jane Doe      | jane@gmail.com        | ADMIN |               |
            | DescUser      | descuser@gmail.com    |       | lorem ipsum   |

    Scenario: Create users with invalid data
        Given Cleaning all user database
        When Creating users
            | name          | email             | password  | role  |
            | John Doe      | john@gmail.com    | 123456    | ADMIN |
        Then The users cannot be created
            | name          | email             | password  |
            | John Doe      | not@gmail.com     | 1234567   |
            | not existing  | john@gmail.com    | 1234567   |
            | empty email   |                   | 1234567   |

    Scenario: Checking password encryption when creating user
        Given Cleaning all user database
        When Creating users
            | name  | email             | password  |
            | user1 | user1@gmail.com   | 123456    |
        Then The user user1 cannot have the field password filled with 123456

