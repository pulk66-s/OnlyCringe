Feature: Checking Login Page Functionnalities

    Scenario: Login with valid credentials
        Given Opening the login page with GoogleChrome
        And Deleting all users
        And Creating users
            | name  | password  | email             |
            | user1 | pass1     | user1@gmail.com   |
        When I fill login field with user1
        And I fill password field with pass1
        And I click on the login button
        Then I should be on the Forum Home page

    Scenario: I should see the User legal Condition
        Given Opening the login page with GoogleChrome
        When I click on the User Condition link
        Then I should be on the User Condition page

    Scenario: I should go to create an account page
        Given Opening the login page with GoogleChrome
        When I click on the Create an account link
        Then I should be on the Create An Account page