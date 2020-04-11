SET
  search_path TO RNOTES;
INSERT INTO users (email, name, password)
VALUES
  (
    'admin@email.com',
    'Administrator',
    '1464acd6765f91fccd3f5bf4f14ebb7ca69f53af91b0a5790c2bba9d8819417b'
  );

INSERT INTO categories (name)
VALUES
  ('Inspiration');
INSERT INTO categories (name)
VALUES
  ('Personal');
INSERT INTO categories (name)
VALUES
  ('Shopping');
INSERT INTO categories (name)
VALUES
  ('Tech');
INSERT INTO categories (name)
VALUES
  ('Work');