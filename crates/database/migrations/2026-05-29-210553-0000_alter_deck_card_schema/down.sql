PRAGMA foreign_keys=OFF;

DROP TABLE IF EXISTS deck_card;

CREATE TABLE deck_card (
  deck_id INTEGER NOT NULL,
  card_id INTEGER NOT NULL,
  amount_main INTEGER NOT NULL DEFAULT 0,
  amount_extra INTEGER NOT NULL DEFAULT 0,
  amount_side INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (deck_id, card_id),
  FOREIGN KEY (deck_id) REFERENCES deck (id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (card_id) REFERENCES card (id) ON DELETE CASCADE ON UPDATE CASCADE
);

PRAGMA foreign_keys=ON;
PRAGMA foreign_key_check;
PRAGMA integrity_check;
