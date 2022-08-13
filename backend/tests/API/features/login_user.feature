Feature: User Login

    Scenario: I should login
        Given Cleaning all user database
        When Creating users
            | name          | email                 | password  |
            | John Doe      | john@gmail.com        | 123456    |
        Then I should be able to login with
            | name      | password  |
            | John Doe  | 123456    |