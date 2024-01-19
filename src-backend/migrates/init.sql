-- create table todos mysql
CREATE TABLE todos (
	id INT AUTO_INCREMENT,
	user_id INT NOT NULL DEFAULT 0,
	title VARCHAR(255) NOT NULL,
	description TEXT,
	status TINYINT NOT NULL DEFAULT 0,
	priority TINYINT NOT NULL DEFAULT 0,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	deleted_at TIMESTAMP NULL DEFAULT NULL,
	deadline TIMESTAMP NULL DEFAULT NULL,
	done boolean NOT NULL DEFAULT false,
	PRIMARY KEY (id)
);

-- create table todos postgres
CREATE TABLE "public"."todos" (
  "id" int4 NOT NULL DEFAULT nextval('mytable_id_seq'::regclass),
  "user_id" int4 NOT NULL DEFAULT 0,
  "title" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "description" text COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::text,
  "status" int2 NOT NULL DEFAULT 0,
  "priority" int2 NOT NULL DEFAULT 0,
  "updated_at" timestamptz(6),
  "created_at" timestamptz(6),
  "deleted_at" timestamptz(6),
  "deadline" timestamptz(6),
  "done" bool,
  CONSTRAINT "todos_pkey" PRIMARY KEY ("id")
);

-- create table todos mysql
CREATE TABLE user (
  user_id INT AUTO_INCREMENT,
  username VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  salt VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP NULL DEFAULT NULL,
  status TINYINT NOT NULL DEFAULT 0,
  PRIMARY KEY (user_id),
  UNIQUE KEY (email)
);

-- create table todos postgres
CREATE TABLE "public"."user" (
  "user_id" int4 NOT NULL DEFAULT nextval('mytable_id_seq'::regclass),
  "username" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "email" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "password" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "salt" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "created_at" timestamptz(6),
  "updated_at" timestamptz(6),
  "deleted_at" timestamptz(6),
  "status" int2 NOT NULL DEFAULT 0,
  CONSTRAINT "user_pkey" PRIMARY KEY ("user_id"),
  CONSTRAINT "user_email_key" UNIQUE ("email")
);
