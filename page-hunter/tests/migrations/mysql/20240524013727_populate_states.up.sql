-- Add up migration script here
CREATE PROCEDURE IF NOT EXISTS populate_states()
BEGIN
  DECLARE j INT DEFAULT 1;
  WHILE j <= 100 DO
    INSERT INTO test.states (country_name, name) VALUES (CONCAT('Country ', j), CONCAT('State ', j));
    SET j = j + 1;
  END WHILE;
END;

CALL populate_states();