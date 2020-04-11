SET
  search_path TO RNOTES;
INSERT INTO users (email, name, password)
VALUES
  (
    'user_a@email.com',
    'user_a',
    '1464acd6765f91fccd3f5bf4f14ebb7ca69f53af91b0a5790c2bba9d8819417b'
  );
INSERT INTO users (email, name, password)
VALUES
  (
    'user_b@email.com',
    'user_b',
    '1464acd6765f91fccd3f5bf4f14ebb7ca69f53af91b0a5790c2bba9d8819417b'
  );
INSERT INTO users (email, name, password)
VALUES
  (
    'user_c@email.com',
    'user_c',
    '1464acd6765f91fccd3f5bf4f14ebb7ca69f53af91b0a5790c2bba9d8819417b'
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