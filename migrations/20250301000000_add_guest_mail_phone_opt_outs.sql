-- Add StructuredValue list columns: mail, phone, opt_outs (JSON arrays)
ALTER TABLE guests ADD COLUMN mail TEXT NOT NULL DEFAULT '[]';
ALTER TABLE guests ADD COLUMN phone TEXT NOT NULL DEFAULT '[]';
ALTER TABLE guests ADD COLUMN opt_outs TEXT NOT NULL DEFAULT '[]';
