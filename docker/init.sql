CREATE TABLE "users" (
  "id" integer PRIMARY KEY,
  "username" varchar,
  "password_hash" varchar,
  "role" integer,
  "created_at" timestamp
);

CREATE TABLE "roles" (
  "id" integer PRIMARY KEY,
  "name" varchar,
  "permissions" json,
  "created_at" timestamp
)

CREATE TABLE "folders" (
  "id" integer PRIMARY KEY,
  "user_id" integer,
  "name" varchar,
  "deck_id" integer,
  "created_at" timestamp
);

CREATE TABLE "collections" (
  "id" integer PRIMARY KEY,
  "user_id" integer,
  "name" varchar,
  "cards" integer,
  "price" decimal,
  "updated_at" timestamp,
  "created_at" timestamp
);

CREATE TABLE "whishlists" (
  "id" integer PRIMARY KEY,
  "user_id" integer,
  "cards" integer,
  "price" decimal,
  "updated_at" timestamp,
  "created_at" timestamp
);

CREATE TABLE "card_counts" (
  "id" integer PRIMARY KEY,
  "card" integer,
  "count" integer
);

CREATE TABLE "decks" (
  "id" integer PRIMARY KEY,
  "user_id" integer,
  "name" varchar,
  "colors" json,
  "tags" json,
  "main_deck" integer,
  "sideboard" integer,
  "considering" integer,
  "tokens" integer,
  "price" decimal,
  "updated_at" timestamp,
  "created_at" timestamp
);

CREATE TABLE "cards" (
  "id" integer PRIMARY KEY,
  "lang" varchar,
  "tcgplayer_id" integer,
  "tcgplayer_etched_id" integer,
  "layout" varchar,
  "oracle_id" integer,
  "prints_search_uri" varchar,
  "rulings_uri" varchar,
  "scryfall_uri" varchar,
  "uri" varchar,
  "all_parts" integer,
  "card_faces" intefer,
  "cmc" decimal,
  "color_indentity" json,
  "color_indicator" json,
  "colors" json,
  "defense" varchar,
  "edhrec_rank" integer,
  "game_changer" boolean,
  "hand_modifier" varchar,
  "keywords" json,
  "legalities" json,
  "life_modifier" varchar,
  "loyalty" varchar,
  "mana_cost" varchar,
  "name" varchar,
  "oracle_text" varchar,
  "penny_rank" integer,
  "power" varchar,
  "produced_mana" json,
  "reserved" boolean,
  "toughness" varchar,
  "type_line" varchar,
  "artist" varchar,
  "artist_ids" json,
  "booster" boolean,
  "border_color" varchar,
  "collector_number" varchar,
  "content_warning" boolean,
  "finishes" json,
  "flavor_name" varchar,
  "flavor_text" varchar,
  "frame_effects" json,
  "frame" varchar,
  "full_art" boolean,
  "highres_image" boolean,
  "illustration_id" integer,
  "image_status" varchar,
  "image_uris" json,
  "oversized" boolean,
  "prices" json,
  "printed_name" varchar,
  "printed_text" varchar,
  "printed_type_line" varchar,
  "promo" boolean,
  "promo_types" json,
  "purchase_uris" json,
  "rarity" varchar,
  "related_uris" json,
  "released_at" timestamp,
  "reprint" boolean,
  "scryfall_set_uri" varchar,
  "set_name" varchar,
  "set_search_uri" varchar,
  "set_type" varchar,
  "set_uri" varchar,
  "set" varchar,
  "set_id" varchar,
  "story_spotlight" boolean,
  "textless" boolean,
  "variation" boolean,
  "variation_of" integer,
  "security_stamp" varchar,
  "watermark" varchar,
  "created_at" timestamp
);

CREATE TABLE "related_cards" (
  "id" integer PRIMARY KEY,
  "component" varchar,
  "name" varchar,
  "type_line" varchar,
  "created_at" timestamp
);

CREATE TABLE "card_faces" (
  "id" integer PRIMARY KEY,
  "artist" varchar,
  "artist_id" integer,
  "cmc" decimal,
  "color_indicator" json,
  "colors" json,
  "defense" varchar,
  "flavor_text" varchar,
  "illustration_id" integer,
  "image_uris" json,
  "layout" varchar,
  "loyalty" varchar,
  "mana_cost" varchar,
  "name" varchar,
  "oracle_id" integer,
  "oracle_text" varchar,
  "power" varchar,
  "printed_name" varchar,
  "printed_text" varchar,
  "printed_type_line" varchar,
  "toughness" varchar,
  "type_line" varchar,
  "watermark" varchar,
  "created_at" timestamp
);

CREATE INDEX ON "cards" ("oracle_id");

ALTER TABLE "users" ADD FOREIGN KEY ("role") REFERENCES "roles" ("id");

ALTER TABLE "folders" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "decks" ADD FOREIGN KEY ("id") REFERENCES "folders" ("deck_id");

ALTER TABLE "collections" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

CREATE TABLE "card_counts_collections" (
  "card_counts_id" integer,
  "collections_cards" integer,
  PRIMARY KEY ("card_counts_id", "collections_cards")
);

ALTER TABLE "card_counts_collections" ADD FOREIGN KEY ("card_counts_id") REFERENCES "card_counts" ("id");

ALTER TABLE "card_counts_collections" ADD FOREIGN KEY ("collections_cards") REFERENCES "collections" ("cards");


ALTER TABLE "whishlists" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

CREATE TABLE "card_counts_whishlists" (
  "card_counts_id" integer,
  "whishlists_cards" integer,
  PRIMARY KEY ("card_counts_id", "whishlists_cards")
);

ALTER TABLE "card_counts_whishlists" ADD FOREIGN KEY ("card_counts_id") REFERENCES "card_counts" ("id");

ALTER TABLE "card_counts_whishlists" ADD FOREIGN KEY ("whishlists_cards") REFERENCES "whishlists" ("cards");


ALTER TABLE "cards" ADD FOREIGN KEY ("id") REFERENCES "card_counts" ("card");

CREATE TABLE "card_counts_decks_main" (
  "card_counts_id" integer,
  "decks_main_deck" integer,
  PRIMARY KEY ("card_counts_id", "decks_main_deck")
);

ALTER TABLE "card_counts_decks_main" ADD FOREIGN KEY ("card_counts_id") REFERENCES "card_counts" ("id");

ALTER TABLE "card_counts_decks_main" ADD FOREIGN KEY ("decks_main_deck") REFERENCES "decks" ("main_deck");


CREATE TABLE "card_counts_decks_sideboard" (
  "card_counts_id" integer,
  "decks_sideboard" integer,
  PRIMARY KEY ("card_counts_id", "decks_sideboard")
);

ALTER TABLE "card_counts_decks_sideboard" ADD FOREIGN KEY ("card_counts_id") REFERENCES "card_counts" ("id");

ALTER TABLE "card_counts_decks_sideboard" ADD FOREIGN KEY ("decks_sideboard") REFERENCES "decks" ("sideboard");


CREATE TABLE "card_counts_decks_considering" (
  "card_counts_id" integer,
  "decks_considering" integer,
  PRIMARY KEY ("card_counts_id", "decks_considering")
);

ALTER TABLE "card_counts_decks_considering" ADD FOREIGN KEY ("card_counts_id") REFERENCES "card_counts" ("id");

ALTER TABLE "card_counts_decks_considering" ADD FOREIGN KEY ("decks_considering") REFERENCES "decks" ("considering");


CREATE TABLE "cards_decks" (
  "cards_id" integer,
  "decks_tokens" integer,
  PRIMARY KEY ("cards_id", "decks_tokens")
);

ALTER TABLE "cards_decks" ADD FOREIGN KEY ("cards_id") REFERENCES "cards" ("id");

ALTER TABLE "cards_decks" ADD FOREIGN KEY ("decks_tokens") REFERENCES "decks" ("tokens");


CREATE TABLE "related_cards_cards" (
  "related_cards_id" integer,
  "cards_all_parts" integer,
  PRIMARY KEY ("related_cards_id", "cards_all_parts")
);

ALTER TABLE "related_cards_cards" ADD FOREIGN KEY ("related_cards_id") REFERENCES "related_cards" ("id");

ALTER TABLE "related_cards_cards" ADD FOREIGN KEY ("cards_all_parts") REFERENCES "cards" ("all_parts");


CREATE TABLE "card_faces_cards" (
  "card_faces_id" integer,
  "cards_card_faces" intefer,
  PRIMARY KEY ("card_faces_id", "cards_card_faces")
);

ALTER TABLE "card_faces_cards" ADD FOREIGN KEY ("card_faces_id") REFERENCES "card_faces" ("id");

ALTER TABLE "card_faces_cards" ADD FOREIGN KEY ("cards_card_faces") REFERENCES "cards" ("card_faces");

