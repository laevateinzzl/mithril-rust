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