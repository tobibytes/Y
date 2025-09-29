-- Recreate initial schema with OAuth-ready user table and fixed FKs

CREATE TYPE "notification_type" AS ENUM (
  'post',
  'like',
  'follow',
  'comment',
  'mention'
);

-- User table with OAuth fields
CREATE TABLE IF NOT EXISTS "user" (
  "id" SERIAL PRIMARY KEY,
  "email" TEXT UNIQUE,
  "provider" VARCHAR NOT NULL,
  "provider_user_id" TEXT NOT NULL,
  "access_token" TEXT,
  "refresh_token" TEXT,
  "token_expires_at" TIMESTAMP,
  "username" VARCHAR,
  "name" TEXT,
  "bio" VARCHAR,
  "profile_picture" TEXT,
  "banner" TEXT,
  "is_verified" BOOLEAN DEFAULT FALSE,
  "is_onboarded" BOOLEAN DEFAULT FALSE,
  "onboarding_complete_at" TIMESTAMP,
  "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
  "updated_at" TIMESTAMP NOT NULL DEFAULT NOW(),
  UNIQUE ("provider","provider_user_id")
);

CREATE TABLE IF NOT EXISTS "follow" (
  "following_user_id" integer,
  "followed_user_id" integer,
  "created_at" timestamp DEFAULT NOW(),
  "updated_at" timestamp DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "post" (
  "id" SERIAL PRIMARY KEY,
  "title" varchar,
  "body" text,
  "has_attachment" boolean,
  "attachment" text[],
  "user_id" integer NOT NULL,
  "status" varchar,
  "created_at" timestamp DEFAULT NOW(),
  "updated_at" timestamp DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "comment" (
  "id" SERIAL PRIMARY KEY,
  "body" text,
  "user_id" integer NOT NULL,
  "post_id" integer NOT NULL,
  "parent_id" integer,
  "has_attachment" boolean,
  "attachment" text[],
  "created_at" timestamp DEFAULT NOW(),
  "updated_at" timestamp DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "post_like" (
  "id" SERIAL PRIMARY KEY,
  "post_id" integer,
  "user_id" integer,
  "created_at" timestamp DEFAULT NOW(),
  "updated_at" timestamp DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "comment_like" (
  "id" SERIAL PRIMARY KEY,
  "comment_id" integer,
  "user_id" integer,
  "created_at" timestamp DEFAULT NOW(),
  "updated_at" timestamp DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "post_dislike" (
  "id" SERIAL PRIMARY KEY,
  "post_id" integer,
  "user_id" integer,
  "created_at" timestamp DEFAULT NOW(),
  "updated_at" timestamp DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "comment_dislike" (
  "id" SERIAL PRIMARY KEY,
  "comment_id" integer,
  "user_id" integer,
  "created_at" timestamp DEFAULT NOW(),
  "updated_at" timestamp DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "message" (
  "id" SERIAL PRIMARY KEY,
  "sender_id" integer,
  "receiver_id" integer,
  "body" text,
  "has_attachment" boolean,
  "attachment" text[],
  "created_at" timestamp DEFAULT NOW(),
  "updated_at" timestamp DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "notification" (
  "id" SERIAL PRIMARY KEY,
  "type_id" integer,
  "type" notification_type,
  "created_at" timestamp DEFAULT NOW(),
  "updated_at" timestamp DEFAULT NOW()
);

COMMENT ON COLUMN "post"."body" IS 'Content of the post';

-- Foreign keys
ALTER TABLE "post" ADD CONSTRAINT "user_post" FOREIGN KEY ("user_id") REFERENCES "user" ("id");

ALTER TABLE "follow" ADD FOREIGN KEY ("following_user_id") REFERENCES "user" ("id");
ALTER TABLE "follow" ADD FOREIGN KEY ("followed_user_id") REFERENCES "user" ("id");

-- Fix incorrect FK (use post_id -> post.id)
ALTER TABLE "comment" ADD FOREIGN KEY ("post_id") REFERENCES "post" ("id");
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
