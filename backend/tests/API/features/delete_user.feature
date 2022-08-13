Feature: User deletion

    Scenario: I should be able to delete user with name / uuid
        Given Cleaning all user database
        When Creating users
            | uuid                                  | name          | email                 | password  |
            | bec0691c-1b09-11ed-9209-e049ca1a256b  | John Doe      | john@gmail.com        | 123456    |
            |                                       | Jane Doe      | jane@gmail.com        | 123456    |
        Then I should being able to delete the users    
            | uuid                                  | name      |
            | bec0691c-1b09-11ed-9209-e049ca1a256b  |           |
            |                                       | Jane Doe  |
        And I should not being able to delete the users
            | uuid                                  | name      |
            | bec0691c-1b09-11ed-9209-e049ca1a256b  |           |
            |                                       | Jane Doe  |
        And The user should be archived
            | uuid                                  | name      |
            | bec0691c-1b09-11ed-9209-e049ca1a256b  |           |
            |                                       | Jane Doe  |
