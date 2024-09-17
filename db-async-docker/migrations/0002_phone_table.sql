
CREATE TABLE IF NOT EXISTS "phone" (
	"id" serial PRIMARY KEY NOT NULL,
	"person_id" integer NOT NULL,
	"phone_number" varchar(25) NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "phone" ADD CONSTRAINT "phone_person_id_person_id_fk" FOREIGN KEY ("person_id") REFERENCES "public"."person"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
