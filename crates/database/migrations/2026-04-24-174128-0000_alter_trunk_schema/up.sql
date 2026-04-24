PRAGMA foreign_keys=OFF;

CREATE TABLE new_trunk (
  id INTEGER NOT NULL PRIMARY KEY,
  card_id TEXT NOT NULL UNIQUE,
  amount INTEGER NOT NULL DEFAULT 1 CHECK (amount > 0),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (card_id) REFERENCES card (card_id) ON DELETE CASCADE ON UPDATE CASCADE
);

INSERT INTO new_trunk SELECT * FROM trunk;

DROP TABLE trunk;

ALTER TABLE new_trunk RENAME TO trunk;

PRAGMA foreign_keys=ON;
PRAGMA foreign_key_check;
