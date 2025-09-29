CREATE TYPE "notification_type" AS ENUM (
  'post',
  'like',
  'follow',
  'comment',
  'mention'
);

CREATE TABLE "follow" (
  "following_user_id" integer,
  "followed_user_id" integer,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "user" (
  "id" integer PRIMARY KEY,
  "username" varchar,
  "name" text,
  "bio" varchar,
  "profile_picture" text,
  "banner" text,
  "is_verified" boolean,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "post" (
  "id" integer PRIMARY KEY,
  "title" varchar,
  "body" text,
  "has_attachment" boolean,
  "attachment" text[],
  "user_id" integer NOT NULL,
  "status" varchar,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "comment" (
  "id" integer PRIMARY KEY,
  "body" text,
  "user_id" integer NOT NULL,
  "post_id" integer NOT NULL,
  "parent_id" integer,
  "has_attachment" boolean,
  "attachment" text[],
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "post_like" (
  "id" integer PRIMARY KEY,
  "post_id" integer,
  "user_id" integer,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "comment_like" (
  "id" integer PRIMARY KEY,
  "comment_id" integer,
  "user_id" integer,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "post_dislike" (
  "id" integer PRIMARY KEY,
  "post_id" integer,
  "user_id" integer,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "comment_dislike" (
  "id" integer PRIMARY KEY,
  "comment_id" integer,
  "user_id" integer,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "message" (
  "id" integer PRIMARY KEY,
  "sender_id" integer,
  "receiver_id" integer,
  "body" text,
  "has_attachment" boolean,
  "attachment" text[],
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "notification" (
  "id" integer PRIMARY KEY,
  "type_id" integer,
  "type" notification_type,
  "created_at" timestamp,
  "updated_at" timestamp
);

COMMENT ON COLUMN "post"."body" IS 'Content of the post';

ALTER TABLE "post" ADD CONSTRAINT "user_post" FOREIGN KEY ("user_id") REFERENCES "user" ("id");

ALTER TABLE "follow" ADD FOREIGN KEY ("following_user_id") REFERENCES "user" ("id");

ALTER TABLE "follow" ADD FOREIGN KEY ("followed_user_id") REFERENCES "user" ("id");

ALTER TABLE "comment" ADD FOREIGN KEY ("id") REFERENCES "post" ("id");

ALTER TABLE "comment" ADD FOREIGN KEY ("user_id") REFERENCES "user" ("id");

ALTER TABLE "post_like" ADD FOREIGN KEY ("post_id") REFERENCES "post" ("id");

ALTER TABLE "post_like" ADD FOREIGN KEY ("user_id") REFERENCES "user" ("id");

ALTER TABLE "comment_like" ADD FOREIGN KEY ("comment_id") REFERENCES "comment" ("id");

ALTER TABLE "comment_like" ADD FOREIGN KEY ("user_id") REFERENCES "user" ("id");

ALTER TABLE "post_dislike" ADD FOREIGN KEY ("post_id") REFERENCES "post" ("id");

ALTER TABLE "post_dislike" ADD FOREIGN KEY ("user_id") REFERENCES "user" ("id");

ALTER TABLE "comment_dislike" ADD FOREIGN KEY ("comment_id") REFERENCES "comment" ("id");

ALTER TABLE "comment_dislike" ADD FOREIGN KEY ("user_id") REFERENCES "user" ("id");

ALTER TABLE "message" ADD FOREIGN KEY ("sender_id") REFERENCES "user" ("id");

ALTER TABLE "message" ADD FOREIGN KEY ("receiver_id") REFERENCES "user" ("id");
