Feature: Get word score
    Scenario Outline: Given a word, derive it's word score
        When I compute the word score of '<word>'
        Then I get <expected_score>

    Examples:
       | word | expected_score |
       | SKY  | 55             |
