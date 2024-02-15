-- Your SQL goes here
CREATE TABLE transaction_tags (
  transaction_id INTEGER,
  tag_id INTEGER,
  
  PRIMARY KEY (transaction_id, tag_id),
  FOREIGN KEY (transaction_id) REFERENCES transactions(id),
  FOREIGN KEY (tag_id) REFERENCES tags(id)
)