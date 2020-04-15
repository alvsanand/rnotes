SET
  search_path TO RNOTES;
INSERT INTO users (email, name, password)
VALUES
  (
    'user_a@email.com',
    'user_a',
    '1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B' --   some_password
  );
INSERT INTO users (email, name, password)
VALUES
  (
    'user_b@email.com',
    'user_b',
    '1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B'
  );
INSERT INTO users (email, name, password)
VALUES
  (
    'user_c@email.com',
    'user_c',
    '1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B'
  );
INSERT INTO categories (name)
VALUES
  ('cat_a');
INSERT INTO categories (name)
VALUES
  ('cat_b');
INSERT INTO notes (title, data, user_id)
VALUES
  ('note_a_user_a', 'some_text_note_a_user_a', 1);
INSERT INTO notes (title, data, user_id, category_id)
VALUES
  ('note_b_user_a', 'some_text_note_a_user_a', 1, 1);
INSERT INTO notes (title, data, user_id, category_id)
VALUES
  ('note_c_user_c', 'some_text_note_c_user_c', 1, 2);
INSERT INTO notes (title, data, user_id)
VALUES
  ('note_a_user_b', 'some_text_note_a_user_b', 2);