SET
  search_path TO RNOTES;
INSERT INTO users (email, name, password)
VALUES
  (
    'admin@email.com',
    'Administrator',
    '1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B'
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