Feature: I want to create accounts

    Scenario: I want to create basic account
        Given Opening the Create An Account page with GoogleChrome
        And Deleting all users
        Then I can create user with form
            | name      | password  | confirm password  | email             | confirm email     |
            | John Doe  | 123456    | 123456            | john@gmail.com    | john@gmail.com    |

    Scenario: I should be able to click on the Home page
        Given Opening the Create An Account page with GoogleChrome
        When I click on the create account Home page button
        Then I should be on the Login Page

    Scenario: I want to create account with invalid data
        Given Opening the Create An Account page with GoogleChrome
        And Deleting all users
        And Creating users
            | name          | password  | email                 |
            | John Doe      | 123456    | john@gmail.com        |
            | existing mail | 123456    | existing@gmail.com    |
        Then I can not create user with form
            | name      | password  | confirm password  | email                 | confirm email         |
            # User with already existing name
            | John Doe  | 111111    | 111111            | 1111@gmail.com        | 1111@gmail.com        |
            # User with existing email
            | testing1  | 111111    | 111111            | existing@gmail.com    | existing@gmail.com    |
            # Different fields
            | testing2  | 111111    | 111112            | goodEmail@gmail.com   | goodEmail@gmail.com   |
            | testing3  | 111111    | 111111            | badEmail@gmail.com    | badEmail2@gmail.com   |
            # Totaly valid user
            # | testing4  | 111111    | 111111            | testing4@gmail.com    | testing4@gmail.com    |